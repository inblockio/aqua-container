#![allow(warnings)]

mod models;
mod util;

use axum::{body::Bytes, extract::{DefaultBodyLimit, Multipart, Path, Request, State}, handler::HandlerWithoutStateExt, http::StatusCode, response::{Html, Redirect}, routing::{get, post}, BoxError, Form, Router, Json};
use ethaddr::address;

use chrono::{NaiveDateTime, DateTime, Utc};

use futures::{Stream, TryStreamExt};
use sha3::*;
use std::collections::BTreeMap;
use std::fs;
use models::file::{FileInfo};
use std::io;
use std::net::SocketAddr;
use std::time::SystemTime;
use axum::response::{IntoResponse, Response};
use tokio::{fs::File, io::BufWriter};
use tokio_util::io::StreamReader;
use tower::ServiceExt;
use tower_http::{
    limit::RequestBodyLimitLayer,
    services::{ServeDir, ServeFile},
    trace::TraceLayer,
};
use tracing_subscriber::{layer::SubscriberExt, util::SubscriberInitExt};
use tower_http::cors::{Any, CorsLayer};
use bonsaidb::core::keyvalue::{KeyStatus, KeyValue};
use bonsaidb::core::schema::{Collection, SerializedCollection};
use bonsaidb::local::config::{Builder, StorageConfiguration};
use bonsaidb::local::Database;
use serde::{Deserialize, Serialize};
use sqlx::{sqlite::SqlitePoolOptions, Pool, Sqlite};
use guardian_common::{crypt, custom_types::*};
use serde_json::json;
use verifier::v1_1::hashes::*;
use crate::util::db_set_up;

const UPLOADS_DIRECTORY: &str = "uploads";

#[derive(Debug, Serialize, Deserialize, Collection, Clone)]
#[collection(name = "page")]
pub struct PageData {
    pub pages: Vec<HashChain>,
}

#[derive(Debug, Clone)]
struct Db {
    pub db: Database,
    pub sqliteDb : Pool<Sqlite>

}

#[tokio::main]
async fn main() {
    tracing_subscriber::fmt()
        .with_max_level(tracing::Level::DEBUG)
        .init();
    let sqliteDb =  db_set_up().await;
    let db = Database::open::<PageData>(StorageConfiguration::new("b0nsa1.bonsaidb")).unwrap();

    // save files to a separate directory to not override files in the current directory
    tokio::fs::create_dir(UPLOADS_DIRECTORY).await;

    let server_database = Db { db: db,  sqliteDb : sqliteDb };

    let app = Router::new()
        .route("/", get(show_form).post(save_request_body))
        .route("/json", post(save_json_file))
        .route("/verificationhash", post(get_verification_hash_for_file))
        .route("/signrevision", post(add_signature_hash_for_file))
        .route("/explorer_files", get(fetch_explorer_files))
        .route("/explorer_file_upload", post(explorer_file_upload))
        .route("/explorer_verify_hash", post(explorer_file_upload))
        .route("/explorer_sign_revision", post(explorer_file_upload))

        //.route("/list", get(show_files_list).post(show_files))
        .with_state(server_database)
        .layer(CorsLayer::permissive())
        .layer(TraceLayer::new_for_http());


    let listener = tokio::net::TcpListener::bind("127.0.0.1:3600")
        .await
        .unwrap();
    tracing::debug!("listening on {}", listener.local_addr().unwrap());
    axum::serve(listener, app).await.unwrap();
}

#[derive(Deserialize, Debug)]
#[allow(dead_code)]
struct Input {
    filename: String,
}

#[derive(Deserialize, Debug)]
#[allow(dead_code)]
struct SInput {
    filename: String,
    signature: String,
    publickey: String,
    wallet_address: String,
}


const MAX_FILE_SIZE: u32 = 20 * 1024 * 1024; // 20 MB in bytes

#[derive(Debug)]
enum UploadError {
    FileTooLarge(u32),
    MissingAccount,
    MissingFile,
    MultipartError(String),
}

impl IntoResponse for UploadError {
    fn into_response(self) -> Response {
        match self {
            UploadError::FileTooLarge(size) => (
                StatusCode::PRECONDITION_FAILED,
                format!("File size {} bytes exceeds maximum of {} bytes", size, MAX_FILE_SIZE)
            ).into_response(),
            UploadError::MissingAccount => (
                StatusCode::PRECONDITION_FAILED,
                "Account field is required".to_string()
            ).into_response(),
            UploadError::MissingFile => (
                StatusCode::PRECONDITION_FAILED,
                "File is required".to_string()
            ).into_response(),
            UploadError::MultipartError(msg) => (
                StatusCode::BAD_REQUEST,
                msg
            ).into_response(),
        }
    }
}

async fn fetch_explorer_files(State(server_database): State<Db>) -> (StatusCode, Json<Vec<FileInfo>>) {
    tracing::debug!("fetch_explorer_files");


    // Initialize an empty Vec to hold the result
    let mut pages: Vec<FileInfo> = Vec::new();

// Fetch all rows from the 'pages' table
    let rows = sqlx::query!("SELECT id, name, extension, page_data FROM pages")
        .fetch_all(&server_database.sqliteDb)
        .await;

    // Handle database result errors
    match rows {
        Ok(records) => {
            // Loop through the records and populate the pages vector
            for row in records {
                pages.push(FileInfo {
                    id: row.id,
                    name: row.name,
                    extension: row.extension,
                    page_data: row.page_data,
                });
            }

            if pages.is_empty() {
                return (StatusCode::NOT_FOUND, Json::from(pages));
            }

            // Return the populated list of pages
            (StatusCode::OK, Json(pages))
        }
        Err(e) => {
            tracing::error!("Failed to fetch records: {:?}", e);
            (StatusCode::INTERNAL_SERVER_ERROR, Json::from(pages))
        }
    }
}

async fn explorer_file_upload(
    State(server_database): State<Db>,
    mut multipart: Multipart,
) -> Result<Json<serde_json::Value>, UploadError> {
    tracing::debug!("explorer_file_upload fn");

    let mut account = None;
    let mut file_info = None;

    // Process only two fields: account and file
    for _ in 0..2 {
        let field = match multipart.next_field().await {
            Ok(Some(field)) => field,
            Ok(None) => break,
            Err(e) => return Err(UploadError::MultipartError(e.to_string())),
        };


        let name = field.name()
            .ok_or_else(|| UploadError::MultipartError("Field name missing".to_string()))?
            .to_string();

        println!("field {}", name);
        match name.as_str() {
            "account" => {
                account = Some(field.text().await
                    .map_err(|e| UploadError::MultipartError(e.to_string()))?);
            },
            "file" => {
                let file_name = field.file_name()
                    .ok_or_else(|| UploadError::MultipartError("File name missing".to_string()))?
                    .to_string();
                let content_type = field.content_type()
                    .ok_or_else(|| UploadError::MultipartError("Content type missing".to_string()))?
                    .to_string();
                let body_bytes = field.bytes().await
                    .map_err(|e| UploadError::MultipartError(e.to_string()))?
                    .to_vec();

                let file_size: u32 = body_bytes.len().try_into()
                    .map_err(|_| UploadError::FileTooLarge(u32::MAX))?;

                if file_size > MAX_FILE_SIZE {
                    return Err(UploadError::FileTooLarge(file_size));
                }

                file_info = Some((file_name, content_type, body_bytes, file_size));
            },
            _ => {
                tracing::warn!("Unexpected field: {}", name);
            }
        }
    }

    // Verify we have both account and file
    let account = account.ok_or(UploadError::MissingAccount)?;
    let (file_name, content_type, body_bytes, file_size) = file_info.ok_or(UploadError::MissingFile)?;

    tracing::debug!("Processing file upload - Account: {}, File: {}, Size: {} bytes", account, file_name, file_size);


    let b64 = Base64::from(body_bytes);
    let mut file_hasher = sha3::Sha3_512::default();
    file_hasher.update(b64.clone());
    let file_hash_current = Hash::from(file_hasher.finalize());

    let mut content_current = BTreeMap::new();

    content_current.insert("file_hash".to_owned(), file_hash_current.to_string());

    // println!("{:#?}", content_current);
    tracing::debug!("{:#?}", content_current);

    let content_hash_current = content_hash(&content_current.clone());

    // println!("{:#?}", content_hash_current);
    tracing::debug!("{:#?}", content_hash_current);

    let domain_id_current = "0".to_owned();
    let timestamp_current = Timestamp::from(chrono::NaiveDateTime::from_timestamp(
        Utc::now().timestamp(),
        0,
    ));

    let metadata_hash_current =
        metadata_hash(&domain_id_current, &timestamp_current.clone(), None);

    let verification_hash_current =
        verification_hash(&content_hash_current, &metadata_hash_current, None, None);

    let pagedata_current = &PageData {
        pages: vec![HashChain {
            genesis_hash: verification_hash_current.clone().to_string(),
            domain_id: domain_id_current,
            title: file_name.clone(),
            namespace: 0,
            chain_height: 0,
            revisions: vec![(
                verification_hash_current,
                Revision {
                    content: RevisionContent {
                        file: Some(FileContent {
                            data: b64,
                            filename: file_name.clone(),
                            size: file_size,
                            comment: "".to_string(),
                        }),
                        content: content_current,
                        content_hash: content_hash_current,
                    },
                    metadata: RevisionMetadata {
                        domain_id: "0".to_string(),
                        time_stamp: timestamp_current,
                        previous_verification_hash: None,
                        metadata_hash: metadata_hash_current,
                        verification_hash: verification_hash_current,
                    },
                    signature: None,
                    witness: None,
                },
            )],
        }],
    };

    // Convert struct to JSON string
    let json_string = serde_json::to_string(&pagedata_current).unwrap();

    let result = sqlx::query!(
        r#"
        INSERT INTO pages (name, extension, page_data)
        VALUES (?, ?, ?)
        "#,
        file_name,
        content_type,
        json_string,
    )
        .execute(&server_database.sqliteDb)
        .await
        .expect("Failed to insert page");

    Ok(Json(serde_json::json!({
        "status": "success",
        "account": account,
        "fileName": file_name,
        "fileSize": file_size,
        "contentType": content_type
    })))

}


async fn save_request_body(
    State(server_database): State<Db>,
    mut multipart: Multipart,
) -> Result<Redirect, (StatusCode, String)> {
    tracing::debug!("save_request_body fn");

    while let Some(field) = multipart.next_field().await.unwrap() {
        // println!("{:#?}", field);

        let name = field.name().unwrap().to_string();
        let file_name = field.file_name().unwrap().to_string();
        let content_type = field.content_type().unwrap().to_string();
        let body_bytes = field.bytes().await.unwrap().to_vec();

        // 4.b add rev.metadata.domain_id to hasher {m}

        let b64 = Base64::from(body_bytes);
        let mut file_hasher = sha3::Sha3_512::default();
        file_hasher.update(b64.clone());
        let file_hash_current = Hash::from(file_hasher.finalize());

        let mut content_current = BTreeMap::new();

        content_current.insert("file_hash".to_owned(), file_hash_current.to_string());

        // println!("{:#?}", content_current);
        tracing::debug!("{:#?}", content_current);

        let content_hash_current = content_hash(&content_current.clone());

        // println!("{:#?}", content_hash_current);
        tracing::debug!("{:#?}", content_hash_current);

        let domain_id_current = "0".to_owned();
        let timestamp_current = Timestamp::from(chrono::NaiveDateTime::from_timestamp(
            Utc::now().timestamp(),
            0,
        ));

        let metadata_hash_current =
            metadata_hash(&domain_id_current, &timestamp_current.clone(), None);

        let verification_hash_current =
            verification_hash(&content_hash_current, &metadata_hash_current, None, None);

        let pagedata_current = &PageData {
            pages: vec![HashChain {
                genesis_hash: verification_hash_current.clone().to_string(),
                domain_id: domain_id_current,
                title: file_name.clone(),
                namespace: 0,
                chain_height: 0,
                revisions: vec![(
                    verification_hash_current,
                    Revision {
                        content: RevisionContent {
                            file: Some(FileContent {
                                data: b64,
                                filename: file_name.clone(),
                                size: 0,
                                comment: "".to_string(),
                            }),
                            content: content_current,
                            content_hash: content_hash_current,
                        },
                        metadata: RevisionMetadata {
                            domain_id: "0".to_string(),
                            time_stamp: timestamp_current,
                            previous_verification_hash: None,
                            metadata_hash: metadata_hash_current,
                            verification_hash: verification_hash_current,
                        },
                        signature: None,
                        witness: None,
                    },
                )],
            }],
        };

        let document = &server_database
            .db
            .set_key(file_name.clone(), &pagedata_current)
            //            .only_if_vacant()
            .execute()
            .unwrap();

        let document2: &Option<PageData> = &server_database.db.get_key(file_name).into().unwrap();

        if document2.is_some() {
            let doc: PageData = document2.clone().unwrap();
            let (_, rev1) = &doc.pages[0].revisions[0];

            if rev1.content.file.is_some() {
                let file: FileContent = rev1.content.file.clone().unwrap();
                let vu8 = file.data.to_vec();
                // println!("{:#?}", String::from_utf8(vu8));
            }

            //            fs::remove_file("../aqua-verifier-js/vef.json").unwrap();
            //            fs::write(
            //                "../aqua-verifier-js/vef.json",
            //                serde_json::to_string(&doc).unwrap(),
            //            );
        }
    }

    Ok(Redirect::to("/"))
}

async fn add_signature_hash_for_file(
    State(server_database): State<Db>,
    Form(input): Form<SInput>,
) -> (StatusCode, String) {
    let mut document2: &Option<PageData> =
        &server_database.db.get_key(&input.filename).into().unwrap();

    if document2.is_some() {
        let mut doc: PageData = document2.clone().unwrap();
        let len = &doc.pages[0].revisions.len();

        let (ver1, rev1) = &doc.pages[0].revisions[len - 1].clone();

        let mut rev2 = rev1.clone();

        rev2.metadata.previous_verification_hash = Some(*ver1);

        let sig: Signature =
            <Signature as std::str::FromStr>::from_str(&input.signature[..]).unwrap();

        let pubk: PublicKey =
            <PublicKey as std::str::FromStr>::from_str(&input.publickey[..]).unwrap();

        let addr = ethaddr::Address::from_str_checksum(&input.wallet_address).unwrap();

        let sig_hash = signature_hash(&sig, &pubk);

        rev2.signature = Some(RevisionSignature {
            signature: sig,
            public_key: pubk,
            signature_hash: sig_hash.clone(),
            wallet_address: addr,
        });

        let timestamp_current = Timestamp::from(chrono::NaiveDateTime::from_timestamp(
            Utc::now().timestamp(),
            0,
        ));

        rev2.metadata.time_stamp = timestamp_current.clone();

        let metadata_hash_current = metadata_hash(
            &doc.pages[0].domain_id,
            &timestamp_current.clone(),
            Some(ver1),
        );

        let verification_hash_current = verification_hash(
            &rev2.content.content_hash,
            &metadata_hash_current,
            Some(&sig_hash),
            None,
        );

        rev2.metadata.metadata_hash = metadata_hash_current;

        &doc.pages[0]
            .revisions
            .push((verification_hash_current, rev2));

        let document = &server_database
            .db
            .set_key(&input.filename, &doc)
            //            .only_if_vacant()
            .execute()
            .unwrap();

        return (StatusCode::OK, ver1.to_string());
    }

    return (StatusCode::NOT_FOUND, "".to_string());
}

async fn get_verification_hash_for_file(
    State(server_database): State<Db>,
    Form(input): Form<Input>,
) -> (StatusCode, String) {
    let document2: &Option<PageData> = &server_database.db.get_key(&input.filename).into().unwrap();

    if document2.is_some() {
        let doc: PageData = document2.clone().unwrap();
        let len = &doc.pages[0].revisions.len();

        let (ver1, _) = &doc.pages[0].revisions[len - 1];
        return (StatusCode::OK, ver1.to_string());
    }
    return (StatusCode::NOT_FOUND, "".to_string());
}

async fn save_json_file(
    State(server_database): State<Db>,
    Form(input): Form<Input>,
) -> Result<Redirect, (StatusCode, String)> {
    // println!("{:#?}", &input);
    tracing::debug!("{:#?}", &input);

    let document2: &Option<PageData> = &server_database.db.get_key(&input.filename).into().unwrap();

    if document2.is_some() {
        let doc: PageData = document2.clone().unwrap();
        let (_, rev1) = &doc.pages[0].revisions[0];

        if rev1.content.file.is_some() {
            let file: FileContent = rev1.content.file.clone().unwrap();
            let vu8 = file.data.to_vec();
            // println!("{:#?}", String::from_utf8(vu8));
        }

        fs::write(
            "./export/".to_owned() + &input.filename + ".aqua.json",
            serde_json::to_string(&doc).unwrap(),
        );
    }
    Ok(Redirect::to("/"))
}


// Handler that returns HTML for a multipart form.
async fn show_form() -> impl IntoResponse {
    tracing::debug!("show_form");
    let html_content = fs::read_to_string("templates/form.html").unwrap_or_else(|_| {
        String::from("<h1>Error loading form page</h1>")
    });
    Html(html_content)
}

// to prevent directory traversal attacks we ensure the path consists of exactly one normal
// component
fn path_is_valid(path: &str) -> bool {
    let path = std::path::Path::new(path);
    let mut components = path.components().peekable();

    if let Some(first) = components.peek() {
        if !matches!(first, std::path::Component::Normal(_)) {
            return false;
        }
    }

    components.count() == 1
}

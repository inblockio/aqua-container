use axum::{
    body::Bytes,
    extract::{DefaultBodyLimit, Multipart, Path, Request, State},
    handler::HandlerWithoutStateExt,
    http::StatusCode,
    response::{Html, Redirect},
    routing::{get, post},
    BoxError, Form, Json, Router,
};
use ethaddr::address;

use chrono::{DateTime, NaiveDateTime, Utc};

use crate::models::file::FileInfo;
use crate::models::input::SInput;
use crate::models::page_data::PageDataContainer;
use crate::util::{
    check_if_page_data_revision_are_okay, check_or_generate_domain, compute_content_hash, db_set_up,
};
use crate::Db;
use axum::response::{IntoResponse, Response};
use bonsaidb::core::keyvalue::{KeyStatus, KeyValue};
use bonsaidb::core::schema::{Collection, SerializedCollection};
use bonsaidb::local::config::{Builder, StorageConfiguration};
use bonsaidb::local::Database;
use futures::{Stream, TryStreamExt};
use guardian_common::{crypt, custom_types::*};
use serde::{Deserialize, Serialize};
use serde_json::json;
use sha3::*;
use sqlx::{sqlite::SqlitePoolOptions, Pool, Sqlite};
use std::collections::BTreeMap;
use std::fs;
use std::io;
use std::net::SocketAddr;
use std::time::SystemTime;
use tokio::{fs::File, io::BufWriter};
use tokio_util::io::StreamReader;
use tower::ServiceExt;
use tracing_subscriber::{layer::SubscriberExt, util::SubscriberInitExt};
use verifier::v1_1::hashes::*;
const MAX_FILE_SIZE: u32 = 20 * 1024 * 1024; // 20 MB in bytes

#[derive(Debug)]
pub enum UploadError {
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
                format!(
                    "File size {} bytes exceeds maximum of {} bytes",
                    size, MAX_FILE_SIZE
                ),
            )
                .into_response(),
            UploadError::MissingAccount => (
                StatusCode::PRECONDITION_FAILED,
                "Account field is required".to_string(),
            )
                .into_response(),
            UploadError::MissingFile => (
                StatusCode::PRECONDITION_FAILED,
                "File is required".to_string(),
            )
                .into_response(),
            UploadError::MultipartError(msg) => (StatusCode::BAD_REQUEST, msg).into_response(),
        }
    }
}

pub async fn fetch_explorer_files(
    State(server_database): State<Db>,
) -> (StatusCode, Json<Vec<FileInfo>>) {
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

pub async fn explorer_file_verify_hash_upload(
    State(server_database): State<Db>,
    mut multipart: Multipart,
) -> (StatusCode, Json<Option<String>>) {
    tracing::debug!("explorer_file_verify_hash_upload fn");

    while let Ok(Some(field)) = multipart.next_field().await {
        let name = match field.name() {
            Some(name) => name.to_string(),
            None => {
                tracing::error!("Field name missing");
                return (
                    StatusCode::BAD_REQUEST,
                    Json(Some("Field missing".to_string())),
                );
            }
        };

        tracing::debug!("Processing field: {}", name);
        match name.as_str() {
            "file" => {
                let file_name = field.file_name().unwrap_or_default().to_string();

                if std::path::Path::new(&file_name)
                    .extension()
                    .and_then(|s| s.to_str())
                    != Some("json")
                {
                    tracing::error!("Uploaded file is not a JSON file");
                    return (
                        StatusCode::BAD_REQUEST,
                        Json(Some("File not JSON".to_string())),
                    );
                }

                // Read the field into a byte vector
                let data = match field.bytes().await {
                    Ok(data) => data,
                    Err(e) => {
                        tracing::error!("Failed to read file data: {:?}", e);
                        return (
                            StatusCode::BAD_REQUEST,
                            Json(Some("JSON broken".to_string())),
                        );
                    }
                };

                // Try to parse the file content into your struct
                match serde_json::from_slice::<PageDataContainer>(&data) {
                    Ok(parsed_data) => {
                        tracing::debug!("file is okay fn");

                        // verify t
                        let mut matches = true;
                        let mut failure_reason = "".to_string();
                        let parsed_data_chain = parsed_data.pages.get(0).unwrap();
                        // if the aqua json file has more than one revision compare the has
                        // current has with the previous  metadata > verification_hash
                        tracing::error!("Loop starts");
                        if (parsed_data_chain.revisions.len() > 1) {
                            
                            tracing::error!("revisions more than 1 result");
                            (matches, failure_reason) =  check_if_page_data_revision_are_okay(parsed_data_chain.revisions.clone());
                            tracing::error!("revisions are valied ? {}", matches);
                        
                        } else {
                            // let rev  = parsed_data_chain.revisions.get(0).unwrap();
                            // let hash =  compute_content_hash(rev);

                            let (verification_hash, revision) = parsed_data_chain
                                .revisions
                                .first()
                                .expect("No revisions found");

                            // Step 3: Recompute the content hash for the revision
                            let recomputed_content_hash = compute_content_hash(&revision.content);

                            match recomputed_content_hash {
                                Ok(data) => {
                                    // Step 4: Compare the recomputed content hash with the stored content hash

                                       
                                    let contnent_hash_str = format!("{:#?}", revision.content.content_hash);
                                    let data_str = format!("{:#?}", revision.content.content_hash);
                                        // data_str,
                                        // contnent_hash_str
                                    
                                    tracing::error!(" returd conetnet is   {} \n  my son content hash is {} \n", data_str, contnent_hash_str);
                                   if data ==revision.content.content_hash{
                                    matches =  true ;
                                   }else{
                    failure_reason=format!("a hash is not valid : {:#?}",  revision.content.content_hash)
                                   }
                                    //revision.content.content_hash;
                                }
                                Err(err) => {
                                    tracing::error!("Error compute_content_hash {} ", err);
                                    return (
                                        StatusCode::OK,
                                        Json(Some("AQUA Chain valid".to_string())),
                                    );
                                }
                            }
                        }
                        tracing::error!("Loop ends");
                        return if matches {
                            tracing::error!("Returning true");
                            (StatusCode::OK, Json(Some("AQUA Chain valid".to_string())))
                        } else {
                            tracing::error!("Returning false");
                            (
                                StatusCode::BAD_REQUEST,
                                Json(Some("AQUA Chin not valid".to_string())),
                            )
                        };
                    }
                    Err(e) => {
                        tracing::error!("Failed to parse JSON: {:?}", e);
                        return (
                            StatusCode::BAD_REQUEST,
                            Json(Some("Failed to parse JSON".to_string())),
                        );
                    }
                }
            }
            _ => continue,
        }
    }

    // Return an error if no file was found
    (StatusCode::BAD_REQUEST, Json(None))
}

pub async fn explorer_file_upload(
    State(server_database): State<Db>,
    mut multipart: Multipart,
) -> (StatusCode, Json<Option<FileInfo>>) {
    tracing::debug!("explorer_file_upload fn");

    let mut account = None;
    let mut file_info = None;

    // Process only two fields: account and file
    for _ in 0..2 {
        let field = match multipart.next_field().await {
            Ok(Some(field)) => field,
            Ok(None) => break,
            Err(e) => {
                tracing::error!("Multipart error: {}", e);
                return (StatusCode::BAD_REQUEST, Json(None));
            }
        };

        let name = match field.name() {
            Some(name) => name.to_string(),
            None => {
                tracing::error!("Field name missing");
                return (StatusCode::BAD_REQUEST, Json(None));
            }
        };

        tracing::debug!("Processing field: {}", name);
        match name.as_str() {
            "account" => {
                account = match field.text().await {
                    Ok(text) => Some(text),
                    Err(e) => {
                        tracing::error!("Failed to read account field: {}", e);
                        return (StatusCode::BAD_REQUEST, Json(None));
                    }
                };
            }
            "file" => {
                let file_name = match field.file_name() {
                    Some(name) => name.to_string(),
                    None => {
                        tracing::error!("File name missing");
                        return (StatusCode::BAD_REQUEST, Json(None));
                    }
                };
                let content_type = match field.content_type() {
                    Some(ct) => ct.to_string(),
                    None => {
                        tracing::error!("Content type missing");
                        return (StatusCode::BAD_REQUEST, Json(None));
                    }
                };
                let body_bytes = match field.bytes().await {
                    Ok(bytes) => bytes.to_vec(),
                    Err(e) => {
                        tracing::error!("Failed to read file bytes: {}", e);
                        return (StatusCode::BAD_REQUEST, Json(None));
                    }
                };

                let file_size: u32 = match body_bytes.len().try_into() {
                    Ok(size) => size,
                    Err(_) => {
                        tracing::error!("File size exceeds u32::MAX");
                        return (StatusCode::BAD_REQUEST, Json(None));
                    }
                };

                if file_size > MAX_FILE_SIZE {
                    tracing::error!("File size {} exceeds maximum allowed size", file_size);
                    return (StatusCode::BAD_REQUEST, Json(None));
                }

                file_info = Some((file_name, content_type, body_bytes, file_size));
            }
            _ => {
                tracing::warn!("Unexpected field: {}", name);
            }
        }
    }

    // Verify we have both account and file
    let account = match account {
        Some(acc) => acc,
        None => {
            tracing::error!("Account information missing");
            return (StatusCode::BAD_REQUEST, Json(None));
        }
    };
    let (file_name, content_type, body_bytes, file_size) = match file_info {
        Some(info) => info,
        None => {
            tracing::error!("File information missing");
            return (StatusCode::BAD_REQUEST, Json(None));
        }
    };

    tracing::debug!(
        "Processing file upload - Account: {}, File: {}, Size: {} bytes",
        account,
        file_name,
        file_size
    );

    let b64 = Base64::from(body_bytes);
    let mut file_hasher = sha3::Sha3_512::default();
    file_hasher.update(b64.clone());
    let file_hash_current = Hash::from(file_hasher.finalize());

    let mut content_current = BTreeMap::new();
    content_current.insert("file_hash".to_owned(), file_hash_current.to_string());

    tracing::debug!("Content current: {:#?}", content_current);

    let content_hash_current = content_hash(&content_current);
    tracing::debug!("Content hash current: {:#?}", content_hash_current);

    let domain_id_current = "0".to_owned();
    let timestamp_current = Timestamp::from(chrono::Utc::now().naive_utc());

    let metadata_hash_current = metadata_hash(&domain_id_current, &timestamp_current, None);
    let verification_hash_current =
        verification_hash(&content_hash_current, &metadata_hash_current, None, None);

    let api_domain = std::env::var("API_DOMAIN").unwrap_or_else(|_| "0".to_string());

    let pagedata_current = crate::models::page_data::PageDataContainer {
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
                            comment: String::new(),
                        }),
                        content: content_current,
                        content_hash: content_hash_current,
                    },
                    metadata: RevisionMetadata {
                        domain_id: api_domain,
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
    let json_string = match serde_json::to_string(&pagedata_current) {
        Ok(json) => json,
        Err(e) => {
            tracing::error!("Failed to serialize page data: {}", e);
            return (StatusCode::INTERNAL_SERVER_ERROR, Json(None));
        }
    };

    // Insert into database
    match sqlx::query!(
        r#"
        INSERT INTO pages (name, extension, page_data)
        VALUES (?, ?, ?)
        RETURNING id
        "#,
        file_name,
        content_type,
        json_string,
    )
    .fetch_one(&server_database.sqliteDb)
    .await
    {
        Ok(record) => {
            let file_info = FileInfo {
                id: record.id,
                name: file_name,
                extension: content_type,
                page_data: json_string,
            };
            (StatusCode::CREATED, Json(Some(file_info)))
        }
        Err(e) => {
            tracing::error!("Failed to insert page: {}", e);
            (StatusCode::INTERNAL_SERVER_ERROR, Json(None))
        }
    }
}

pub async fn explorer_sign_revision(
    State(server_database): State<Db>,
    Form(input): Form<SInput>,
) -> (StatusCode, Json<Option<FileInfo>>) {
    tracing::debug!("explorer_sign_revision");

    // Get the name parameter from the input
    if input.filename.is_empty() {
        return (StatusCode::BAD_REQUEST, Json(None));
    };

    // Fetch a single row from the 'pages' table where name matches
    let row = sqlx::query!(
        "SELECT id, name, extension, page_data FROM pages WHERE name = ? LIMIT 1",
        input.filename
    )
    .fetch_optional(&server_database.sqliteDb)
    .await;

    // Handle database result errors
    match row {
        Ok(Some(row)) => {
            // Deserialize page data
            let deserialized: PageDataContainer = match serde_json::from_str(&row.page_data) {
                Ok(data) => data,
                Err(e) => {
                    tracing::error!("Failed to parse page data record: {:?}", e);
                    return (StatusCode::INTERNAL_SERVER_ERROR, Json(None));
                }
            };

            let mut doc = deserialized;
            let len = doc.pages[0].revisions.len();

            let (ver1, rev1) = &doc.pages[0].revisions[len - 1].clone();

            let mut rev2 = rev1.clone();
            rev2.metadata.previous_verification_hash = Some(*ver1);

            // Parse input data with proper error handling
            let sig = match input.signature.parse::<Signature>() {
                Ok(s) => s,
                Err(e) => {
                    tracing::error!("Failed to parse signature: {:?}", e);
                    return (StatusCode::BAD_REQUEST, Json(None));
                }
            };
            let pubk = match input.publickey.parse::<PublicKey>() {
                Ok(p) => p,
                Err(e) => {
                    tracing::error!("Failed to parse public key: {:?}", e);
                    return (StatusCode::BAD_REQUEST, Json(None));
                }
            };
            let addr = match ethaddr::Address::from_str_checksum(&input.wallet_address) {
                Ok(a) => a,
                Err(e) => {
                    tracing::error!("Failed to parse wallet address: {:?}", e);
                    return (StatusCode::BAD_REQUEST, Json(None));
                }
            };

            let sig_hash = signature_hash(&sig, &pubk);

            rev2.signature = Some(RevisionSignature {
                signature: sig,
                public_key: pubk,
                signature_hash: sig_hash.clone(),
                wallet_address: addr,
            });

            let timestamp_current = Timestamp::from(chrono::Utc::now().naive_utc());
            rev2.metadata.time_stamp = timestamp_current.clone();

            let metadata_hash_current =
                metadata_hash(&doc.pages[0].domain_id, &timestamp_current, Some(ver1));

            let verification_hash_current = verification_hash(
                &rev2.content.content_hash,
                &metadata_hash_current,
                Some(&sig_hash),
                None,
            );

            rev2.metadata.metadata_hash = metadata_hash_current;
            doc.pages[0]
                .revisions
                .push((verification_hash_current, rev2));

            // Serialize the updated document
            let page_data_new = match serde_json::to_string(&doc) {
                Ok(data) => data,
                Err(e) => {
                    tracing::error!("Failed to serialize updated page data: {:?}", e);
                    return (StatusCode::INTERNAL_SERVER_ERROR, Json(None));
                }
            };

            // Update the database with the new document
            let result = sqlx::query!(
                "UPDATE pages SET page_data = ? WHERE id = ?",
                page_data_new,
                row.id
            )
            .execute(&server_database.sqliteDb)
            .await;

            // Handle the result of the update
            match result {
                Ok(_) => {
                    let file_info = FileInfo {
                        id: row.id,
                        name: row.name,
                        extension: row.extension,
                        page_data: page_data_new,
                    };
                    (StatusCode::OK, Json(Some(file_info)))
                }
                Err(e) => {
                    tracing::error!("Failed to update page data: {:?}", e);
                    (StatusCode::INTERNAL_SERVER_ERROR, Json(None))
                }
            }
        }
        Ok(None) => (StatusCode::NOT_FOUND, Json(None)),
        Err(e) => {
            tracing::error!("Failed to fetch record: {:?}", e);
            (StatusCode::INTERNAL_SERVER_ERROR, Json(None))
        }
    }
}

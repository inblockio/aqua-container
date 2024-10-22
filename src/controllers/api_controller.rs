use axum::{
    body::Bytes,
    extract::{DefaultBodyLimit, Multipart, Path, Request, State},
    handler::HandlerWithoutStateExt,
    http::{HeaderMap, StatusCode},
    response::{Html, Redirect},
    routing::{get, post},
    BoxError, Form, Json, Router,
};
use ethaddr::address;
use chrono::{DateTime, NaiveDateTime, Utc};
use ethers::core::k256::sha2::Sha256;
use crate::models::input::{RevisionInput, WitnessInput, DeleteInput, UpdateConfigurationInput};
use crate::models::page_data::{PageDataContainer, ApiResponse};
use crate::models::{file::FileInfo, page_data};
use crate::util::{
    check_if_page_data_revision_are_okay, 
    check_or_generate_domain,
     compute_content_hash, 
     db_set_up,
     make_empty_hash,
     update_env_file
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
use sha3::{Digest, Sha3_512};
use sqlx::{sqlite::SqlitePoolOptions, Pool, Sqlite};
use std::collections::BTreeMap;
use std::fs;
use std::io;
use std::net::SocketAddr;
use std::time::SystemTime;
use tokio::{fs::File, io::BufWriter};
use tokio_util::io::StreamReader;
use tower::ServiceExt;
use tracing_subscriber::{fmt::format, layer::SubscriberExt, util::SubscriberInitExt};
use verifier::v1_1::hashes::*;
use std::collections::HashMap;
use std::env;
use sqlx::Row;

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
    headers: HeaderMap, 
) -> (StatusCode, Json<ApiResponse>) {
    tracing::debug!("fetch_explorer_files");
    let mut log_data : Vec<String> = Vec::new();
    let mut res : ApiResponse = ApiResponse {
        logs: log_data,
        file: None,
        files: Vec::new()
    };

    // Initialize an empty Vec to hold the result
    let mut pages: Vec<FileInfo> = Vec::new();


    // Extract the 'public_key' header
    let query_string : String = match headers.get("public_key") {
        Some(value) => match value.to_str() {
            Ok(key) => {
               format!("SELECT id, name, extension, page_data , owner , mode FROM pages where mode ='pulic' or  owner = '{}'",key) 
            },
            Err(err) => {

                // return (StatusCode::BAD_REQUEST, Json(json!({"error": "Invalid public_key header"})))

                tracing::debug!("error {} ", err);
                "SELECT id, name, extension, page_data , owner , mode FROM pages where mode ='public'".to_string()
            },
        },
        None => {
            // return (StatusCode::BAD_REQUEST, Json(json!({"error": "public_key header missing"})))

            tracing::debug!("public_key header missing ");
            "SELECT id, name, extension, page_data , owner , mode  FROM pages where mode ='public'".to_string()
        },
    };


    tracing::debug!("Sql query ==>  {}",query_string);

    // Fetch all rows from the 'pages' table
    let rows = sqlx::query(&query_string)
        .fetch_all(&server_database.sqliteDb)
        .await;

    // Handle database result errors
    match rows {
        Ok(records) => {
            // Loop through the records and populate the pages vector
            // id: row.id,
                    // name: row.name,
                    // extension: row.extension,
                    // page_data: row.page_data,
            for row in records {
                pages.push(FileInfo {
                    id: row.get("id"),           // Get id from the row
                    name: row.get("name"),       // Get name from the row
                    extension: row.get("extension"), // Get extension from the row
                    page_data: row.get("page_data"), // Get page_data from the row
                    mode: row.get("mode"), // Get owner from the row
                    owner: row.get("owner"), // Get owner from the row
                    
                });
            }

            if pages.is_empty() {
                res.logs.push("No pages found".to_string());
                return (StatusCode::NOT_FOUND, Json::from(res));
            }

            res.files = pages;
            // Return the populated list of pages
            (StatusCode::OK, Json::from(res))
        }
        Err(e) => {
            tracing::error!("Failed to fetch records: {:?}", e);
            res.logs.push(format!("Error: Failed to fetch records: {:?}", e));
            res.files = pages;
            (StatusCode::INTERNAL_SERVER_ERROR, Json::from(res))
        }
    }
}

pub async fn explorer_file_verify_hash_upload(
    State(server_database): State<Db>,
    mut multipart: Multipart,
) -> (StatusCode, Json<ApiResponse>) {
    tracing::debug!("explorer_file_verify_hash_upload fn");
    let mut log_data: Vec<String> = Vec::new();
    let mut res : ApiResponse = ApiResponse {
        logs: log_data,
        file: None,
        files: Vec::new()
    };

    while let Ok(Some(field)) = multipart.next_field().await {
        let name = match field.name() {
            Some(name) => name.to_string(),
            None => {
                tracing::error!("Field name missing");
                res.logs.push("Field name missing".to_string());
                return (
                    StatusCode::BAD_REQUEST,
                    Json(res),
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
                    res.logs.push("Error: Uploaded file is not a JSON file".to_string());
                    return (
                        StatusCode::BAD_REQUEST,
                        Json(res),
                    );
                }

                // Read the field into a byte vector
                let data = match field.bytes().await {
                    Ok(data) => data,
                    Err(e) => {
                        tracing::error!("Failed to read file data: {:?}", e);
                        res.logs.push(format!("Error: Failed to read file data: {:?}", e));
                        return (
                            StatusCode::BAD_REQUEST,
                            Json(res),
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
                            (matches, failure_reason) = check_if_page_data_revision_are_okay(
                                parsed_data_chain.revisions.clone(),
                            );
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

                                    let contnent_hash_str =
                                        format!("{:#?}", revision.content.content_hash);
                                    let data_str = format!("{:#?}", revision.content.content_hash);
                                    // data_str,
                                    // contnent_hash_str

                                    tracing::error!(
                                        " returd conetnet is   {} \n  my son content hash is {} \n",
                                        data_str,
                                        contnent_hash_str
                                    );
                                    if data == revision.content.content_hash {
                                        matches = true;
                                    } else {
                                        failure_reason = format!(
                                            "a hash is not valid : {:#?}",
                                            revision.content.content_hash
                                        )
                                    }
                                    //revision.content.content_hash;
                                }
                                Err(err) => {
                                    tracing::error!("Error compute_content_hash {} ", err);
                                    res.logs.push("AQUA Chain valid".to_string());
                                    return (
                                        StatusCode::OK,
                                        Json(res),
                                    );
                                }
                            }
                        }
                        tracing::error!("Loop ends");
                        return if matches {
                            tracing::error!("Returning true");
                            res.logs.push("AQUA Chain valid".to_string());
                            (StatusCode::OK, Json(res))
                        } else {
                            tracing::error!("Returning false");
                            res.logs.push(failure_reason);
                            (StatusCode::BAD_REQUEST, Json(res))
                        };
                    }
                    Err(e) => {
                        tracing::error!("Failed to parse JSON: {:?}", e);
                        res.logs.push(format!("Failed to parse JSON: {:?}", e));
                        return (
                            StatusCode::BAD_REQUEST,
                            Json(res),
                        );
                    }
                }
            }
            _ => continue,
        }
    }

    // Return an error if no file was found
    (StatusCode::BAD_REQUEST, Json(res))
}

pub async fn explorer_file_upload(
    State(server_database): State<Db>,
    headers: HeaderMap,
    mut multipart: Multipart,
   
) -> (StatusCode, Json<ApiResponse>) {
    tracing::debug!("explorer_file_upload fn");

    let mut log_data: Vec<String> = Vec::new();
    let mut res : ApiResponse = ApiResponse {
        logs: log_data,
        file: None,
        files: Vec::new()
    };


    // Extract the 'public_key' header
    let public_key  = match headers.get("public_key") {
        Some(value) => match value.to_str() {
            Ok(key) => {
               key  
            },
            Err(err) => {

                tracing::error!("headers get error {} ", err);
                // return (StatusCode::BAD_REQUEST,  Json(json!({"error": "Invalid public_key header"})))
            
                res.logs.push(format!("Error: Meta mask public key  error: {:?}", err));
                return (StatusCode::BAD_REQUEST, Json(res));
            },
        },
        None => {

            tracing::debug!("public_key header missing ");
            // return (StatusCode::BAD_REQUEST, Json(json!({"error": "public_key header missing"})))
            res.logs.push("Error: Meta mask public key  missing".to_string());
            return (StatusCode::BAD_REQUEST, Json(res));

        },
    };


    let mut account = None;
    let mut file_info = None;

    // Process only two fields: account and file
    for _ in 0..2 {
        let field = match multipart.next_field().await {
            Ok(Some(field)) => field,
            Ok(None) => break,
            Err(e) => {
                tracing::error!("Multipart error: {}", e);
                res.logs.push(format!("Multipart error: {}", e));
                return (StatusCode::BAD_REQUEST, Json(res));
            }
        };

        let name = match field.name() {
            Some(name) => name.to_string(),
            None => {
                tracing::error!("Field name missing");
                res.logs.push("Field name missing".to_string());
                return (StatusCode::BAD_REQUEST, Json(res));
            }
        };

        tracing::debug!("Processing field: {}", name);
        match name.as_str() {
            "account" => {
                account = match field.text().await {
                    Ok(text) => Some(text),
                    Err(e) => {
                        tracing::error!("Failed to read account field: {}", e);
                        res.logs.push(format!("Failed to read account field: {}", e));
                        return (StatusCode::BAD_REQUEST, Json(res));
                    }
                };
            }
            "file" => {
                let file_name = match field.file_name() {
                    Some(name) => name.to_string(),
                    None => {
                        tracing::error!("File name missing");
                        res.logs.push("File name missing".to_string());
                        return (StatusCode::BAD_REQUEST, Json(res));
                    }
                };
                let content_type = match field.content_type() {
                    Some(ct) => ct.to_string(),
                    None => {
                        tracing::error!("Content type missing");
                        res.logs.push("Content type missing".to_string());
                        return (StatusCode::BAD_REQUEST, Json(res));
                    }
                };
                let body_bytes = match field.bytes().await {
                    Ok(bytes) => bytes.to_vec(),
                    Err(e) => {
                        tracing::error!("Failed to read file bytes: {}", e);
                        res.logs.push(format!("Failed to read file bytes: {}", e));
                        return (StatusCode::BAD_REQUEST, Json(res));
                    }
                };

                let file_size: u32 = match body_bytes.len().try_into() {
                    Ok(size) => size,
                    Err(_) => {
                        tracing::error!("File size exceeds u32::MAX");
                        res.logs.push("File size exceeds u32::MAX".to_string());
                        return (StatusCode::BAD_REQUEST, Json(res));
                    }
                };

                if file_size > MAX_FILE_SIZE {
                    tracing::error!("File size {} exceeds maximum allowed size", file_size);
                    res.logs.push(format!("File size {} exceeds maximum allowed size", file_size));
                    return (StatusCode::BAD_REQUEST, Json(res));
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
            res.logs.push("Account information missing".to_string());
            return (StatusCode::BAD_REQUEST, Json(res));
        }
    };
    let (file_name, content_type, body_bytes, file_size) = match file_info {
        Some(info) => info,
        None => {
            tracing::error!("File information missing");
            res.logs.push("File information missing".to_string());
            return (StatusCode::BAD_REQUEST, Json(res));
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

    let api_domain = std::env::var("API_DOMAIN").unwrap_or_else(|_| "0".to_string());
    let domain_id_current = api_domain.clone();
    let timestamp_current = Timestamp::from(chrono::Utc::now().naive_utc());

    tracing::debug!(
        "Domain ID: {}, Current timestamp: {:#?}",
        domain_id_current,
        timestamp_current
    );

    let metadata_hash_current = metadata_hash(&domain_id_current, &timestamp_current, None);
    tracing::debug!("HASH: {}", metadata_hash_current);
    let verification_hash_current =
        verification_hash(&content_hash_current, &metadata_hash_current, None, None);

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
            res.logs.push(format!("Failed to serialize page data: {}", e));
            return (StatusCode::INTERNAL_SERVER_ERROR, Json(res));
        }
    };

    let mut mode = "private".to_string(); 
    let file_mode = env::var("FILE_MODE").unwrap_or_default();

    if !file_mode.is_empty() {
    mode = file_mode;
    }
    // Insert into database
    match sqlx::query!(
        r#"
        INSERT INTO pages (name, extension, page_data, mode, owner)
        VALUES (?, ?, ?, ? , ?)
        RETURNING id
        "#,
        file_name,
        content_type,
        json_string,
        mode,
        public_key
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
               mode: mode,
               owner:  public_key.to_string()
            };
            res.file = Some(file_info);
            (StatusCode::CREATED, Json(res))
        }
        Err(e) => {
            tracing::error!("Failed to insert page: {}", e);
            res.logs.push(format!("Failed to insert page: {}", e));
            (StatusCode::INTERNAL_SERVER_ERROR, Json(res))
        }
    }
}


pub async fn explorer_sign_revision(
    State(server_database): State<Db>,
    Form(input): Form<RevisionInput>,
) -> (StatusCode, Json<ApiResponse>) {
    let mut log_data : Vec<String> = Vec::new();

    tracing::debug!("explorer_sign_revision");

    // Get the name parameter from the input
    if input.filename.is_empty() {
        log_data.push("Error : file name is empty".to_string());
        let res : ApiResponse = ApiResponse {
            logs: log_data,
            file: None,
            files: Vec::new()
        };
        return (StatusCode::BAD_REQUEST, Json(res));
    };

    // Fetch a single row from the 'pages' table where name matches
    let row = sqlx::query!(
        "SELECT id, name, extension, page_data, owner, mode FROM pages WHERE name = ? LIMIT 1",
        input.filename
    )
    .fetch_optional(&server_database.sqliteDb)
    .await;

    // Handle database result errors
    match row {
        Ok(Some(row)) => {
            // Deserialize page data
            let deserialized: PageDataContainer = match serde_json::from_str(&row.page_data) {
                Ok(data) => {
                    log_data.push("Success  : parse page data".to_string());
                    data
                },
                Err(e) => {
                    tracing::error!("Failed to parse page data record: {:?}", e);
                    log_data.push(format!("error : Failed to parse page data record: {:?}", e));
                    let res : ApiResponse = ApiResponse {
                        logs: log_data,
                        file: None,
                        files: Vec::new()
                    };
                    return (StatusCode::INTERNAL_SERVER_ERROR, Json(res));
                }
            };

            let mut doc = deserialized;
            let len = doc.pages[0].revisions.len();

            let (ver1, rev1) = &doc.pages[0].revisions[len - 1].clone();

            let mut rev2 = rev1.clone();
            rev2.witness = None;
            rev2.metadata.previous_verification_hash = Some(*ver1);

            // Parse input data with proper error handling
            let sig = match input.signature.parse::<Signature>() {
                Ok(s) => {
                
                    log_data.push("Success :  signature  parse successfully".to_string());
                    s
                },
                Err(e) => {
                    tracing::error!("Failed to parse signature: {:?}", e);

                    log_data.push(format!("error : Failed to parse  signature: {:?}", e));
                    let res : ApiResponse = ApiResponse {
                        logs: log_data,
                        file: None,
                        files: Vec::new()
                    };

                    return (StatusCode::BAD_REQUEST, Json(res));
                }
            };
            let pubk = match input.publickey.parse::<PublicKey>() {
                Ok(p) => {
                    log_data.push("Success : public  key  parsed successfully".to_string());
                
                    p
                },
                Err(e) => {
                    tracing::error!("Failed to parse public key: {:?}", e);

                    log_data.push(format!("error : Failed to parse  public key: {:?}", e));
                    let res : ApiResponse = ApiResponse {
                        logs: log_data,
                        file: None,
                        files: Vec::new()
                    };

                    return (StatusCode::BAD_REQUEST, Json(res));
                }
            };
            let addr = match ethaddr::Address::from_str_checksum(&input.wallet_address) {
                Ok(a) => {
                    log_data.push("wallet address parsed successfully".to_string());
                    
                    a
                },
                Err(e) => {
                    tracing::error!("Failed to parse wallet address: {:?}", e);
                    log_data.push(format!("Failed to parse wallet address: {:?}", e));
                    
                    let res : ApiResponse = ApiResponse {
                        logs: log_data,
                        file: None,
                        files: Vec::new()
                    };

                    return (StatusCode::BAD_REQUEST, Json(res));
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
            rev2.metadata.verification_hash = verification_hash_current;

            doc.pages[0]
                .revisions
                .push((verification_hash_current, rev2));

            // Serialize the updated document
            let page_data_new = match serde_json::to_string(&doc) {
                Ok(data) => {
                    log_data.push("revision  serialized  successfully".to_string());
                    
                    data
                },
                Err(e) => {
                    tracing::error!("Failed to serialize updated page data: {:?}", e);

                    log_data.push(format!("Failed to serialize updated page data : {:?}", e));

                    let res : ApiResponse = ApiResponse {
                        logs: log_data,
                        file: None,
                        files: Vec::new()
                    };

                    return (StatusCode::INTERNAL_SERVER_ERROR, Json(res));
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
                        owner : row.owner,
                        mode : row.mode
                    };
                    let res : ApiResponse = ApiResponse {
                        logs: log_data,
                        file: Some(file_info),
                        files: Vec::new()
                    };
                    (StatusCode::OK, Json(res))
                }
                Err(e) => {
                    tracing::error!("Failed to update page data: {:?}", e);
                    log_data.push(format!("Failed to update page data : {:?}", e));

                    let res : ApiResponse = ApiResponse {
                        logs: log_data,
                        file: None,
                        files: Vec::new()
                    };
                    (StatusCode::INTERNAL_SERVER_ERROR, Json(res))
                }
            }
        }
        Ok(None) => {

            tracing::error!("Failed not found ", );

            log_data.push("Failed data not found in database".to_string());

            let res : ApiResponse = ApiResponse {
                logs: log_data,
                file: None,
                files: Vec::new()
            };
            (StatusCode::NOT_FOUND, Json(res))
        },
        Err(e) => {
            tracing::error!("Failed to fetch record: {:?}", e);
            
            log_data.push(format!("Failed to fetch record : {:?}", e));


            let res : ApiResponse = ApiResponse {
                logs: log_data,
                file: None,
                files: Vec::new()
            };

            (StatusCode::INTERNAL_SERVER_ERROR, Json(res))
        }
    }
}


pub async fn explorer_delete_all_files(
    State(server_database): State<Db>,
) -> (StatusCode, Json<ApiResponse>) {

    let mut log_data : Vec<String> = Vec::new();

    let result = sqlx::query!("DELETE FROM pages ", )
        .execute(& server_database.sqliteDb)
        .await;

        match result {
            Ok(result_data) => {
                   // Check the number of affected rows
                if result_data.rows_affected() > 0 {
                    tracing::error!("Successfully deleted all the row with name");
                    log_data.push("Error : files data is deleted ".to_string());
                    let res : ApiResponse = ApiResponse {
                        logs: log_data,
                        file: None,
                        files: Vec::new()
                    };
                    return (StatusCode::OK, Json(res));
                } else {
                    tracing::error!("Error : No row deleted");

                    log_data.push("Error : No data deleted".to_string());
                    let res : ApiResponse = ApiResponse {
                        logs: log_data,
                        file: None,
                        files: Vec::new()
                    };
                    return (StatusCode::OK, Json(res));
                }
            }
            Err(e) => {
                tracing::error!("Failed to delete page data: {:?}", e);
                log_data.push(format!("Error occurred {}", e));
                    let res : ApiResponse = ApiResponse {
                        logs: log_data,
                        file: None,
                        files: Vec::new()
                    };
                (StatusCode::INTERNAL_SERVER_ERROR, Json(res))
            }
        }


}

pub async fn explorer_delete_file(
    State(server_database): State<Db>,
    Form(input): Form<DeleteInput>,
) -> (StatusCode, Json<ApiResponse>) {

    tracing::debug!("explorer_delete_file");
    let mut log_data : Vec<String> = Vec::new();

    // Get the name parameter from the input
    if input.filename.is_empty() {

        log_data.push("Error : file name is empty".to_string());
        let res : ApiResponse = ApiResponse {
            logs: log_data,
            file: None,
            files: Vec::new()
        };
        return (StatusCode::BAD_REQUEST, Json(res));
    };

  
        let result = sqlx::query!("DELETE FROM pages WHERE name = ?", input.filename)
        .execute(& server_database.sqliteDb)
        .await;

        match result {
            Ok(result_data) => {
                   // Check the number of affected rows
                if result_data.rows_affected() > 0 {
                    tracing::error!("Successfully deleted the row with name: {}", input.filename);
                    log_data.push("Error : file data is deleted ".to_string());
                    let res : ApiResponse = ApiResponse {
                        logs: log_data,
                        file: None,
                        files: Vec::new()
                    };
                    return (StatusCode::OK, Json(res));
                } else {
                    tracing::error!("No row found with ID: {}", input.filename);

                    log_data.push(format!("Error : No row found with name {}", input.filename));
                    let res : ApiResponse = ApiResponse {
                        logs: log_data,
                        file: None,
                        files: Vec::new()
                    };
                    return (StatusCode::NOT_FOUND, Json(res));
                }
            }
            Err(e) => {
                tracing::error!("Failed to update page data: {:?}", e);
                log_data.push(format!("Error occurred {}", e));
                    let res : ApiResponse = ApiResponse {
                        logs: log_data,
                        file: None,
                        files: Vec::new()
                    };
                (StatusCode::INTERNAL_SERVER_ERROR, Json(res))
            }
        }



}


pub async fn explorer_witness_file(
    State(server_database): State<Db>,
    Form(input): Form<WitnessInput>,
) -> (StatusCode, Json<ApiResponse>) {

    tracing::debug!("explorer_witness_file");
    let mut log_data : Vec<String> = Vec::new();

    // Get the name parameter from the input
    if input.filename.is_empty() {

        log_data.push("Error : file name is empty".to_string());
        let res : ApiResponse = ApiResponse {
            logs: log_data,
            file: None,
            files: Vec::new()
        };
        return (StatusCode::BAD_REQUEST, Json(res));
    };

    log_data.push("Success : file name is not  empty".to_string());

    // Fetch a single row from the 'pages' table where name matches
    let row = sqlx::query!(
        "SELECT id, name, extension, page_data , owner , mode FROM pages WHERE name = ? LIMIT 1",
        input.filename
    )
    .fetch_optional(&server_database.sqliteDb)
    .await;

    // Handle database result errors
    match row {
        Ok(Some(row)) => {
            log_data.push (format!("Success :  Page data for {} not found in database", input.filename));

            // Deserialize page data
            let deserialized: PageDataContainer = match serde_json::from_str(&row.page_data) {
                Ok(data) => {
                    
                    log_data.push("Success :  Page Data Object parse".to_string());
            
                    data
                },
                Err(e) => {
                    tracing::error!("Failed to parse page data record: {:?}", e);

                    

                    log_data.push("Error : Failure to parse Page Data Object".to_string());
            
                  
                   let res : ApiResponse = ApiResponse {
                    logs: log_data,
                    file: None,
                    files: Vec::new()
                };

                    return (StatusCode::INTERNAL_SERVER_ERROR, Json(res));
                }
            };

            let mut doc = deserialized.clone();
            let len = doc.pages[0].revisions.len();

            let (ver1, rev1) = &doc.pages[0].revisions[len - 1].clone();

            let mut rev2 = rev1.clone();
            rev2.metadata.previous_verification_hash = Some(*ver1);

            let txHash = match input.tx_hash.parse::<TxHash>() {
                Ok(s) => s,
                Err(e) => {
                    tracing::error!("Failed to parse tx hash: {:?}", e);
                    log_data.push (format!("Error :  Failed to to parse tx hash: {:?}", e));

                    let res : ApiResponse = ApiResponse {
                        logs: log_data,
                        file: None,
                        files: Vec::new()
                    };

                    return (StatusCode::BAD_REQUEST, Json(res));
                }
            };

            log_data.push (format!("Success :  Parsed tx hash: {:?}", txHash));
            tracing::debug!("Tx hash: {}", txHash);

            let wallet_address = match ethaddr::Address::from_str_checksum(&input.wallet_address) {
                Ok(a) => a,
                Err(e) => {
                    tracing::error!("Failed to parse wallet address: {:?}", e);
                    log_data.push (format!("Error :  Failed to parse wallet address: {:?}", e));

                    let res : ApiResponse = ApiResponse {
                        logs: log_data ,
                        file: None,
                        files: Vec::new()
                    };
                    return (StatusCode::BAD_REQUEST, Json(res));
                }
            };
            log_data.push (format!("Success  :  parsed wallet address: {:?}", wallet_address));


            let domain_snapshot_genesis_string = &deserialized.pages.get(0).unwrap().genesis_hash;

            let mut hasher = sha3::Sha3_512::default();
            hasher.update("");

            let domain_snapshot_genesis_hash = Hash::from(hasher.finalize());
            tracing::debug!(
                "Rev1 Metadata verification hash: {}",
                rev1.metadata.verification_hash
            );

            let witness_hash = witness_hash(
                &domain_snapshot_genesis_hash,
                &rev1.metadata.verification_hash,
                "sepolia",
                &txHash,
            );
            tracing::debug!("Tx hash after user: {}", txHash);

            let mut merkle_tree_successor_hasher = sha3::Sha3_512::default();
            merkle_tree_successor_hasher.update(format!(
                "{}{}",
                &rev1.metadata.verification_hash.to_string(),
                make_empty_hash().to_string()
            ));

            let merkle_tree_successor = Hash::from(merkle_tree_successor_hasher.finalize());

            let mut merkle_tree = Vec::new();
            merkle_tree.push(MerkleNode {
                left_leaf: rev1.metadata.verification_hash,
                right_leaf: make_empty_hash(),
                successor: merkle_tree_successor,
            });

            let mut hasher_verification = sha3::Sha3_512::default();
            hasher_verification.update(format!(
                "{}{}",
                domain_snapshot_genesis_hash.to_string(),
                &rev1.metadata.verification_hash.to_string()
            ));

            let witness_event_verification_hash = Hash::from(hasher_verification.finalize());
            tracing::debug!(
                "Witness event verification hash: {}",
                witness_event_verification_hash
            );

            rev2.witness = Some(RevisionWitness {
                domain_snapshot_genesis_hash: domain_snapshot_genesis_hash,
                merkle_root: rev1.metadata.verification_hash,
                witness_network: "sepolia".to_string(),
                witness_event_transaction_hash: txHash,
                witness_event_verification_hash: witness_event_verification_hash,
                witness_hash: witness_hash,
                structured_merkle_proof: merkle_tree,
            });

            rev2.signature = None;

            let timestamp_current = Timestamp::from(chrono::Utc::now().naive_utc());
            rev2.metadata.time_stamp = timestamp_current.clone();

            tracing::debug!(
                "Current timestamp: {}, domain id: {}, previous Ver: {}",
                timestamp_current,
                doc.pages[0].domain_id,
                ver1
            );

            let metadata_hash_current =
                metadata_hash(&doc.pages[0].domain_id, &timestamp_current, Some(ver1));

            let verification_hash_current = verification_hash(
                &rev2.content.content_hash,
                &metadata_hash_current,
                None,
                Some(&witness_hash),
            );

            rev2.metadata.metadata_hash = metadata_hash_current;
            rev2.metadata.verification_hash = verification_hash_current;

            doc.pages[0]
                .revisions
                .push((verification_hash_current, rev2));

            // Serialize the updated document
            let page_data_new = match serde_json::to_string(&doc) {
                Ok(data) => data,
                Err(e) => {
                    tracing::error!("Failed to serialize updated page data: {:?}", e);
                    log_data.push (format!("Error :  Failed to serialize updated page data {:?} ", e));

                    let res : ApiResponse = ApiResponse {
                        logs: log_data ,
                        file: None,
                        files: Vec::new()
                    };
                    return (StatusCode::INTERNAL_SERVER_ERROR, Json(res));
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
                        owner : row.owner,
                        mode : row.mode
                    };

                   let res : ApiResponse =  ApiResponse{
                    logs: log_data ,
                    file: Some(file_info),
                    files: Vec::new()
                };
                    (StatusCode::OK, Json(res))
                }
                Err(e) => {
                    tracing::error!("Failed to update page data: {:?}", e);
                    log_data.push (format!("Error :  unable to update database with Page data for {} ", input.filename));

                    let res : ApiResponse = ApiResponse {
                        logs: log_data ,
                        file: None,
                        files: Vec::new()
                    };
                    (StatusCode::INTERNAL_SERVER_ERROR, Json(res))
                }
            }
        }
        
        Ok(None) => {

            log_data.push (format!("Error :  Page data for {} not found in database", input.filename));

            let res : ApiResponse = ApiResponse {
                logs: log_data ,
                file: None,
                files: Vec::new()
            };
            (StatusCode::NOT_FOUND, Json(res))
        },
        Err(e) => {
            
            tracing::error!("Failed to fetch record: {:?}", e);
            log_data.push (format!("Error :  Failed to fetch record {:?}", e));

            let res : ApiResponse = ApiResponse {
                logs: log_data ,
                file: None,
                files: Vec::new()
            };
            
            (StatusCode::INTERNAL_SERVER_ERROR, Json(res))
        }
    }
}


pub async fn explorer_fetch_configuration(
    State(server_database): State<Db>,
) -> (StatusCode, Json<HashMap<String, String>>) {
    let mut config_data = HashMap::new();

    tracing::debug!("explorer_sign_revision");

    let api_domain = env::var("API_DOMAIN").unwrap_or_default();
    let chain = env::var("CHAIN").unwrap_or_default();
    let mode = env::var("FILE_MODE").unwrap_or_default();

    config_data.insert("chain".to_string(), chain);
    config_data.insert("domain".to_string(), api_domain);
    config_data.insert("mode".to_string(), mode);

    return (StatusCode::OK, Json(config_data));
}


pub async fn explorer_update_configuration(
    State(server_database): State<Db>,
    Form(input): Form<UpdateConfigurationInput>,
) -> (StatusCode, Json<ApiResponse>) {
    let mut log_data : Vec<String> = Vec::new();

    // Log the input
    log_data.push(format!("Updating configuration with chain: {} and domain: {} mode : {}", input.chain, input.domain, input.mode));

    // Update the .env file with the new chain and domain
    if let Err(e) = update_env_file("CHAIN", &input.chain) {
        log_data.push(format!("Failed to update CHAIN in .env file: {}", e));
    }

    if let Err(e) = update_env_file("API_DOMAIN", &input.domain) {
        log_data.push(format!("Failed to update API_DOMAIN in .env file: {}", e));
    }

    if let Err(e) = update_env_file("FILE_MODE", &input.mode) {
        log_data.push(format!("Failed to update FILE_MODE in .env file: {}", e));
    }

    // Prepare the response
    let res = ApiResponse {
        logs: log_data,
        file: None,
        files: Vec::new()
    };

    (StatusCode::OK, Json(res))

}
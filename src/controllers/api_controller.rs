use crate::models::input::{DeleteInput, RevisionInput, UpdateConfigurationInput, WitnessInput};
use crate::models::page_data::{ApiResponse, PageDataContainer};
use crate::models::{file::FileInfo, page_data};
use crate::util::{
    check_if_page_data_revision_are_okay, check_or_generate_domain, compute_content_hash,
    get_content_type, get_file_info, make_empty_hash, update_env_file,
};
use crate::Db;
use aqua_verifier::util::{
    content_hash, metadata_hash, signature_hash, verification_hash, witness_hash,
};
use aqua_verifier_rs_types::models::base64::Base64;
use aqua_verifier_rs_types::models::content::RevisionContentContent;
use aqua_verifier_rs_types::models::content::{FileContent, RevisionContent};
use aqua_verifier_rs_types::models::hash::Hash;
use aqua_verifier_rs_types::models::metadata::RevisionMetadata;
use aqua_verifier_rs_types::models::page_data::HashChain;
use aqua_verifier_rs_types::models::public_key::PublicKey;
use aqua_verifier_rs_types::models::revision::Revision;
use aqua_verifier_rs_types::models::signature::{RevisionSignature, Signature};
use aqua_verifier_rs_types::models::timestamp::Timestamp;
use aqua_verifier_rs_types::models::tx_hash::TxHash;
use aqua_verifier_rs_types::models::witness::{MerkleNode, RevisionWitness};
use axum::response::{IntoResponse, Response};
use axum::{
    body::Bytes,
    extract::{DefaultBodyLimit, Multipart, Path, Request, State},
    handler::HandlerWithoutStateExt,
    http::{HeaderMap, StatusCode},
    response::{Html, Redirect},
    routing::{get, post},
    BoxError, Form, Json, Router,
};
use chrono::{DateTime, NaiveDateTime, Utc};
use dotenv::from_path;
use ethaddr::address;
use ethers::core::k256::sha2::Sha256;
use futures::{Stream, TryStreamExt};
use serde::{Deserialize, Serialize};
extern crate serde_json_path_to_error as serde_json;
use crate::db::pages_db::{
    db_data, delete_all_data, delete_all_user_files, delete_page_data,
    fetch_all_pages_data_per_user, fetch_page_data, insert_page_data, update_page_data,
};
use dotenv::{dotenv, vars};
use sha3::{Digest, Sha3_512};
use std::collections::HashMap;
use std::env;
use std::error::Error;
use std::fs;
use std::io;
use std::net::SocketAddr;
use std::time::SystemTime;
use std::{collections::BTreeMap, time::UNIX_EPOCH};
use tokio::{fs::File, io::BufWriter};
use tokio_util::io::StreamReader;
use tower::ServiceExt;
use tracing_subscriber::{fmt::format, layer::SubscriberExt, util::SubscriberInitExt};

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
    let mut log_data: Vec<String> = Vec::new();
    let mut res: ApiResponse = ApiResponse {
        logs: log_data.clone(),
        file: None,
        files: Vec::new(),
    };

    // Initialize an empty Vec to hold the result
    let mut pages: Vec<FileInfo> = Vec::new();

    let user_address: Result<String, String> = match headers.get("metamask_address") {
        Some(value) => match value.to_str() {
            Ok(address) => Ok(address.to_string()), // Successfully extracted address
            Err(_) => Err("Failed to parse the metamask_address header.".to_string()),
        },
        None => Err("metamask_address header not found.".to_string()), // Header not found
    };

    let mut conn = match server_database.pool.get() {
        Ok(connection) => connection,
        Err(e) => {
            log_data.push("Failed data not found in database".to_string());

            log_data.push("Failed to get database connection".to_string());
            let res: ApiResponse = ApiResponse {
                logs: log_data.clone(),
                file: None,
                files: Vec::new(),
            };
            return (StatusCode::INTERNAL_SERVER_ERROR, Json(res));
        }
    };

    if user_address.is_ok() {
        let page_data_result = fetch_all_pages_data_per_user(user_address.unwrap(), &mut conn);
        if page_data_result.is_err() {
            log_data.push("Failed data not found in database".to_string());

            let res: ApiResponse = ApiResponse {
                logs: log_data.clone(),
                file: None,
                files: Vec::new(),
            };
            return (StatusCode::NOT_FOUND, Json(res));
        }

        let page_data = page_data_result.unwrap();

        for row in page_data {
            pages.push(FileInfo {
                id: row.id.unwrap().try_into().unwrap(),
                name: row.name,
                extension: row.extension,
                page_data: row.page_data,
                mode: row.mode,
                owner: row.owner,
            });
        }

        if pages.is_empty() {
            res.logs.push("No pages found".to_string());
            return (StatusCode::OK, Json::from(res));
        }

        res.files = pages;
        // Return the populated list of pages
        return (StatusCode::OK, Json::from(res));
    } else {
        log_data
            .push("Unable to parse metamask address/ Metamask address not provided".to_string());
        let res: ApiResponse = ApiResponse {
            logs: log_data.clone(),
            file: None,
            files: Vec::new(),
        };
        return (StatusCode::INTERNAL_SERVER_ERROR, Json(res));
    }

    (StatusCode::INTERNAL_SERVER_ERROR, Json::from(res))
}

pub async fn explorer_file_verify_hash_upload(
    State(server_database): State<Db>,
    mut multipart: Multipart,
) -> (StatusCode, Json<ApiResponse>) {
    tracing::debug!("explorer_file_verify_hash_upload fn");
    let mut log_data: Vec<String> = Vec::new();
    let mut res: ApiResponse = ApiResponse {
        logs: log_data,
        file: None,
        files: Vec::new(),
    };

    while let Ok(Some(field)) = multipart.next_field().await {
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
            "file" => {
                let file_name = field.file_name().unwrap_or_default().to_string();

                if std::path::Path::new(&file_name)
                    .extension()
                    .and_then(|s| s.to_str())
                    != Some("json")
                {
                    tracing::error!("Uploaded file is not a JSON file");
                    res.logs
                        .push("Error: Uploaded file is not a JSON file".to_string());
                    return (StatusCode::BAD_REQUEST, Json(res));
                }

                // Read the field into a byte vector
                let data = match field.bytes().await {
                    Ok(data) => data,
                    Err(e) => {
                        tracing::error!("Failed to read file data: {:?}", e);
                        res.logs
                            .push(format!("Error: Failed to read file data: {:?}", e));
                        return (StatusCode::BAD_REQUEST, Json(res));
                    }
                };

                // Try to parse the file content into your struct
                match serde_json::from_slice::<PageDataContainer<HashChain>>(&data) {
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
                                    return (StatusCode::OK, Json(res));
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
                        return (StatusCode::BAD_REQUEST, Json(res));
                    }
                }
            }
            _ => continue,
        }
    }

    // Return an error if no file was found
    (StatusCode::BAD_REQUEST, Json(res))
}

pub async fn explorer_aqua_file_upload(
    State(server_database): State<Db>,
    headers: HeaderMap,
    mut multipart: Multipart,
) -> (StatusCode, Json<ApiResponse>) {
    tracing::debug!("explorer_aqua_file_upload fn");
    let mut log_data: Vec<String> = Vec::new();
    let mut res: ApiResponse = ApiResponse {
        logs: log_data,
        file: None,
        files: Vec::new(),
    };

    // Extract the 'metamask_address' header
    let metamask_address = match headers.get("metamask_address") {
        Some(value) => match value.to_str() {
            Ok(key) => key,
            Err(err) => {
                tracing::error!("headers get error {} ", err);
                // return (StatusCode::BAD_REQUEST,  Json(json!({"error": "Invalid metamask_address header"})))

                res.logs
                    .push(format!("Error: Meta mask public key  error: {:?}", err));
                return (StatusCode::BAD_REQUEST, Json(res));
            }
        },
        None => {
            tracing::debug!("metamask_address header missing ");
            // return (StatusCode::BAD_REQUEST, Json(json!({"error": "metamask_address header missing"})))
            res.logs
                .push("Error: Meta mask public key  missing".to_string());
            return (StatusCode::BAD_REQUEST, Json(res));
        }
    };

    let mut account: Option<String> = None;
    let mut aqua_json: Option<PageDataContainer<HashChain>> = None;

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
                        res.logs
                            .push(format!("Failed to read account field: {}", e));
                        return (StatusCode::BAD_REQUEST, Json(res));
                    }
                };
            }
            "file" => {
                // Read the file content
                let file_content = match field.bytes().await {
                    Ok(content) => content,
                    Err(e) => {
                        tracing::error!("Failed to read file content: {}", e);
                        res.logs.push(format!("Failed to read file content: {}", e));
                        return (StatusCode::BAD_REQUEST, Json(res));
                    }
                };

                // Parse JSON content into AquaData struct
                aqua_json =
                    match serde_json::from_slice::<PageDataContainer<HashChain>>(&file_content) {
                        Ok(data) => Some(data),
                        Err(e) => {
                            tracing::error!("Failed to parse JSON: {}", e);
                            res.logs.push(format!("Failed to parse JSON: {}", e));
                            return (StatusCode::BAD_REQUEST, Json(res));
                        }
                    };
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

    // Verify we have both account and file
    let aqua_json = match aqua_json {
        Some(acc) => acc,
        None => {
            tracing::error!("Aqua JSON data erorr");
            res.logs.push("Aqua JSON data erorr".to_string());
            return (StatusCode::BAD_REQUEST, Json(res));
        }
    };

    tracing::debug!(
        "Processing aqua  upload Account {} - File data {:#?} ",
        account,
        aqua_json,
    );

    let mut mode = "private".to_string();
    let file_mode = env::var("FILE_MODE").unwrap_or_default();

    if !file_mode.is_empty() {
        mode = file_mode;
    }

    let start = SystemTime::now();
    let timestamp = start
        .duration_since(UNIX_EPOCH)
        .expect("Time went backwards")
        .as_secs();
    println!("Current Unix timestamp: {}", timestamp);

    let mut file_name = format!("{}", timestamp);

    let chain: Option<&HashChain> = aqua_json.pages.get(0);

    if (chain.is_none()) {
        tracing::error!("Aqua JSON data erorr first chain not found");
        res.logs
            .push("Aqua data data erorr first chain not found".to_string());
        return (StatusCode::BAD_REQUEST, Json(res));
    }

    let genesis_revision = chain
        .unwrap()
        .revisions
        .iter()
        .find_map(|(hash, revision)| {
            if hash.to_string() == chain.unwrap().genesis_hash {
                tracing::info!("Found geneisis revision");
                Some(revision)
            } else {
                tracing::error!("genesis revision not found");
                None
            }
        });
    if (genesis_revision.is_none()) {
        tracing::error!("Aqua JSON data erorr genesis revision not found");
        res.logs
            .push("Aqua data data erorr genesis revision not found".to_string());
        return (StatusCode::BAD_REQUEST, Json(res));
    }
    if (genesis_revision.unwrap().content.file.is_none()) {
        tracing::error!("Aqua JSON data erorr genesis revision does not contain file info");
        res.logs
            .push("Aqua data data erorr genesis revision does not contain file info".to_string());
        return (StatusCode::BAD_REQUEST, Json(res));
    }
    let file_name = genesis_revision
        .unwrap()
        .content
        .file
        .clone()
        .unwrap()
        .filename;

    let path = std::path::Path::new(&file_name);
    let mut content_type: String = String::from("");

    // Check if the file has an extension
    if (path.extension().is_some()) {
        tracing::error!("Aqua JSON generating file type from extension");
        res.logs
            .push("Aqua JSON generating file type from extension".to_string());

        match get_content_type(&file_name) {
            Some(data) => {
                content_type = data;
            }
            None => {
                content_type = "unknown".to_string();
            }
        }
    } else {
        tracing::error!("Aqua JSON generating file type from content bytes");
        res.logs
            .push("Aqua data data erorr genesis type from content bytes".to_string());

        let file_data_info = get_file_info(
            genesis_revision
                .unwrap()
                .content
                .file
                .clone()
                .unwrap()
                .data
                .to_string(),
        );

        content_type = match file_data_info {
            Ok(data) => {
                tracing::error!(
                    "file type found   {} the gerenal result is  {:#?}",
                    data.file_type,
                    data
                );
                data.file_type
            }
            Err(err) => {
                tracing::error!("Failed infer file type  {}", err);
                "unknown".to_string()
            }
        };
    }

    // Convert struct to JSON string
    let json_string = match serde_json::to_string(&aqua_json) {
        Ok(json) => json,
        Err(e) => {
            tracing::error!("Failed to serialize page data: {}", e);
            res.logs
                .push(format!("Failed to serialize page data: {}", e));
            return (StatusCode::INTERNAL_SERVER_ERROR, Json(res));
        }
    };

    let db_data_model = db_data {
        id: 0,
        name: file_name,
        extension: content_type,
        page_data: json_string,
        mode: mode,
        owner: metamask_address.to_string(),
    };

    let mut conn = match server_database.pool.get() {
        Ok(connection) => connection,
        Err(e) => {
            res.logs
                .push("Failed to get database connection".to_string());

            println!("Error Fetching connection {:#?}", res);
            return (StatusCode::INTERNAL_SERVER_ERROR, Json(res));
        }
    };

    let insert_result = insert_page_data(db_data_model.clone(), &mut conn);
    if insert_result.is_err() {
        let err = insert_result.err().unwrap();
        tracing::error!("Failed to insert page: {}", err);
        res.logs.push(format!("Failed to insert page: {}", err));
        return (StatusCode::INTERNAL_SERVER_ERROR, Json(res));
    }

    let file_info = FileInfo {
        id: insert_result.unwrap(), //record.id,
        name: db_data_model.name,
        extension: db_data_model.extension,
        page_data: db_data_model.page_data,
        mode: db_data_model.mode,
        owner: db_data_model.owner.to_string(),
    };
    res.file = Some(file_info);
    return (StatusCode::CREATED, Json(res));
}

pub async fn explorer_file_upload(
    State(server_database): State<Db>,
    headers: HeaderMap,
    mut multipart: Multipart,
) -> (StatusCode, Json<ApiResponse>) {
    tracing::debug!("explorer_file_upload fn");

    let mut log_data: Vec<String> = Vec::new();
    let mut res: ApiResponse = ApiResponse {
        logs: log_data,
        file: None,
        files: Vec::new(),
    };

    // Extract the 'metamask_address' header
    let metamask_address = match headers.get("metamask_address") {
        Some(value) => match value.to_str() {
            Ok(key) => key,
            Err(err) => {
                tracing::error!("headers get error {} ", err);
                // return (StatusCode::BAD_REQUEST,  Json(json!({"error": "Invalid metamask_address header"})))

                res.logs
                    .push(format!("Error: Meta mask public key  error: {:?}", err));
                return (StatusCode::BAD_REQUEST, Json(res));
            }
        },
        None => {
            tracing::debug!("metamask_address header missing ");
            // return (StatusCode::BAD_REQUEST, Json(json!({"error": "metamask_address header missing"})))
            res.logs
                .push("Error: Meta mask public key  missing".to_string());
            return (StatusCode::BAD_REQUEST, Json(res));
        }
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
                        res.logs
                            .push(format!("Failed to read account field: {}", e));
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
                    res.logs.push(format!(
                        "File size {} exceeds maximum allowed size",
                        file_size
                    ));
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

    let revision_content_content = RevisionContentContent {
        file_hash: file_hash_current,
    };

    let pagedata_current = PageDataContainer {
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
                        content: revision_content_content, //content_current,
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
            res.logs
                .push(format!("Failed to serialize page data: {}", e));
            return (StatusCode::INTERNAL_SERVER_ERROR, Json(res));
        }
    };

    let mut mode = "private".to_string();
    let file_mode = env::var("FILE_MODE").unwrap_or_default();

    if !file_mode.is_empty() {
        mode = file_mode;
    }

    let db_data_model = db_data {
        id: 0,
        name: file_name,
        extension: content_type,
        page_data: json_string,
        mode: mode,
        owner: metamask_address.to_string(),
    };

    let mut conn = match server_database.pool.get() {
        Ok(connection) => connection,
        Err(e) => {
            res.logs
                .push("Failed to get database connection".to_string());

            println!("Error Fetching connection {:#?}", res);
            return (StatusCode::INTERNAL_SERVER_ERROR, Json(res));
        }
    };

    let insert_result = insert_page_data(db_data_model.clone(), &mut conn);

    if insert_result.is_err() {
        let er = insert_result.err().unwrap();
        tracing::error!("Failed to insert page: {}", er.clone());
        res.logs.push(format!("Failed to insert page: {}", er));
        return (StatusCode::INTERNAL_SERVER_ERROR, Json(res));
    }

    let file_info = FileInfo {
        id: insert_result.unwrap(), //record.id,
        name: db_data_model.name,
        extension: db_data_model.extension,
        page_data: db_data_model.page_data,
        mode: db_data_model.mode,
        owner: db_data_model.owner.to_string(),
    };

    res.file = Some(file_info);
    return (StatusCode::CREATED, Json(res));
}

pub async fn explorer_sign_revision(
    State(server_database): State<Db>,
    Form(input): Form<RevisionInput>,
) -> (StatusCode, Json<ApiResponse>) {
    let mut log_data: Vec<String> = Vec::new();

    tracing::debug!("explorer_sign_revision");

    // Get the name parameter from the input
    if input.filename.is_empty() {
        log_data.push("Error : file name is empty".to_string());
        let res: ApiResponse = ApiResponse {
            logs: log_data,
            file: None,
            files: Vec::new(),
        };
        return (StatusCode::BAD_REQUEST, Json(res));
    };

    let mut conn = match server_database.pool.get() {
        Ok(connection) => connection,
        Err(e) => {
            log_data.push("Failed data not found in database".to_string());

            log_data.push("Failed to get database connection".to_string());
            let res: ApiResponse = ApiResponse {
                logs: log_data,
                file: None,
                files: Vec::new(),
            };

            println!("Error Fetching connection {:#?}", res);
            return (StatusCode::INTERNAL_SERVER_ERROR, Json(res));
        }
    };

    // let insert_result = insert_page_data(db_data_model.clone(), & mut conn);
    let page_data_result = fetch_page_data(input.filename, &mut conn);

    if page_data_result.is_err() {
        tracing::error!("Failed not found ",);

        log_data.push("Failed data not found in database".to_string());

        let res: ApiResponse = ApiResponse {
            logs: log_data,
            file: None,
            files: Vec::new(),
        };
        return (StatusCode::NOT_FOUND, Json(res));
    }
    let page_data = page_data_result.unwrap();

    let deserialized: PageDataContainer<HashChain> =
        match serde_json::from_str(&page_data.page_data) {
            Ok(data) => {
                log_data.push("Success  : parse page data".to_string());
                data
            }
            Err(e) => {
                tracing::error!("Failed to parse page data record: {:?}", e);
                log_data.push(format!("error : Failed to parse page data record: {:?}", e));
                if let Some(source) = e.source() {
                    tracing::info!("Source: {}", source);
                } else {
                    tracing::info!("Source NOT FOUND ");
                }
                let res: ApiResponse = ApiResponse {
                    logs: log_data,
                    file: None,
                    files: Vec::new(),
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
        }
        Err(e) => {
            tracing::error!("Failed to parse signature: {:?}", e);

            log_data.push(format!("error : Failed to parse  signature: {:?}", e));
            let res: ApiResponse = ApiResponse {
                logs: log_data,
                file: None,
                files: Vec::new(),
            };

            return (StatusCode::BAD_REQUEST, Json(res));
        }
    };
    let pubk = match input.publickey.parse::<PublicKey>() {
        Ok(p) => {
            log_data.push("Success : public  key  parsed successfully".to_string());

            p
        }
        Err(e) => {
            tracing::error!("Failed to parse public key: {:?}", e);

            log_data.push(format!("error : Failed to parse  public key: {:?}", e));
            let res: ApiResponse = ApiResponse {
                logs: log_data,
                file: None,
                files: Vec::new(),
            };

            return (StatusCode::BAD_REQUEST, Json(res));
        }
    };
    let addr = match ethaddr::Address::from_str_checksum(&input.wallet_address) {
        Ok(a) => {
            log_data.push("wallet address parsed successfully".to_string());

            a
        }
        Err(e) => {
            tracing::error!("Failed to parse wallet address: {:?}", e);
            log_data.push(format!("Failed to parse wallet address: {:?}", e));

            let res: ApiResponse = ApiResponse {
                logs: log_data,
                file: None,
                files: Vec::new(),
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

    println!("Revisisons len is: {}", doc.pages[0].revisions.len());

    // Serialize the updated document
    let page_data_new = match serde_json::to_string(&doc) {
        Ok(data) => {
            log_data.push("revision  serialized  successfully".to_string());

            data
        }
        Err(e) => {
            tracing::error!("Failed to serialize updated page data: {:?}", e);

            log_data.push(format!("Failed to serialize updated page data : {:?}", e));

            let res: ApiResponse = ApiResponse {
                logs: log_data,
                file: None,
                files: Vec::new(),
            };

            return (StatusCode::INTERNAL_SERVER_ERROR, Json(res));
        }
    };

    let mut new_data = page_data.clone();
    new_data.page_data = page_data_new.clone();

    let mut conn = match server_database.pool.get() {
        Ok(connection) => connection,
        Err(e) => {
            log_data.push("Failed data not found in database".to_string());

            log_data.push("Failed to get database connection".to_string());
            let res: ApiResponse = ApiResponse {
                logs: log_data,
                file: None,
                files: Vec::new(),
            };

            println!("Error Fetching connection {:#?}", res);
            return (StatusCode::INTERNAL_SERVER_ERROR, Json(res));
        }
    };

    // let insert_result = insert_page_data(db_data_model.clone(), & mut conn);
    // let page_data_result = fetch_page_data(input.filename, & mut conn);

    let update_result = update_page_data(new_data.clone(), &mut conn);
    if update_result.is_err() {
        let e = update_result.err().unwrap();
        tracing::error!("Failed to update page data: {:?}", e);
        log_data.push(format!("Failed to update page data : {:?}", e));

        let res: ApiResponse = ApiResponse {
            logs: log_data,
            file: None,
            files: Vec::new(),
        };
        return (StatusCode::INTERNAL_SERVER_ERROR, Json(res));
    }

    let file_info = FileInfo {
        id: new_data.id,
        name: new_data.name,
        extension: new_data.extension,
        page_data: page_data_new.clone(),
        owner: new_data.owner,
        mode: new_data.mode,
    };
    let res: ApiResponse = ApiResponse {
        logs: log_data,
        file: Some(file_info),
        files: Vec::new(),
    };
    return (StatusCode::OK, Json(res));
}

pub async fn explorer_delete_all_files(
    State(server_database): State<Db>,
    headers: HeaderMap,
) -> (StatusCode, Json<ApiResponse>) {
    let mut log_data: Vec<String> = Vec::new();

    let user_address: Result<String, String> = match headers.get("metamask_address") {
        Some(value) => match value.to_str() {
            Ok(address) => Ok(address.to_string()), // Successfully extracted address
            Err(_) => Err("Failed to parse the metamask_address header.".to_string()),
        },
        None => Err("metamask_address header not found.".to_string()), // Header not found
    };

    if user_address.is_err() {
        log_data
            .push("Unable to parse metamask address/ Metamask address not provided".to_string());
        let res: ApiResponse = ApiResponse {
            logs: log_data.clone(),
            file: None,
            files: Vec::new(),
        };
        return (StatusCode::INTERNAL_SERVER_ERROR, Json(res));
    }

    let mut conn = match server_database.pool.get() {
        Ok(connection) => connection,
        Err(e) => {
            log_data.push("Failed data not found in database".to_string());

            log_data.push("Failed to get database connection".to_string());
            let res: ApiResponse = ApiResponse {
                logs: log_data,
                file: None,
                files: Vec::new(),
            };

            println!("Error Fetching connection {:#?}", res);
            return (StatusCode::INTERNAL_SERVER_ERROR, Json(res));
        }
    };

    let addr = user_address.unwrap();
    let result = delete_all_user_files(addr, &mut conn);

    match result {
        Ok(result_data) => {
            // Check the number of affected rows
            // if result_data.rows_affected() > 0 {
            tracing::error!("Successfully deleted all the row with name");
            log_data.push("Error : files data is deleted ".to_string());
            let res: ApiResponse = ApiResponse {
                logs: log_data,
                file: None,
                files: Vec::new(),
            };
            return (StatusCode::OK, Json(res));
            // } else {
            //     tracing::error!("Error : No row deleted");

            //     log_data.push("Error : No data deleted".to_string());
            //     let res: ApiResponse = ApiResponse {
            //         logs: log_data,
            //         file: None,
            //         files: Vec::new(),
            //     };
            //     return (StatusCode::OK, Json(res));
            // }
        }
        Err(e) => {
            tracing::error!("Failed to delete page data: {:?}", e);
            log_data.push(format!("Error occurred {}", e));
            let res: ApiResponse = ApiResponse {
                logs: log_data,
                file: None,
                files: Vec::new(),
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
    let mut log_data: Vec<String> = Vec::new();

    // Get the name parameter from the input
    if input.filename.is_empty() {
        log_data.push("Error : file name is empty".to_string());
        let res: ApiResponse = ApiResponse {
            logs: log_data,
            file: None,
            files: Vec::new(),
        };
        return (StatusCode::BAD_REQUEST, Json(res));
    };

    // let result = sqlx::query!("DELETE FROM pages WHERE name = ?", input.filename)
    //     .execute(&server_database.sqliteDb)
    //     .await;

    let mut conn = match server_database.pool.get() {
        Ok(connection) => connection,
        Err(e) => {
            log_data.push("Failed data not found in database".to_string());

            log_data.push("Failed to get database connection".to_string());
            let res: ApiResponse = ApiResponse {
                logs: log_data,
                file: None,
                files: Vec::new(),
            };

            println!("Error Fetching connection {:#?}", res);
            return (StatusCode::INTERNAL_SERVER_ERROR, Json(res));
        }
    };

    let result = delete_page_data(input.filename.clone(), &mut conn);

    match result {
        Ok(result_data) => {
            // Check the number of affected rows
            if result_data > 0 {
                tracing::error!("Successfully deleted the row with name: {}", input.filename);
                log_data.push("Error : file data is deleted ".to_string());
                let res: ApiResponse = ApiResponse {
                    logs: log_data,
                    file: None,
                    files: Vec::new(),
                };
                return (StatusCode::OK, Json(res));
            } else {
                tracing::error!("No row found with ID: {}", input.filename);

                log_data.push(format!("Error : No row found with name {}", input.filename));
                let res: ApiResponse = ApiResponse {
                    logs: log_data,
                    file: None,
                    files: Vec::new(),
                };
                return (StatusCode::NOT_FOUND, Json(res));
            }
        }
        Err(e) => {
            tracing::error!("Failed to update page data: {:?}", e);
            log_data.push(format!("Error occurred {}", e));
            let res: ApiResponse = ApiResponse {
                logs: log_data,
                file: None,
                files: Vec::new(),
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
    let mut log_data: Vec<String> = Vec::new();

    // Get the name parameter from the input
    if input.filename.is_empty() {
        log_data.push("Error : file name is empty".to_string());
        let res: ApiResponse = ApiResponse {
            logs: log_data,
            file: None,
            files: Vec::new(),
        };
        return (StatusCode::BAD_REQUEST, Json(res));
    };

    if input.network.is_empty() {
        log_data.push("Error : Network is empty".to_string());
        let res: ApiResponse = ApiResponse {
            logs: log_data,
            file: None,
            files: Vec::new(),
        };
        return (StatusCode::BAD_REQUEST, Json(res));
    };

    log_data.push("Success : file name is not  empty".to_string());

    let mut conn = match server_database.pool.get() {
        Ok(connection) => connection,
        Err(e) => {
            log_data.push("Failed data not found in database".to_string());

            log_data.push("Failed to get database connection".to_string());
            let res: ApiResponse = ApiResponse {
                logs: log_data,
                file: None,
                files: Vec::new(),
            };

            println!("Error Fetching connection {:#?}", res);
            return (StatusCode::INTERNAL_SERVER_ERROR, Json(res));
        }
    };

    let page_data_result = fetch_page_data(input.filename.clone(), &mut conn);

    if page_data_result.is_err() {
        tracing::error!("Failed not found  {}", page_data_result.err().unwrap());

        log_data.push("Failed data not found in database".to_string());

        let res: ApiResponse = ApiResponse {
            logs: log_data,
            file: None,
            files: Vec::new(),
        };
        return (StatusCode::NOT_FOUND, Json(res));
    }

    let page_data = page_data_result.unwrap();

    log_data.push(format!(
        "Success :  Page data for {} not found in database",
        input.filename
    ));

    // Deserialize page data
    let deserialized: PageDataContainer<HashChain> =
        match serde_json::from_str(&page_data.page_data) {
            Ok(data) => {
                log_data.push("Success :  Page Data Object parse".to_string());

                data
            }
            Err(e) => {
                tracing::error!("Failed to parse page data record: {:?}", e);

                log_data.push("Error : Failure to parse Page Data Object".to_string());

                let res: ApiResponse = ApiResponse {
                    logs: log_data,
                    file: None,
                    files: Vec::new(),
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
            log_data.push(format!("Error :  Failed to to parse tx hash: {:?}", e));

            let res: ApiResponse = ApiResponse {
                logs: log_data,
                file: None,
                files: Vec::new(),
            };

            return (StatusCode::BAD_REQUEST, Json(res));
        }
    };

    log_data.push(format!("Success :  Parsed tx hash: {:?}", txHash));
    tracing::debug!("Tx hash: {}", txHash);

    let wallet_address = match ethaddr::Address::from_str_checksum(&input.wallet_address) {
        Ok(a) => a,
        Err(e) => {
            tracing::error!("Failed to parse wallet address: {:?}", e);
            log_data.push(format!("Error :  Failed to parse wallet address: {:?}", e));

            let res: ApiResponse = ApiResponse {
                logs: log_data,
                file: None,
                files: Vec::new(),
            };
            return (StatusCode::BAD_REQUEST, Json(res));
        }
    };
    log_data.push(format!(
        "Success  :  parsed wallet address: {:?}",
        wallet_address
    ));

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
        witness_network: input.network,
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
            log_data.push(format!(
                "Error :  Failed to serialize updated page data {:?} ",
                e
            ));

            let res: ApiResponse = ApiResponse {
                logs: log_data,
                file: None,
                files: Vec::new(),
            };
            return (StatusCode::INTERNAL_SERVER_ERROR, Json(res));
        }
    };

    let mut new_data = page_data.clone();
    new_data.page_data = page_data_new.clone();

    let mut conn = match server_database.pool.get() {
        Ok(connection) => connection,
        Err(e) => {
            log_data.push("Failed data not found in database".to_string());

            log_data.push("Failed to get database connection".to_string());
            let res: ApiResponse = ApiResponse {
                logs: log_data,
                file: None,
                files: Vec::new(),
            };

            println!("Error Fetching connection {:#?}", res);
            return (StatusCode::INTERNAL_SERVER_ERROR, Json(res));
        }
    };

    let update_result = update_page_data(new_data.clone(), &mut conn);
    if update_result.is_err() {
        let e = update_result.err().unwrap();
        tracing::error!("Failed to update page data: {:?}", e);
        log_data.push(format!("Failed to update page data : {:?}", e));

        let res: ApiResponse = ApiResponse {
            logs: log_data,
            file: None,
            files: Vec::new(),
        };
        return (StatusCode::INTERNAL_SERVER_ERROR, Json(res));
    }

    let file_info = FileInfo {
        id: new_data.id,
        name: new_data.name,
        extension: new_data.extension,
        page_data: page_data_new,
        owner: new_data.owner,
        mode: new_data.mode,
    };
    let res: ApiResponse = ApiResponse {
        logs: log_data,
        file: Some(file_info),
        files: Vec::new(),
    };
    return (StatusCode::OK, Json(res));

    // Update the database with the new document
    // let result = sqlx::query!(
    //     "UPDATE pages SET page_data = ? WHERE id = ?",
    //     page_data_new,
    //     row.id
    // )
    // .execute(&server_database.sqliteDb)
    // .await;

    // // Handle the result of the update
    // match result {
    //     Ok(_) => {
    //         let file_info = FileInfo {
    //             id: row.id,
    //             name: row.name,
    //             extension: row.extension,
    //             page_data: page_data_new,
    //             owner: row.owner,
    //             mode: row.mode,
    //         };

    //         let res: ApiResponse = ApiResponse {
    //             logs: log_data,
    //             file: Some(file_info),
    //             files: Vec::new(),
    //         };
    //         (StatusCode::OK, Json(res))
    //     }
    //     Err(e) => {
    //         tracing::error!("Failed to update page data: {:?}", e);
    //         log_data.push(format!(
    //             "Error :  unable to update database with Page data for {} ",
    //             input.filename
    //         ));

    //         let res: ApiResponse = ApiResponse {
    //             logs: log_data,
    //             file: None,
    //             files: Vec::new(),
    //         };
    //        return (StatusCode::INTERNAL_SERVER_ERROR, Json(res));
    //     }
    // }

    // Handle database result errors
    // match row {
    //     Ok(Some(row)) => {

    //     }

    //     Ok(None) => {
    //         log_data.push(format!(
    //             "Error :  Page data for {} not found in database",
    //             input.filename
    //         ));

    //         let res: ApiResponse = ApiResponse {
    //             logs: log_data,
    //             file: None,
    //             files: Vec::new(),
    //         };
    //         (StatusCode::NOT_FOUND, Json(res))
    //     }
    //     Err(e) => {
    //         tracing::error!("Failed to fetch record: {:?}", e);
    //         log_data.push(format!("Error :  Failed to fetch record {:?}", e));

    //         let res: ApiResponse = ApiResponse {
    //             logs: log_data,
    //             file: None,
    //             files: Vec::new(),
    //         };

    //         (StatusCode::INTERNAL_SERVER_ERROR, Json(res))
    //     }
    // }
}

// pub async fn explorer_fetch_configuration(
//     State(server_database): State<Db>,
// ) -> (StatusCode, Json<HashMap<String, String>>) {
//     let mut config_data = HashMap::new();

//     tracing::debug!("explorer_sign_revision");

//     dotenv().ok();

//     let api_domain = env::var("API_DOMAIN").unwrap_or_default();
//     let chain = env::var("CHAIN").unwrap_or_default();
//     let mode = env::var("FILE_MODE").unwrap_or_default();

//     tracing::debug!("api domain: {}", api_domain);
//     tracing::debug!("Chain: {}", chain);
//     tracing::debug!("File mode: {}", mode);

//     config_data.insert("chain".to_string(), chain);
//     config_data.insert("domain".to_string(), api_domain);
//     config_data.insert("mode".to_string(), mode);

//     return (StatusCode::OK, Json(config_data));
// }

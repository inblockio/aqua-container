use crate::models::input::{DeleteInput, RevisionInput, UpdateConfigurationInput, WitnessInput};
use crate::models::page_data::{ApiResponse, PageDataContainer};
use crate::models::{file::FileInfo, page_data};
use crate::util::{
    check_if_page_data_revision_are_okay, check_or_generate_domain, compute_content_hash,
     get_content_type, get_file_info, make_empty_hash, update_env_file,
};
use crate::Db;
use aqua_verifier_rs_types::models::base64::Base64;
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
use aqua_verifier_rs_types::models::content::RevisionContentContent;
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
use verifier::util::{
    content_hash, metadata_hash, signature_hash, verification_hash, witness_hash,
};
extern crate serde_json_path_to_error as serde_json;
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
use crate::db::pages_db::{db_data, insert_page_data, fetch_page_data, update_page_data, delete_page_data, delete_all_data};



// We parse the .env file directly
pub async fn explorer_fetch_configuration(
    State(server_database): State<Db>,
) -> (StatusCode, Json<HashMap<String, String>>) {
    let mut config_data = HashMap::new();

    // Read the entire .env file and parse key-value pairs
    if let Ok(env_content) = fs::read_to_string(".env") {
        for line in env_content.lines() {
            if let Some((key, value)) = line.split_once('=') {
                match key.trim() {
                    "API_DOMAIN" => {
                        config_data.insert("domain".to_string(), value.trim().to_string())
                    }
                    "CHAIN" => config_data.insert("chain".to_string(), value.trim().to_string()),
                    "FILE_MODE" => config_data.insert("mode".to_string(), value.trim().to_string()),
                    "CONTRACT_ADDRESS" => {
                        config_data.insert("contract".to_string(), value.trim().to_string())
                    }
                    _ => None,
                };
            }
        }
    }

    (StatusCode::OK, Json(config_data))
}

pub async fn explorer_update_configuration(
    State(server_database): State<Db>,
    Form(input): Form<UpdateConfigurationInput>,
) -> (StatusCode, Json<ApiResponse>) {
    let mut log_data: Vec<String> = Vec::new();

    // Log the input
    log_data.push(format!(
        "Updating configuration with chain: {} and domain: {} mode : {}",
        input.chain, input.domain, input.mode
    ));

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

    if let Err(e) = update_env_file("CONTRACT_ADDRESS", &input.contract) {
        log_data.push(format!(
            "Failed to update CONTRACT_ADDRESS in .env file: {}",
            e
        ));
    }

    // Prepare the response
    let res = ApiResponse {
        logs: log_data,
        file: None,
        files: Vec::new(),
    };

    (StatusCode::OK, Json(res))
}

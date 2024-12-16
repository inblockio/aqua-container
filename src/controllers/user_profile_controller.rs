use crate::db::user_profiles::{fetch_user_profile, update_user_profile};
use crate::models::input::{DeleteInput, RevisionInput, UpdateConfigurationInput, WitnessInput};
use crate::models::page_data::{ApiResponse, PageDataContainer};
use crate::models::user_profiles::UserProfileApiResponse;
use crate::models::UserProfilesTable;
use crate::models::{file::FileInfo, page_data};
use crate::util::{
    check_if_page_data_revision_are_okay, check_or_generate_domain, compute_content_hash,
    get_content_type, get_file_info, make_empty_hash, update_env_file,
};
use crate::Db;
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
use aqua_verifier::util::{
    content_hash, metadata_hash, signature_hash, verification_hash, witness_hash,
};
extern crate serde_json_path_to_error as serde_json;
use crate::db::pages_db::{
    delete_all_data, delete_page_data, fetch_page_data, insert_page_data, update_page_data,
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

// We parse the .env file directly
pub async fn explorer_fetch_user_profile(
    State(server_database): State<Db>,
    headers: HeaderMap,
) -> (StatusCode, Json<UserProfileApiResponse>) {
    // let mut config_data = HashMap::new();

    let mut log_data: Vec<String> = Vec::new();
    let mut res: UserProfileApiResponse = UserProfileApiResponse {
        logs: log_data.clone(),
        user_profile: None,
    };

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

    let mut conn = match server_database.pool.get() {
        Ok(connection) => connection,
        Err(e) => {
            // error!("Failed to get database connection: {}", e);
            log_data.push("Failed to get database connection".to_string());
            println!("Error Fetching connection {:#?}", res);
            return (StatusCode::INTERNAL_SERVER_ERROR, Json(res));
        }
    };

    let user_profile = fetch_user_profile(metamask_address.to_owned(), &mut conn);
    println!("Fetched user profile: {:?}", user_profile);

    if user_profile.is_err() {
        return (StatusCode::NOT_FOUND, Json(res));
    }

    let _user_profile = user_profile.unwrap();

    res.user_profile = Some(_user_profile);
    return (StatusCode::OK, Json(res));

    (StatusCode::OK, Json(res))
}

pub async fn explorer_update_user_profile(
    State(server_database): State<Db>,
    Form(input): Form<UserProfilesTable>,
) -> (StatusCode, Json<UserProfileApiResponse>) {
    let mut log_data: Vec<String> = Vec::new();
    let mut res: UserProfileApiResponse = UserProfileApiResponse {
        logs: log_data.clone(),
        user_profile: Some(input.clone()),
    };

    let mut conn = match server_database.pool.get() {
        Ok(connection) => connection,
        Err(e) => {
            log_data.push("Failed data not found in database".to_string());

            log_data.push("Failed to get database connection".to_string());

            println!("Error Fetching connection {:#?}", res);
            return (StatusCode::INTERNAL_SERVER_ERROR, Json(res));
        }
    };

    // let insert_result = insert_page_data(db_data_model.clone(), & mut conn);
    // let page_data_result = fetch_page_data(input.filename, & mut conn);

    let update_result = update_user_profile(input.clone(), &mut conn);

    if update_result.is_err() {
        let e = update_result.err().unwrap();
        tracing::error!("Failed to update user profile: {:?}", e);
        log_data.push(format!("Failed to update user profile : {:?}", e));
        return (StatusCode::INTERNAL_SERVER_ERROR, Json(res));
    }

    (StatusCode::OK, Json(res))
}

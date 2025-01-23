use crate::db::settings::{get_setting, update_setting};
// use crate::db::user_profiles::{fetch_user_profile, update_user_profile};
use crate::models::database_models::SettingsTable;
use crate::models::input::{DeleteInput, RevisionInput, UpdateConfigurationInput, WitnessInput};
use crate::models::user_profiles::UseSettingsApiResponse;
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
use diesel::r2d2::ConnectionManager;
use diesel::r2d2::Pool;
use diesel::PgConnection;
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
    State(server_database): State<Pool<ConnectionManager<PgConnection>>>,
    headers: HeaderMap,
) -> (StatusCode, Json<UseSettingsApiResponse>) {
    // let mut config_data = HashMap::new();

    let mut log_data: Vec<String> = Vec::new();
    let mut res: UseSettingsApiResponse = UseSettingsApiResponse {
        logs: log_data.clone(),
        user_settings: None,
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

    let mut conn = match server_database.get() {
        Ok(connection) => connection,
        Err(e) => {
            // error!("Failed to get database connection: {}", e);
            log_data.push("Failed to get database connection".to_string());
            println!("Error Fetching connection {:#?}", res);
            return (StatusCode::INTERNAL_SERVER_ERROR, Json(res));
        }
    };

    let user_settings = get_setting(
        &mut conn,
        metamask_address
        );
    println!("Fetched user profile: {:?}", user_settings);

    if user_settings.is_err() {
        return (StatusCode::NOT_FOUND, Json(res));
    }

    let user_settings_data = user_settings.unwrap();

    res.user_settings = Some(user_settings_data);
    return (StatusCode::OK, Json(res));

}

pub async fn explorer_update_user_profile(
    State(server_database): State<Pool<ConnectionManager<PgConnection>>>,
    headers: HeaderMap,
    Form(input): Form<SettingsTable>,
 
) -> (StatusCode, Json<UseSettingsApiResponse>) {
    let mut log_data: Vec<String> = Vec::new();
    let mut res: UseSettingsApiResponse = UseSettingsApiResponse {
        logs: log_data.clone(),
        user_settings: None, //Some(input.clone()),
    };

    let mut conn = match server_database.get() {
        Ok(connection) => connection,
        Err(e) => {
            log_data.push("Failed data not found in database".to_string());

            log_data.push("Failed to get database connection".to_string());

            println!("Error Fetching connection {:#?}", res);
            return (StatusCode::INTERNAL_SERVER_ERROR, Json(res));
        }
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


    // let insert_result = insert_page_data(db_data_model.clone(), & mut conn);
    // let page_data_result = fetch_page_data(input.filename, & mut conn);

    let update_result = update_setting(
        &mut conn,
        metamask_address,
        input.clone(), //SettingsTable { cli_pub_key: "cli_pub_key".to_string(), cli_priv_key: "cli_priv_key".to_string(), witness_network: "witness_network".to_string(), witness_contract_address: "witness_contract_address".to_string(), theme: "theme".to_string() },
    );

    if update_result.is_err() {
        let e = update_result.err().unwrap();
        tracing::error!("Failed to update user profile: {:?}", e);
        log_data.push(format!("Failed to update user profile : {:?}", e));
        return (StatusCode::INTERNAL_SERVER_ERROR, Json(res));
    }

    (StatusCode::OK, Json(res))
}

#![allow(warnings)]

mod models;
mod util;
mod controllers;
pub mod auth;
// pub mod verification;
// pub mod revision_integrity;

use aqua_verifier_rs_types::models::page_data::HashChain;
use auth::{fetch_nonce_session, siwe_sign_in};
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
use bonsaidb::core::{keyvalue::{KeyStatus, KeyValue}};
use bonsaidb::core::schema::{Collection, SerializedCollection};
use bonsaidb::local::config::{Builder, StorageConfiguration};
use bonsaidb::local::Database;
use serde::{Deserialize, Serialize};
use sqlx::{sqlite::SqlitePoolOptions, Pool, Sqlite};
extern crate serde_json_path_to_error as serde_json;

use crate::util::{check_or_generate_domain, db_set_up};
use controllers::form_controller::{show_form, save_request_body, save_json_file, get_verification_hash_for_file, add_signature_hash_for_file };
use  controllers::api_controller::{fetch_explorer_files, explorer_file_upload, explorer_aqua_file_upload, explorer_sign_revision, explorer_witness_file, explorer_delete_file,  explorer_delete_all_files, explorer_fetch_configuration, explorer_update_configuration};
use crate::controllers::api_controller::explorer_file_verify_hash_upload;

const UPLOADS_DIRECTORY: &str = "uploads";



#[derive(Debug, Clone)]
pub struct Db {
    pub db: Database,
    pub sqliteDb : Pool<Sqlite>

}

/// How to get started
/// This is a axum project , wit uses two databases
///     a. sqlite
///     b. bonsaidb
///
/// The endpoints and controllers can be grouped into 2
///     a. endpoint and controller for a single form.html that's server side rendered(ssr)
///     b. endpoint and controller for ( solid js located in web folder)  front end
///
///  The ssr page is a minimalistic example , showing generation of aqua chain json file, witnessing and validation  of the qua chain file
///  The solid js project shows  broader example of how the aqua protocol can be utilised it also  enables generation of aqua chain json file, witnessing and validation  of the qua chain file
///
///

#[tokio::main]
async fn main() {
    tracing_subscriber::fmt()
        .with_max_level(tracing::Level::DEBUG)
        .init();

    dotenv::dotenv().ok();

    check_or_generate_domain();

    let sqliteDb =  db_set_up().await;
    let db = Database::open::<crate::models::page_data::PageDataContainer<HashChain>>(StorageConfiguration::new("b0nsa1.bonsaidb")).unwrap();

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
        .route("/explorer_aqua_file_upload", post(explorer_aqua_file_upload))
        .route("/explorer_verify_hash", post(explorer_file_verify_hash_upload))
        .route("/explorer_sign_revision", post(explorer_sign_revision))
        .route("/explorer_witness_file", post(explorer_witness_file))
        .route("/explorer_delete_file", post(explorer_delete_file))
        .route("/explorer_delete_all_files", get(explorer_delete_all_files))
        .route("/explorer_fetch_configuration", get(explorer_fetch_configuration))
        .route("/explorer_update_configuration", post(explorer_update_configuration))
        .route("/siwe", post(siwe_sign_in))
        .route("/fetch_nonce_session", post(fetch_nonce_session))
        
        //.route("/list", get(show_files_list).post(show_files))
        .with_state(server_database)
        .layer(CorsLayer::permissive())
        .layer(TraceLayer::new_for_http())
        .layer(DefaultBodyLimit::max(20 * 1024 * 1024));;


    let listener = tokio::net::TcpListener::bind("127.0.0.1:3600")
        .await
        .unwrap();
    tracing::debug!("listening on {}", listener.local_addr().unwrap());
    axum::serve(listener, app).await.unwrap();
}


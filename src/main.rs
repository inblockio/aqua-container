#![allow(warnings)]

pub mod auth;
mod controllers;
mod db;
mod models;
mod util;
// pub mod verification;
// pub mod revision_integrity;

use aqua_verifier_rs_types::models::page_data::HashChain;
use auth::{fetch_nonce_session, siwe_sign_in};
use axum::{
    body::Bytes,
    extract::{DefaultBodyLimit, Multipart, Path, Request, State},
    handler::HandlerWithoutStateExt,
    http::StatusCode,
    response::{Html, Redirect},
    routing::{get, post},
    BoxError, Form, Json, Router,
};
use diesel::SqliteConnection;
use diesel_migrations::embed_migrations;
use ethaddr::address;

use axum::response::{IntoResponse, Response};
use chrono::{DateTime, NaiveDateTime, Utc};
use futures::{Stream, TryStreamExt};
use models::file::FileInfo;
use serde_json::json;
use sha3::*;
use util::run_db_migrations;
use std::collections::BTreeMap;
use std::fs;
use std::io;
use std::net::SocketAddr;
use std::time::SystemTime;
use tokio::{fs::File, io::BufWriter};
use tokio_util::io::StreamReader;
use tower::ServiceExt;
use tower_http::cors::{Any, CorsLayer};
use tower_http::{
    limit::RequestBodyLimitLayer,
    services::{ServeDir, ServeFile},
    trace::TraceLayer,
};
use tracing_subscriber::{layer::SubscriberExt, util::SubscriberInitExt};
use crate::models::DB_POOL;
use diesel::r2d2::{self, ConnectionManager};
use serde::{Deserialize, Serialize};
extern crate serde_json_path_to_error as serde_json;
use std::sync::{mpsc, Mutex, MutexGuard};
use crate::controllers::api_controller::explorer_file_verify_hash_upload;
use crate::util::{check_or_generate_domain, establish_connection};
use controllers::api_controller::{
    explorer_aqua_file_upload, explorer_delete_all_files, explorer_delete_file,
    explorer_fetch_configuration, explorer_file_upload, explorer_sign_revision,
    explorer_update_configuration, explorer_witness_file, fetch_explorer_files,
};

const UPLOADS_DIRECTORY: &str = "uploads";



#[derive(Clone)]
pub struct Db {
    pub pool:DB_POOL,
}

// Handler function that returns a JSON response
async fn status_handler() -> Json<serde_json::Value> {
    Json(json!({ "status": "okay" }))
}

/// How to get started
/// This is a axum project  with a react front end in web
///
///  The react js project shows   example of how the aqua protocol can be utilised it
///  enables generation of aqua chain json file, witnessing and validation  of the qua chain file
#[tokio::main]
async fn main() {
    tracing_subscriber::fmt()
        .with_max_level(tracing::Level::DEBUG)
        .init();

    dotenv::dotenv().ok();

    check_or_generate_domain();

    
    // Establish database connection pool
    let pool:  r2d2::Pool<ConnectionManager<SqliteConnection>> = crate::util::establish_connection();

    // Run migrations
    // Get a connection from the pool to pass to run_db_migrations
    let mut conn = pool.get().expect("Failed to get database connection");
    if let Err(e) = run_db_migrations(&mut conn) {
        eprintln!("Failed to run migrations: {}", e);
        return;
    }
    // save files to a separate directory to not override files in the current directory
    tokio::fs::create_dir(UPLOADS_DIRECTORY).await;

    let server_database = Db { pool  };

    let app = Router::new()
        .route("/", get(status_handler).post(status_handler))
        .route("/explorer_files", get(fetch_explorer_files))
        .route("/explorer_file_upload", post(explorer_file_upload))
        .route(
            "/explorer_aqua_file_upload",
            post(explorer_aqua_file_upload),
        )
        .route(
            "/explorer_verify_hash",
            post(explorer_file_verify_hash_upload),
        )
        .route("/explorer_sign_revision", post(explorer_sign_revision))
        .route("/explorer_witness_file", post(explorer_witness_file))
        .route("/explorer_delete_file", post(explorer_delete_file))
        .route("/explorer_delete_all_files", get(explorer_delete_all_files))
        .route(
            "/explorer_fetch_configuration",
            get(explorer_fetch_configuration),
        )
        .route(
            "/explorer_update_configuration",
            post(explorer_update_configuration),
        )
        .route("/siwe", post(siwe_sign_in))
        .route("/fetch_nonce_session", post(fetch_nonce_session))
        //.route("/list", get(show_files_list).post(show_files))
        .with_state(server_database)
        .layer(CorsLayer::permissive())
        .layer(TraceLayer::new_for_http())
        .layer(DefaultBodyLimit::max(20 * 1024 * 1024));

    let listener = tokio::net::TcpListener::bind("127.0.0.1:3600")
        .await
        .unwrap();
    tracing::debug!("listening on {}", listener.local_addr().unwrap());
    axum::serve(listener, app).await.unwrap();
}

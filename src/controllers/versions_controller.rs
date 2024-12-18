use std::env;

use axum::{
    body::Bytes,
    extract::{DefaultBodyLimit, Multipart, Path, Request, State},
    handler::HandlerWithoutStateExt,
    http::StatusCode,
    response::{Html, Redirect},
    routing::{get, post},
    BoxError, Form, Json, Router,
};
use serde_json::json;

// Handler function that returns a JSON response
pub async fn version_details() -> Json<serde_json::Value> {
    let mut frontend = env::var("FRONTEND_VERSION").unwrap_or_default();
    let mut backend = env::var("BACKEND_VERSION").unwrap_or_default();
    let mut aquifier = env::var("AQUIFIER_VERSION").unwrap_or_default();
    let mut protocol = env::var("PROTOCOL_VERSION").unwrap_or_default();

    if frontend.is_empty() {
        frontend = "1.2.0".to_string()
    }

    if backend.is_empty() {
        backend = "1.2.0".to_string()
    }
    if aquifier.is_empty() {
        aquifier = "1.2.0".to_string()
    }
    if protocol.is_empty() {
        protocol = "1.2.0".to_string()
    }

    Json(json!({
        "backend" : backend,
        "frontend" : frontend,
        "aquifier" : aquifier,
        "protocol" : protocol,
    }))
}

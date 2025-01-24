use crate::models::file::FileDataInformation;
use base64::{engine::general_purpose::STANDARD, Engine as _};
use chrono::{DateTime, TimeZone, Utc};
use diesel::prelude::*;
use ethers::core::k256::SecretKey;
use ethers::prelude::*;
use rand::distributions::Alphanumeric;
use rand::{thread_rng, Rng};
use serde_json::Value;
use sha3::{Digest, Sha3_512};
use siwe::TimeStamp;
use time::OffsetDateTime;
use std::collections::BTreeMap;
use std::collections::HashMap;
use std::io::Write;
use std::path::Path;
use std::{env, fs};
// use diesel::sqlite::SqliteConnection;
use diesel::Connection;
use diesel_migrations::{embed_migrations, EmbeddedMigrations, MigrationHarness};



pub fn get_file_info(base64_string: String) -> Result<FileDataInformation, String> {
    // First, decode the base64 string
    let bytes = STANDARD
        .decode(base64_string)
        .map_err(|e| format!("Failed to decode base64: {}", e))?;

    // Get file size
    let size_bytes = bytes.len();

    // Define file signatures using a Vec instead of fixed-size arrays
    let file_signatures: HashMap<Vec<u8>, (String, String)> = [
        // Image formats
        (
            vec![0xFF, 0xD8, 0xFF],
            ("JPEG".to_string(), "image/jpeg".to_string()),
        ),
        (
            vec![0x89, 0x50, 0x4E, 0x47],
            ("PNG".to_string(), "image/png".to_string()),
        ),
        (
            vec![0x47, 0x49, 0x46],
            ("GIF".to_string(), "image/gif".to_string()),
        ),
        (
            vec![0x3C, 0x73, 0x76, 0x67],
            ("SVG".to_string(), "image/svg+xml".to_string()),
        ),
        // Document formats
        (
            vec![0x25, 0x50, 0x44, 0x46],
            ("PDF".to_string(), "application/pdf".to_string()),
        ),
        (
            vec![0x50, 0x4B, 0x03, 0x04],
            ("ZIP".to_string(), "application/zip".to_string()),
        ),
        (
            vec![0xD0, 0xCF, 0x11, 0xE0, 0xA1, 0xB1, 0x1A, 0xE1],
            ("DOC".to_string(), "application/msword".to_string()),
        ), // Older DOC files
        (
            vec![0x50, 0x4B, 0x03, 0x04],
            (
                "DOCX".to_string(),
                "application/vnd.openxmlformats-officedocument.wordprocessingml.document"
                    .to_string(),
            ),
        ), // DOCX files
        // Audio formats
        (
            vec![0x49, 0x44, 0x33],
            ("MP3".to_string(), "audio/mpeg".to_string()),
        ), // MP3
        (
            vec![0x52, 0x49, 0x46, 0x46],
            ("WAV".to_string(), "audio/wav".to_string()),
        ), // WAV files
        // Video formats
        (
            vec![
                0x00, 0x00, 0x00, 0x18, 0x66, 0x74, 0x79, 0x70, 0x6D, 0x70, 0x34, 0x32,
            ],
            ("MP4".to_string(), "video/mp4".to_string()),
        ), // MP4
        (
            vec![0x1A, 0x45, 0xDF, 0xA3],
            ("MKV".to_string(), "video/x-matroska".to_string()),
        ), // MKV format
        // JSON and XML for other document-like formats
        (
            vec![0x7B],
            ("JSON".to_string(), "application/json".to_string()),
        ),
        (
            vec![0x3C, 0x3F, 0x78, 0x6D, 0x6C],
            ("XML".to_string(), "application/xml".to_string()),
        ),
    ]
    .into_iter()
    .collect();

    // Detect file type based on magic numbers
    let (file_type, mime_type) = detect_file_type(&bytes, &file_signatures)?;

    Ok(FileDataInformation {
        file_type: file_type.to_string(),
        size_bytes,
        mime_type: mime_type.to_string(),
    })
}

fn detect_file_type(
    bytes: &[u8],
    signatures: &HashMap<Vec<u8>, (String, String)>,
) -> Result<(String, String), String> {
    // Check if we have enough bytes to check signatures
    if bytes.is_empty() {
        return Err("Empty file content".to_string());
    }

    // Try to match file signatures
    for (signature, (file_type, mime_type)) in signatures {
        if bytes.len() >= signature.len() {
            if bytes.starts_with(signature) {
                return Ok((file_type.clone(), mime_type.clone()));
            }
        }
    }

    // Special case for JSON: check if it's valid JSON
    if let Ok(text) = std::str::from_utf8(bytes) {
        if text.trim_start().starts_with('{') || text.trim_start().starts_with('[') {
            if serde_json::from_str::<serde_json::Value>(text).is_ok() {
                return Ok(("JSON".to_string(), "application/json".to_string()));
            }
        }
    }

    // If no known signature is found, try to detect if it's text
    if is_probably_text(bytes) {
        Ok(("TEXT".to_string(), "text/plain".to_string()))
    } else {
        Ok(("BINARY".to_string(), "application/octet-stream".to_string()))
    }
}

fn is_probably_text(bytes: &[u8]) -> bool {
    // Check if the content appears to be text by looking for common text characteristics
    let text_chars = bytes
        .iter()
        .filter(|&&byte| {
            byte >= 32 && byte <= 126 || byte == b'\n' || byte == b'\r' || byte == b'\t'
        })
        .count();

    // If more than 90% of the bytes are printable ASCII characters, it's probably text
    (text_chars as f64 / bytes.len() as f64) > 0.9
}

pub fn get_content_type(file_name: &str) -> Option<String> {
    // Define a mapping of file extensions to MIME types
    let mut mime_types: HashMap<&str, &str> = HashMap::new();

    // Populate the HashMap with file extensions and their corresponding MIME types
    mime_types.insert("jpg", "image/jpeg");
    mime_types.insert("jpeg", "image/jpeg");
    mime_types.insert("png", "image/png");
    mime_types.insert("gif", "image/gif");
    mime_types.insert("svg", "image/svg+xml");
    mime_types.insert("pdf", "application/pdf");
    mime_types.insert("zip", "application/zip");
    mime_types.insert("mp3", "audio/mpeg");
    mime_types.insert("wav", "audio/wav");
    mime_types.insert("mp4", "video/mp4");
    mime_types.insert("mkv", "video/x-matroska");
    mime_types.insert("json", "application/json");
    mime_types.insert("xml", "application/xml");
    mime_types.insert("txt", "text/plain");

    // Use the Path to check for file extension
    let path = Path::new(file_name);

    // Get the file extension if it exists
    if let Some(extension) = path.extension() {
        // Convert the extension to a string and check the mapping
        if let Some(extension_str) = extension.to_str() {
            return mime_types.get(extension_str).map(|&mime| mime.to_string());
        }
    }

    // Return None if the file has no extension or the extension is not recognized
    None
}

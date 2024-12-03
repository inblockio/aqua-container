use aqua_verifier_rs_types::models::revision::Revision;
use aqua_verifier_rs_types::models::hash::Hash;
use aqua_verifier_rs_types::models::content::RevisionContent;
use diesel::r2d2::ConnectionManager;
use ethers::core::k256::SecretKey;
use ethers::prelude::*;
// use guardian_common::custom_types::{Hash, Revision, RevisionContent};
// use guardian_common::prelude::Base64;
use rand::distributions::Alphanumeric;
use rand::{thread_rng, Rng};
use serde_json::Value;
use sha3::{Digest, Sha3_512};
use verifier::util::content_hash;
use std::collections::BTreeMap;
use std::io::Write;
use std::path::Path;
use std::{env, fs};
use base64::{Engine as _, engine::general_purpose::STANDARD};
use std::collections::HashMap;

use crate::models::file::FileDataInformation;
use diesel::{r2d2, Connection};
use diesel::prelude::*;
use diesel::sqlite::SqliteConnection;
use diesel_migrations::{embed_migrations, EmbeddedMigrations, MigrationHarness};


const MIGRATIONS: EmbeddedMigrations = embed_migrations!();
type DB = diesel::sqlite::Sqlite;

pub fn run_db_migrations(conn: &mut impl MigrationHarness<DB>) -> Result<(), Box<dyn std::error::Error + Send + Sync + 'static>> {
    conn.run_pending_migrations(MIGRATIONS)?;
    Ok(())
}

pub fn establish_connection() ->  r2d2::Pool<ConnectionManager<SqliteConnection>> {
    let database_url = std::env::var("DATABASE_URL")
        .expect("DATABASE_URL must be set");
    
        println!("Database url {}",database_url );
    let manager = ConnectionManager::<SqliteConnection>::new(database_url);
    
    r2d2::Pool::builder()
        .build(manager)
        .expect("Failed to create database pool")
}



// pub fn run_migrations(connection: &mut impl MigrationHarness<diesel::sqlite::Sqlite>) ->  {

//     let MIGRATIONS: EmbeddedMigrations = embed_migrations!("../migrations");

//     // This will run the necessary migrations.
//     //
//     // See the documentation for `MigrationHarness` for
//     // all available methods.
//     connection.run_pending_migrations(MIGRATIONS)?;

  
// }


pub fn check_or_generate_domain() {
    // Check if API_DOMAIN is set
    let api_domain = env::var("API_DOMAIN").unwrap_or_default();
    let chain = env::var("CHAIN").unwrap_or_default();

    if chain.is_empty() {
        // Update the .env file with the new API_DOMAIN
        if let Err(e) = update_env_file("CHAIN", "sepolia") {
            println!("Failed to update .env file: {}", e);
        }
    }else {
        println!("chain is set: {}", chain);
    }
    if api_domain.is_empty() {
        println!("API_DOMAIN is empty, generating a random one...");

        // Generate a random alphanumeric string
        let random_domain: String = thread_rng()
            .sample_iter(&Alphanumeric)
            .take(10)
            .map(char::from)
            .collect();

       
        println!("Generated API_DOMAIN: {:?}", random_domain);

        // Update the .env file with the new API_DOMAIN
        if let Err(e) = update_env_file("API_DOMAIN", &random_domain) {
            eprintln!("Failed to update .env file: {}", e);
        }
    } else {
        println!("API_DOMAIN is set: {}", api_domain);
    }


    println!("REMOTE is set: {}", env::var("VITE_REMOTE").unwrap_or_default());
}

// Function to update the .env file with the new API_DOMAIN
pub fn update_env_file(key: &str, value: &str) -> std::io::Result<()> {
    // Read the existing .env file contents
    let mut env_content = std::fs::read_to_string(".env").unwrap_or_default();

    // Check if the key already exists in the file
    if env_content.contains(key) {
        // Update the existing key-value pair
        let new_content = env_content
            .lines()
            .map(|line| {
                if line.starts_with(key) {
                    format!("{}={}", key, value)
                } else {
                    line.to_string()
                }
            })
            .collect::<Vec<_>>()
            .join("\n");

        // Write the updated content back to the .env file
        std::fs::write(".env", new_content)?;
    } else {
        // If the key doesn't exist, append it to the file
        let mut file = std::fs::OpenOptions::new().append(true).open(".env")?;
        writeln!(file, "{}={}", key, value)?;
    }
    Ok(())
}

pub fn check_if_page_data_revision_are_okay(revisions: Vec<(Hash, Revision)>) -> (bool, String) {
    let mut is_valid = (true, "".to_string());
    let has_valid_genessis = revsions_has_valid_genesis(revisions.clone());
    tracing::debug!("revsions_has_valid_genesis {:#?}", has_valid_genessis);

    if has_valid_genessis.is_none() {
        return (false, "revisions do not contain a valid genesis".to_string());
    }

    // check if the revision > metadata > previous_verification_hash is among the hash in revsions par
    // if more that one is none return false
    // there is a broken revision chain
    let mut all_hashes: Vec<Hash> = Vec::new();
    revisions
        .iter()
        .for_each(|(hash, revision)| all_hashes.push(hash.clone()));

    let genesis_hash_str = format!("{:#?}", has_valid_genessis.unwrap());

    for (index, (current_hash, current_revision)) in revisions.iter().enumerate() {
        let current_hash_str = format!("{:#?}", current_hash);

        // check hash if match the newly generated one
        let recomputed_content_hash = compute_content_hash(&current_revision.content);

        match recomputed_content_hash {
            Ok(data) => {
                if data == *current_hash {
                    tracing::error!("hashes match the generetaed one continue ...");
                }else{
                    tracing::error!("\n hashes do not match revision has {:#?} \n vs generated hash {:#?} \n",data,current_hash );
                    is_valid = (false, format!("a hash is not valid : {:#?}",  current_hash));

                    break;
                }
            }
            Err(error) => {
                tracing::error!("an error occured {}", error);
                is_valid = (false, "error generating a hash ".to_string());
                break;
            }
        }
        // let contnent_hash_str = format!("{:#?}", revision.content.content_hash);
        // let data_str = format!("{:#?}", revision.content.content_hash);
        // tracing::error!("returd conetnet is   {} \n  my json content hash is {} \n", data_str, contnent_hash_str);
        // matches = data ==revision.content.content_hash  ;//revision.content.content_hash;

        // chec if the hash chain is valid (ie if there any orphan revisions)
        if current_hash_str == genesis_hash_str {
            tracing::debug!("ignoring genessis hash is {:#?}", genesis_hash_str);
        } else {
            let contains = all_hashes.contains(current_hash);

            if contains == false {
                tracing::debug!("cannot find hash is {:#?}", current_hash_str);
                is_valid = (false, "Hash chain is invalid ".to_string());;
                break;
            }
        }
    }

    return is_valid;
}
pub fn revsions_has_valid_genesis(revisions: Vec<(Hash, Revision)>) -> Option<Hash> {
    // let mut is_valid= true;

    if revisions.len() <= 1 {
        tracing::debug!("The lengthe is equal to or ess than 1 {}", revisions.len());
        return None;
    }

    let mut revision_genesis: Vec<&Revision> = Vec::new();

    for (index, (hash, revision)) in revisions.iter().enumerate() {
        match revision.metadata.previous_verification_hash {
            Some(data) => {
                tracing::debug!("The previous hash is {:#?}", data);
            }
            None => {
                tracing::debug!("pushing revision to vector {:#?}", revision);
                revision_genesis.push(revision)
            }
        }
    }

    if revision_genesis.len() > 1 {
        tracing::debug!(
            "The genesis revision  length {} are {:#?}",
            revision_genesis.len(),
            revision_genesis
        );
        return None;
    }

    let res = revision_genesis.first();
    if res.is_none() {
        tracing::debug!("No genesis hash  (vec is empty)",);
        return None;
    }

    tracing::debug!("************************ {:#?}", res);
    // we use unwrapp becasue we are guaranteed the res has value due to the if check above
    return Some(res.unwrap().metadata.verification_hash);
}

pub fn compute_content_hash(contentPar: &RevisionContent) -> Result<Hash, String> {
    let b64 = contentPar.file.clone().unwrap().data; 
    
    let mut file_hasher = sha3::Sha3_512::default();
    file_hasher.update(b64.clone());
    let file_hash_current = Hash::from(file_hasher.finalize());

    let mut content_current = BTreeMap::new();

    content_current.insert("file_hash".to_owned(), file_hash_current.to_string());

    // println!("{:#?}", content_current);
    tracing::debug!("{:#?}", content_current);

    let content_hash_current = content_hash(&content_current.clone());

    tracing::debug!("{:#?}", content_hash_current);

    Ok(content_hash_current)
}

// pub fn content_hash()


pub fn make_empty_hash() -> Hash {
    let mut hasher = sha3::Sha3_512::default();
    hasher.update("");
    let empty_hash = Hash::from(hasher.finalize());
    empty_hash
}


pub fn get_file_info(base64_string: String) -> Result<FileDataInformation, String> {
    // First, decode the base64 string
    let bytes = STANDARD.decode(base64_string)
        .map_err(|e| format!("Failed to decode base64: {}", e))?;

    // Get file size
    let size_bytes = bytes.len();

    // Define file signatures using a Vec instead of fixed-size arrays
    let file_signatures: HashMap<Vec<u8>, (String, String)> = [
      // Image formats
      (vec![0xFF, 0xD8, 0xFF], ("JPEG".to_string(), "image/jpeg".to_string())),
      (vec![0x89, 0x50, 0x4E, 0x47], ("PNG".to_string(), "image/png".to_string())),
      (vec![0x47, 0x49, 0x46], ("GIF".to_string(), "image/gif".to_string())),
      (vec![0x3C, 0x73, 0x76, 0x67], ("SVG".to_string(), "image/svg+xml".to_string())),
      
      // Document formats
      (vec![0x25, 0x50, 0x44, 0x46], ("PDF".to_string(), "application/pdf".to_string())),
      (vec![0x50, 0x4B, 0x03, 0x04], ("ZIP".to_string(), "application/zip".to_string())),
      (vec![0xD0, 0xCF, 0x11, 0xE0, 0xA1, 0xB1, 0x1A, 0xE1], ("DOC".to_string(), "application/msword".to_string())), // Older DOC files
      (vec![0x50, 0x4B, 0x03, 0x04], ("DOCX".to_string(), "application/vnd.openxmlformats-officedocument.wordprocessingml.document".to_string())), // DOCX files
      
      // Audio formats
      (vec![0x49, 0x44, 0x33], ("MP3".to_string(), "audio/mpeg".to_string())), // MP3
      (vec![0x52, 0x49, 0x46, 0x46], ("WAV".to_string(), "audio/wav".to_string())), // WAV files
      
      // Video formats
      (vec![0x00, 0x00, 0x00, 0x18, 0x66, 0x74, 0x79, 0x70, 0x6D, 0x70, 0x34, 0x32], ("MP4".to_string(), "video/mp4".to_string())), // MP4
      (vec![0x1A, 0x45, 0xDF, 0xA3], ("MKV".to_string(), "video/x-matroska".to_string())), // MKV format
      
      // JSON and XML for other document-like formats
      (vec![0x7B], ("JSON".to_string(), "application/json".to_string())),
      (vec![0x3C, 0x3F, 0x78, 0x6D, 0x6C], ("XML".to_string(), "application/xml".to_string())),
    ].into_iter().collect();

    // Detect file type based on magic numbers
    let (file_type, mime_type) = detect_file_type(&bytes, &file_signatures)?;

    Ok(FileDataInformation{
        file_type: file_type.to_string(),
        size_bytes,
        mime_type: mime_type.to_string(),
    })
}

fn detect_file_type(bytes: &[u8], signatures: &HashMap<Vec<u8>, (String, String)>) -> Result<(String, String), String> {
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
    let text_chars = bytes.iter()
        .filter(|&&byte| byte >= 32 && byte <= 126 || byte == b'\n' || byte == b'\r' || byte == b'\t')
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
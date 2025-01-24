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

const MIGRATIONS: EmbeddedMigrations = embed_migrations!();
// type DB = diesel::sqlite::Sqlite;
use diesel::pg::PgConnection; // Import PgConnection
use diesel::r2d2::{ConnectionManager, Pool};

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

pub fn vec_to_string(vec: Vec<i32>) -> String {
    vec.iter()
        .map(|num| num.to_string())
        .collect::<Vec<String>>()
        .join(",")
}

pub fn string_to_vec(s: &str) -> Result<Vec<i32>, String> {
    s.split(',')
        .map(|num_str| num_str.trim().parse::<i32>().map_err(|e| e.to_string()))
        .collect()
}

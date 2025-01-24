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


pub fn timestamp_to_datetime_utc(timestamp: &TimeStamp) -> DateTime<Utc> {
    // Access the inner OffsetDateTime
    let offset_date_time: &OffsetDateTime = timestamp.as_ref();

    // Get the Unix timestamp in seconds
    let unix_seconds = offset_date_time.unix_timestamp();

    // Convert Unix timestamp to DateTime<Utc>
    Utc.timestamp_opt(unix_seconds, 0).unwrap()
}
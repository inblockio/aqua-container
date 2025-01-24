use crate::models::file::FileDataInformation;
use base64::{engine::general_purpose::STANDARD, Engine as _};
use chrono::{DateTime, TimeZone, Utc};
use diesel::prelude::*;
use diesel::r2d2::{ConnectionManager, Pool};
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


/// Establishes a connection pool for PostgreSQL.
///
/// # Returns
/// Returns an `r2d2::Pool<ConnectionManager<PgConnection>>` for managing database connections.
///
/// # Panics
/// Panics if the `DATABASE_URL` environment variable is not set or if the connection pool cannot be created.
pub fn establish_connection() -> Pool<ConnectionManager<PgConnection>> {
    // Load the DATABASE_URL from environment variables
    let database_url = std::env::var("DATABASE_URL").expect("DATABASE_URL must be set");

    println!("Database URL: {}", database_url);

    // Create a connection manager for PostgreSQL
    let manager = ConnectionManager::<PgConnection>::new(database_url);

    // Create a connection pool
    diesel::r2d2::Pool::builder()
        .build(manager)
        .expect("Failed to create database pool")
}




// pub fn run_db_migrations(
//     conn: &mut impl MigrationHarness<PgConnection>,
// ) -> Result<(), Box<dyn std::error::Error + Send + Sync + 'static>> {
//     conn.run_pending_migrations(MIGRATIONS)?;
//     Ok(())
// }




// pub fn run_migrations(connection: &mut impl MigrationHarness<diesel::sqlite::Sqlite>) ->  {

//     let MIGRATIONS: EmbeddedMigrations = embed_migrations!("../migrations");

//     // This will run the necessary migrations.
//     //
//     // See the documentation for `MigrationHarness` for
//     // all available methods.
//     connection.run_pending_migrations(MIGRATIONS)?;

// }


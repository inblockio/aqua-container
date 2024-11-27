
use diesel::r2d2::{self, ConnectionManager};
use diesel::SqliteConnection;
use serde::{Deserialize, Serialize};
use diesel::prelude::*;
pub mod file;
pub mod page_data;
pub mod input;


pub type DB_POOL = r2d2::Pool<ConnectionManager<SqliteConnection>>;

use diesel::prelude::*;
use chrono::NaiveDateTime;
use siwe::TimeStamp;

#[derive(Queryable, Selectable, Serialize, Deserialize, Debug, Clone)]
#[diesel(table_name = crate::schema::pages)]
pub struct PagesTable {
    pub id: Option<i32>,
    pub name: String,
    pub extension: String,
    pub page_data: String,
    pub owner: String,
    pub mode: String,
    pub created_at: Option<chrono::NaiveDateTime>,
}

#[derive(Queryable, Selectable, Deserialize, Serialize, Debug, Clone, Insertable)]
#[diesel(table_name = crate::schema::siwe_sessions)]
#[diesel(check_for_backend(diesel::sqlite::Sqlite))]
pub struct SiweSessionsTable {
    pub id: Option<i32>,
    pub address: String,
    pub nonce: String,
    pub issued_at: String,
    pub expiration_time: Option<String>,
}
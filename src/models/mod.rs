
use diesel::r2d2::{self, ConnectionManager};
use diesel::SqliteConnection;
use serde::{Deserialize, Serialize};
use diesel::prelude::*;
pub mod file;
pub mod page_data;
pub mod input;
pub mod user_profiles;
pub mod share_data;
use chrono::{DateTime, Utc};

pub type DB_POOL = r2d2::Pool<ConnectionManager<SqliteConnection>>;

use diesel::prelude::*;
use chrono::NaiveDateTime;
use siwe::TimeStamp;
use diesel::prelude::*;
use diesel::expression::AsExpression;
use diesel::sql_types::Nullable;

#[derive(Queryable, Selectable, Serialize, Deserialize, Debug, Clone, Insertable)]
#[diesel(table_name = crate::schema::pages)]
pub struct PagesTable {
    pub id: Option<i32>,
    pub name: String,
    pub extension: String,
    pub page_data: String,
    pub owner: String,
    pub mode: String,
    pub created_at: String,
    pub is_shared : bool
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct PagesDataTable {
    pub id: i64,
    pub name: String,
    pub extension: String,
    pub page_data: String,
    pub mode: String,
    pub owner: String,
    pub is_shared:  bool
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


#[derive(Queryable, Selectable, Deserialize, Serialize, Debug, Clone, Insertable)]
#[diesel(table_name = crate::schema::share_data)]
#[diesel(check_for_backend(diesel::sqlite::Sqlite))]
pub struct ShareDataTable {
    pub id: Option<i32>,
    pub file_id: i32,
    pub identifier: String,
    pub created_time: String,
}

 
#[derive(Queryable, Selectable, Deserialize, Serialize, Debug, Clone, Insertable)]
#[diesel(table_name = crate::schema::user_profiles)]
#[diesel(check_for_backend(diesel::sqlite::Sqlite))]
pub struct UserProfilesTable {
    pub id: Option<i32>,
    pub address: String,
    pub chain: String,
    pub theme: String,
    pub contract_address: String,
    pub file_mode: String,
    pub domain_name: String,
}
use aqua_verifier_rs_types::models::chain::AquaChain;
use aqua_verifier_rs_types::models::revision::Revision;
use diesel::prelude::*;
use diesel::r2d2::{self, ConnectionManager};
use diesel::SqliteConnection;
use serde::{Deserialize, Serialize};
pub mod api;
pub mod file;
pub mod input;
pub mod share_data;
pub mod user_profiles;
use chrono::{DateTime, Utc};

pub type DB_POOL = r2d2::Pool<ConnectionManager<SqliteConnection>>;
use crate::util::vec_to_string;
use chrono::NaiveDateTime;
use diesel::expression::AsExpression;
use diesel::prelude::*;
use diesel::prelude::*;
use diesel::sql_types::Nullable;
use siwe::TimeStamp;

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

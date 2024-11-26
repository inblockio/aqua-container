
use serde::{Deserialize, Serialize};
use crate::auth::SiweSession;
use crate::models::DB_POOL;

pub fn insert_siwe_data(data: SiweSession, db_connection : DB_POOL) -> Result<(i64), String> {
    return Ok(0);
}

pub fn fetch_siwe_data(db_connection : DB_POOL ) -> Result<SiweSession, String> {
    return Err("not implmented".to_string());
}
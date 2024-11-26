
use serde::{Deserialize, Serialize};
use crate::auth::SiweSession;

pub fn insert_siwe_data(data: SiweSession) -> Result<(i64), String> {
    return Ok(0);
}

pub fn fetch_siwe_data( ) -> Result<SiweSession, String> {
    return Err("not implmented".to_string());
}
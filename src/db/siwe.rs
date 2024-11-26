
use serde::{Deserialize, Serialize};
use crate::auth::SiweSession;
use crate::models::{DB_POOL, SiweSessionsTable};

pub fn insert_siwe_data(data: SiweSession, db_connection : DB_POOL) -> Result<(i64), String> {

    let user = diesel::insert_into(crate::schema::siwe_sessions::dsl::siwe_sessions)
        .values(&SiweSessionsTable {
             id: None,
     address: data.address,
     nonce: data.nonce,
     issued_at: data.issued_at,
     expiration_time:data.expiration_time,
        })
        .get_result::<SiweSessionsTable>(&mut db_connection)
        .expect("Error saving new siwe data");
    return Ok(0);
}

pub fn fetch_siwe_data(db_connection : DB_POOL ) -> Result<SiweSession, String> {

    return Err("not implmented".to_string());
}
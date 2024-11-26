use diesel::prelude::*; // This will import RunQueryDsl
use crate::auth::SiweSession;
use crate::models::{SiweSessionsTable, DB_POOL};
use serde::{Deserialize, Serialize};

pub fn insert_siwe_data(data: SiweSession,mut db_connection: DB_POOL) -> Result<(i64), String> {
    
    // Use returning instead of get_result
    let inserted_id = diesel::insert_into(crate::schema::siwe_sessions::dsl::siwe_sessions)
        .values(&SiweSessionsTable {
            id: None,
            address: data.address,
            nonce: data.nonce,
            issued_at: data.issued_at,
            expiration_time: data.expiration_time,
        })
        .returning(crate::schema::siwe_sessions::dsl::id)
        .first::<i32>(&mut db_connection)
        .map_err(|e| format!("Error saving new siwe data: {}", e))?;

    Ok(inserted_id as i64)

}

pub fn fetch_siwe_data(mut db_connection: DB_POOL) -> Result<Vec<SiweSessionsTable>, String> {
    use crate::schema::siwe_sessions::dsl::*;

    siwe_sessions
        .load::<SiweSessionsTable>(&mut db_connection)
        .map_err(|e| format!("Error fetching SIWE sessions: {}", e))
}

// Or to fetch a single record
pub fn fetch_siwe_data_by_address(address: String, mut db_connection: DB_POOL) -> Result<SiweSessionsTable, String> {
    use crate::schema::siwe_sessions::dsl::*;

    siwe_sessions
        .filter(siwe_sessions::address.eq(address))
        .first::<SiweSessionsTable>(&mut db_connection)
        .map_err(|e| format!("Error fetching SIWE session: {}", e))
}

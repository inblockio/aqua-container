use diesel::prelude::*;
use diesel::r2d2::{ConnectionManager, PooledConnection}; // This will import RunQueryDsl
use crate::auth::SiweSession;
use crate::models::{SiweSessionsTable, DB_POOL};
use serde::{Deserialize, Serialize};

pub fn insert_siwe_data(
    data: SiweSession, 
    db_connection: &mut PooledConnection<ConnectionManager<SqliteConnection>>
) -> Result<i64, String> {
    let record =  &SiweSessionsTable {
        id: None,
        address: data.address,
        nonce: data.nonce,
        issued_at: data.issued_at,
        expiration_time: data.expiration_time,
    };
    let inserted_id: i32 = diesel::insert_into(crate::schema::siwe_sessions::table)
        .values(record)
        .returning(crate::schema::siwe_sessions::dsl::id)
        .get_result::<Option<i32>>(db_connection)
        .map_err(|e| format!("Error saving new siwe data: {}", e))?
        .unwrap_or(-1);  // Provide a default value if None

    Ok(inserted_id as i64)
}
pub fn fetch_siwe_data(
    db_connection: &mut PooledConnection<ConnectionManager<SqliteConnection>>
) -> Result<Vec<SiweSessionsTable>, String> {
    use crate::schema::siwe_sessions::dsl::*;

    siwe_sessions
        .select(SiweSessionsTable::as_select())
        .load::<SiweSessionsTable>(db_connection)
        .map_err(|e| format!("Error fetching SIWE sessions: {}", e))
}

pub fn fetch_siwe_data_by_address(
    address_param: &str,
    db_connection: &mut PooledConnection<ConnectionManager<SqliteConnection>>
) -> Result<Vec<SiweSessionsTable>, String> {
    use crate::schema::siwe_sessions::dsl::*;

    siwe_sessions
        .filter(address.eq(address_param))
        .select(SiweSessionsTable::as_select())
        .load::<SiweSessionsTable>(db_connection)
        .map_err(|e| format!("Error fetching SIWE sessions for address: {}", e))
}
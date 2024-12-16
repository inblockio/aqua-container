use diesel::prelude::*;
use diesel::r2d2::{ConnectionManager, PooledConnection}; // This will import RunQueryDsl

use crate::models::{ShareDataTable, DB_POOL};
use serde::{Deserialize, Serialize};

pub fn insert_share_data(
    data: ShareDataTable, 
    db_connection: &mut PooledConnection<ConnectionManager<SqliteConnection>>
) -> Result<i64, String> {
  
    let inserted_id: i32 = diesel::insert_into(crate::schema::share_data::table)
        .values(data)
        .returning(crate::schema::share_data::dsl::id)
        .get_result::<Option<i32>>(db_connection)
        .map_err(|e| format!("Error saving new share data: {}", e))?
        .unwrap_or(-1);  // Provide a default value if None

    Ok(inserted_id as i64)
}

pub fn fetch_all_share_data(
    db_connection: &mut PooledConnection<ConnectionManager<SqliteConnection>>
) -> Result<Vec<ShareDataTable>, String> {
    use crate::schema::share_data::dsl::*;

    share_data
        .select(ShareDataTable::as_select())
        .load::<ShareDataTable>(db_connection)
        .map_err(|e| format!("Error fetching SIWE sessions: {}", e))
}

pub fn fetch_share_data_by_address(
    identifier_param: &str,
    db_connection: &mut PooledConnection<ConnectionManager<SqliteConnection>>
) -> Result<Vec<ShareDataTable>, String> {
    use crate::schema::share_data::dsl::*;

    share_data
        .filter(identifier.eq(identifier_param))
        .select(ShareDataTable::as_select())
        .load::<ShareDataTable>(db_connection)
        .map_err(|e| format!("Error fetching SIWE sessions for address: {}", e))
}

// pub fn fetch_share_session_by_nonce(
//     nonce_value: &str,
//     db_connection: &mut PooledConnection<ConnectionManager<SqliteConnection>>,
// ) -> Result<ShareDataTable, String> {
//     use crate::schema::siwe_sessions::dsl::*;

//     siwe_sessions
//         .filter(nonce.eq(nonce_value)) // Use the passed `nonce_value`
//         .first::<ShareDataTable>(db_connection) // Fetch the first matching result
//         .map_err(|e| format!("Error fetching SIWE session for nonce: {}", e)) // Map any errors
// }

pub fn delete_share_session_by_nonce(
    identifier_param: &str,
    db_connection: &mut PooledConnection<ConnectionManager<SqliteConnection>>,
) -> Result<usize, String> {
    use crate::schema::share_data::dsl::*;

    diesel::delete(share_data.filter(identifier.eq(identifier_param)))
        .execute(db_connection)
        .map_err(|e| format!("Error deleting share data : {}", e))
}
use aqua_verifier_rs_types::models::chain::AquaChain;
use aqua_verifier_rs_types::models::revision::Revision;
use diesel::{
    prelude::*,
    r2d2::{ConnectionManager, PooledConnection},
    result::Error as DieselError,
    SqliteConnection,
};
use chrono::{NaiveDateTime, Utc};
use serde::{Deserialize, Serialize};

pub fn insert_aqua_chain(
    aqua_chain: AquaChain,
    file_hash: String,
    file_name: String,
    file_content: String,
    owner: String,
    mode: String,
    db_connection: &mut PooledConnection<ConnectionManager<SqliteConnection>>,
) -> Result<i32, String> {
    use crate::schema::aqua_chain;

    let db_record = aqua_chain.to_db(file_hash, file_name, file_content, owner, mode);

    diesel::insert_into(aqua_chain::table)
        .values(&db_record)
        .execute(db_connection)
        .map_err(|e| format!("Error inserting AquaChain: {}", e))?;

    let inserted_id: i32 = aqua_chain::table
        .select(aqua_chain::id)
        .order(aqua_chain::id.desc())
        .first(db_connection)
        .map_err(|e| format!("Error fetching inserted ID: {}", e))?;

    Ok(inserted_id)
}

pub fn update_aqua_chain(
    chain_id: i32,
    updated_data: AquaChain,
    db_connection: &mut PooledConnection<ConnectionManager<SqliteConnection>>,
) -> Result<(), String> {
    use crate::schema::aqua_chain::dsl::*;

    let updated_record = updated_data.to_db(
        updated_data.file_index[0].0.clone(),
        updated_data.file_index[0].1.clone(),
        updated_data.revisions[0].0.clone(),
        updated_data.revisions[0].0.clone(),
        "default_mode".to_string(), // Replace as needed
    );

    diesel::update(aqua_chain.filter(id.eq(chain_id)))
        .set((
            file_hash.eq(updated_record.file_hash),
            file_name.eq(updated_record.file_name),
            revisions.eq(updated_record.revisions),
            file_content.eq(updated_record.file_content),
            owner.eq(updated_record.owner),
            mode.eq(updated_record.mode),
            is_shared.eq(updated_record.is_shared),
            updated_at.eq(Utc::now().naive_utc()),
        ))
        .execute(db_connection)
        .map_err(|e| format!("Error updating AquaChain: {}", e))?;

    Ok(())
}

pub fn delete_aqua_chain(
    chain_id: i32,
    db_connection: &mut PooledConnection<ConnectionManager<SqliteConnection>>,
) -> Result<(), String> {
    use crate::schema::aqua_chain::dsl::*;

    diesel::delete(aqua_chain.filter(id.eq(chain_id)))
        .execute(db_connection)
        .map_err(|e| format!("Error deleting AquaChain: {}", e))?;

    Ok(())
}

pub fn fetch_aqua_chain_by_owner(
    owner_name: String,
    db_connection: &mut PooledConnection<ConnectionManager<SqliteConnection>>,
) -> Result<Vec<AquaChainDb>, String> {
    use crate::schema::aqua_chain::dsl::*;

    let results = aqua_chain
        .filter(owner.eq(owner_name))
        .load::<AquaChainDb>(db_connection)
        .map_err(|e| format!("Error fetching AquaChains by owner: {}", e))?;

    Ok(results)
}
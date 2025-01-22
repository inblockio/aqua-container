use aqua_verifier_rs_types::models::revision::Revision;
use diesel::{
    prelude::*,
    r2d2::{ConnectionManager, PooledConnection},
    result::Error as DieselError,
    SqliteConnection,
};
use chrono::{NaiveDateTime, Utc};
use serde::{Deserialize, Serialize};

use crate::models::RevisionDb;

pub fn insert_revision(
    revision: RevisionDb,
    db_connection: &mut PooledConnection<ConnectionManager<SqliteConnection>>,
) -> Result<i32, String> {
    use crate::schema::revisions;

    diesel::insert_into(revisions::table)
        .values(&revision)
        .execute(db_connection)
        .map_err(|e| format!("Error inserting revision: {}", e))?;

    let inserted_id: i32 = revisions::table
        .select(revisions::id)
        .order(revisions::id.desc())
        .first(db_connection)
        .map_err(|e| format!("Error fetching inserted ID: {}", e))?;

    Ok(inserted_id)
}

pub fn fetch_revision_by_id(
    revision_id: i32,
    db_connection: &mut PooledConnection<ConnectionManager<SqliteConnection>>,
) -> Result<RevisionDb, String> {
    use crate::schema::revisions::dsl::*;

    revisions
        .filter(id.eq(revision_id))
        .first::<RevisionDb>(db_connection)
        .map_err(|e| format!("Error fetching revision: {}", e))
}

pub fn update_revision_data(
    revision_par: RevisionDb,
    revision_id: i32,
    db_connection: &mut PooledConnection<ConnectionManager<SqliteConnection>>,
) -> Result<(), String> {
    use crate::schema::revisions::dsl::*;

    println!("Updating revision data");

    let res = diesel::update(revisions.find(revision_id))
        .set((
            revision_hash.eq(&revision_par.revision_hash),
            previous_verification_hash.eq(&revision_par.previous_verification_hash),
            nonce.eq(&revision_par.nonce),
            local_timestamp.eq(&revision_par.local_timestamp),
            revision_type.eq(&revision_par.revision_type),
            file_hash.eq(&revision_par.file_hash),
            content.eq(&revision_par.content),
            link_type.eq(&revision_par.link_type),
            link_require_indepth_verification.eq(&revision_par.link_require_indepth_verification),
            link_verification_hash.eq(&revision_par.link_verification_hash),
            link_uri.eq(&revision_par.link_uri),
            signature_data.eq(&revision_par.signature),
            signature_public_key.eq(&revision_par.signature_public_key),
            signature_wallet_address.eq(&revision_par.signature_wallet_address),
            signature_type.eq(&revision_par.signature_type),
            witness_merkle_root.eq(&revision_par.witness_merkle_root),
            witness_timestamp.eq(&revision_par.witness_timestamp),
            witness_network.eq(&revision_par.witness_network),
            witness_smart_contract_address.eq(&revision_par.witness_smart_contract_address),
            witness_transaction_hash.eq(&revision_par.witness_transaction_hash),
            witness_sender_account_address.eq(&revision_par.witness_sender_account_address),
            leaves.eq(&revision_par.leaves),
            updated_at.eq(Utc::now().naive_utc().to_string()),
        ))
        .execute(db_connection)
        .map_err(|e| format!("Error updating revision data: {}", e))?;

    println!("Updating result is: {:#?}", res);
    Ok(())
}

pub fn delete_revisions_by_id(
    revision_id: i32,
    db_connection: &mut PooledConnection<ConnectionManager<SqliteConnection>>,
) -> Result<i8, String> {
    use crate::schema::revisions::dsl::*;

    let deleted_count = diesel::delete(revisions.filter(id.eq(&revision_id)))
        .execute(db_connection)
        .map_err(|e| format!("Error deleting revision: {}", e))?;

    Ok(deleted_count as i8)
}

pub fn delete_all_revisions(
    db_connection: &mut PooledConnection<ConnectionManager<SqliteConnection>>,
) -> Result<(), String> {
    use crate::schema::revisions::dsl::*;

    diesel::delete(revisions)
        .execute(db_connection)
        .map_err(|e| format!("Error deleting all revisions: {}", e))?;

    Ok(())
}

use aqua_verifier_rs_types::models::revision::Revision;
use chrono::{NaiveDateTime, Utc};
use diesel::{
    prelude::*,
    r2d2::{ConnectionManager, PooledConnection},
    result::Error as DieselError,
    SqliteConnection,
};
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
        .select(RevisionDb::as_select()) // Explicitly select fields for RevisionDb
        .first::<RevisionDb>(db_connection)
        .map_err(|e| format!("Error fetching revision: {}", e))
}


// pub fn fetch_revision_by_nounce(
//     revision_nounce: String,
//     db_connection: &mut PooledConnection<ConnectionManager<SqliteConnection>>,
// ) -> Result<RevisionDb, String> {
//     use crate::schema::revisions::dsl::*;

//     revisions
//         .filter(nonce.eq(revision_nounce))
//         .select(RevisionDb::as_select()) // Explicitly select fields for RevisionDb
//         .first::<RevisionDb>(db_connection)
//         .map_err(|e| format!("Error fetching revision: {}", e))
// }
// use diesel::result::Error;

pub fn fetch_revision_by_nonce(
    revision_nonce: String,
    db_connection: &mut PooledConnection<ConnectionManager<SqliteConnection>>,
) -> Result<RevisionDb, String> {
    use crate::schema::revisions::dsl::*;

    match revisions
        .filter(nonce.eq(revision_nonce.clone()))
        .select(RevisionDb::as_select()) // Explicitly select fields for RevisionDb
        .first::<RevisionDb>(db_connection)
    {
        Ok(revision) => Ok(revision), // Found the revision, return it
        Err(diesel::result::Error::NotFound) => {
            Err(format!("No revision found with nonce: {}", revision_nonce)) // Custom error when not found
        }
        Err(e) => Err(format!("Error fetching revision: {}", e)), // Other errors
    }
}


pub fn update_revision_data(
    revision_par: RevisionDb,
    revision_id: i32,
    revision_hash : String,
    db_connection: &mut PooledConnection<ConnectionManager<SqliteConnection>>,
) -> Result<(), String> {
    use crate::schema::revisions::dsl::*;

    println!("Updating revision data");

    let res = diesel::update(revisions.find(revision_id))
        .set((
            revision_hash.eq(revision_hash),
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
            signature_data.eq(&revision_par.signature_data),
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



pub fn update_revision_data_using_nounce(
    revision_par: RevisionDb,
    nounce_par: String,
    revision_hash : String,
    db_connection: &mut PooledConnection<ConnectionManager<SqliteConnection>>,
) -> Result<(), String> {
    use crate::schema::revisions::dsl::*;

    println!("Updating revision data");

    let res = diesel::update(revisions)
        .filter(nonce.eq(nounce_par))  // Changed from find() to filter() since we're querying by nonce
        .set((
            revision_hash.eq(revision_hash),
            previous_verification_hash.eq(&revision_par.previous_verification_hash),
            local_timestamp.eq(&revision_par.local_timestamp),
            revision_type.eq(&revision_par.revision_type),
            file_hash.eq(&revision_par.file_hash),
            content.eq(&revision_par.content),
            link_type.eq(&revision_par.link_type),
            link_require_indepth_verification.eq(&revision_par.link_require_indepth_verification),
            link_verification_hash.eq(&revision_par.link_verification_hash),
            link_uri.eq(&revision_par.link_uri),
            signature_data.eq(&revision_par.signature_data),
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
            updated_at.eq(chrono::Utc::now().naive_utc()),  // Fixed timestamp handling
        ))
        .execute(db_connection)
        .map_err(|e| format!("Error updating revision data: {}", e))?;

    println!("Updated {} record(s)", res);
    Ok(())
}

pub fn update_revision_data_by_nonce(
    nonce_value: String,
    updated_revision: RevisionDb,
    revision_hash: String,
    db_connection: &mut PooledConnection<ConnectionManager<SqliteConnection>>,
) -> Result<(), String> {
    use crate::schema::revisions::dsl::*;

    println!("Updating revision data by nonce");

    let res = diesel::update(revisions.filter(nonce.eq(&nonce_value)))
        .set((
            revision_hash.eq(revision_hash),
            previous_verification_hash.eq(&updated_revision.previous_verification_hash),
            nonce.eq(&updated_revision.nonce), // Update nonce if necessary
            local_timestamp.eq(&updated_revision.local_timestamp),
            revision_type.eq(&updated_revision.revision_type),
            file_hash.eq(&updated_revision.file_hash),
            content.eq(&updated_revision.content),
            link_type.eq(&updated_revision.link_type),
            link_require_indepth_verification.eq(&updated_revision.link_require_indepth_verification),
            link_verification_hash.eq(&updated_revision.link_verification_hash),
            link_uri.eq(&updated_revision.link_uri),
            signature_data.eq(&updated_revision.signature_data),
            signature_public_key.eq(&updated_revision.signature_public_key),
            signature_wallet_address.eq(&updated_revision.signature_wallet_address),
            signature_type.eq(&updated_revision.signature_type),
            witness_merkle_root.eq(&updated_revision.witness_merkle_root),
            witness_timestamp.eq(&updated_revision.witness_timestamp),
            witness_network.eq(&updated_revision.witness_network),
            witness_smart_contract_address.eq(&updated_revision.witness_smart_contract_address),
            witness_transaction_hash.eq(&updated_revision.witness_transaction_hash),
            witness_sender_account_address.eq(&updated_revision.witness_sender_account_address),
            leaves.eq(&updated_revision.leaves),
            updated_at.eq(Utc::now().naive_utc().to_string()), // Update only updated_at
        ))
        .execute(db_connection)
        .map_err(|e| format!("Error updating revision data by nonce: {}", e))?;

    println!("Update result is: {:#?}", res);
    Ok(())
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

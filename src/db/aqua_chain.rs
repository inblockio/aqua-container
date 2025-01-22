use crate::{
    db::aqua_revisions::insert_revision,
    models::{aqua_chain_to_aqua_chain_db, revision_to_revision_db, AquaChainDb, RevisionDb},
    util::vec_to_string,
};
use aqua_verifier_rs_types::models::chain::AquaChain;
use aqua_verifier_rs_types::models::revision::Revision;
use chrono::{NaiveDateTime, Utc};
use diesel::{
    prelude::*,
    r2d2::{ConnectionManager, PooledConnection},
    result::Error as DieselError,
    SqliteConnection,
};
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

    let mut rev_id_list: Vec<i32> = Vec::new();

    for item in aqua_chain.revisions.iter() {
        // Method 4: Get current UTC time
        // let dt = chrono::Utc::now().naive_utc();

        // Method 5: Get current local time
        let dt = chrono::Local::now().naive_local();

        let rev = revision_to_revision_db(item.1.clone(), dt, dt);

        let rev = insert_revision(rev, db_connection);
        if rev.is_err() {
            return Err(format!("Error inserting revision: {}", rev.err().unwrap()));
        }
        let rev_id = rev.unwrap();
        rev_id_list.push(rev_id);
    }
    let db_record = aqua_chain_to_aqua_chain_db(
        aqua_chain.file_index[0].0.clone(),
        aqua_chain.file_index[0].1.clone(),
        file_content,
        owner,
        mode,
        rev_id_list,
    );
    // aqua_chain.to_db(file_hash, file_name, file_content, owner, mode);

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

    // ...
    // let updated_record : AquaChainDb = aqua_chain_to_aqua_chain_db(
    //     aqua_chain.fi,
    // );

    // diesel::update(aqua_chain.filter(id.eq(chain_id)))
    //     .set((
    //         file_hash.eq(updated_record.file_hash),
    //         file_name.eq(updated_record.file_name),
    //         revisions.eq(updated_record.revisions),
    //         file_content.eq(updated_record.file_content),
    //         owner.eq(updated_record.owner),
    //         mode.eq(updated_record.mode),
    //         is_shared.eq(updated_record.is_shared),
    //         updated_at.eq(Utc::now().naive_utc()),
    //     ))
    //     .execute(db_connection)
    //     .map_err(|e| format!("Error updating AquaChain: {}", e))?;

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
        // .select(AquaChainDb::as_select()) // Explicitly specify the selection
        .load::<AquaChainDb>(db_connection)
        .map_err(|e| format!("Error fetching AquaChains by owner: {}", e))?;

    Ok(results)
}

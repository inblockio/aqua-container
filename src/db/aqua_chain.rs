use crate::{
    db::aqua_revisions::{fetch_revision_by_nonce,  insert_revision, update_revision_data_by_nonce},
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
    file_hash: String,
    file_name: String,
    file_content: String,
    owner: String,
    mode: String,
    db_connection: &mut PooledConnection<ConnectionManager<SqliteConnection>>,
) -> Result<(), String> {
    use crate::schema::aqua_chain::dsl::*;

    let mut revsion_id : Vec<i32> = Vec::new();

    for (hash , revision) in updated_data.revisions.iter() {

         
        match fetch_revision_by_nonce(revision.nonce.clone(),  db_connection) {
            Ok(revision_db) => {
                println!("update_aqua_chain: Found revision: {:?}", revision);

                let rev_db_structure = revision_to_revision_db(revision.clone(), revision_db.created_at, Utc::now().naive_utc());
                
                let revision_in_db =update_revision_data_by_nonce(
                    revision.nonce.clone(),
                    rev_db_structure.clone(),
                    revision.clone().file_hash.unwrap(),
                    db_connection,
                );
            }
            Err(e) => {
                if e.contains("No revision found with nonce") {
                    println!("update_aqua_chain : Custom error: {}", e); // Handling 'NotFound' error specifically

                    let dt = chrono::Local::now().naive_local();

                    let rev = revision_to_revision_db(revision.clone(), dt, dt);

                    let rev = insert_revision(rev, db_connection);

                    if rev.is_err() {
                        return Err(format!("Error inserting revision: {}", rev.err().unwrap()));
                    }

                    revsion_id.push(rev.unwrap());
                } else {
                    println!("update_aqua_chain : Other error: {}", e); // Handling all other errors
                    return Err(format!("Error fetching revision: {}", e));
                }
            }
        }
 
    }
    
    let aqua_chain_db_data_result = aqua_chain
        .filter(id.eq(chain_id))
        .first::<AquaChainDb>(db_connection)
        .map_err(|e| format!("Error fetching AquaChain: {}", e));

    if aqua_chain_db_data_result.is_err()    {
        return Err(format!("Error fetching AquaChain(cannot fetch chain not saved in db): {}", aqua_chain_db_data_result.err().unwrap()));
    }

    let aqua_chain_db_data = aqua_chain_db_data_result.unwrap();


    diesel::update(aqua_chain.filter(id.eq(chain_id)))
        .set((
            file_hash.eq(file_hash),
            file_name.eq(file_name),
            revisions.eq(vec_to_string(revsion_id)),
            file_content.eq(file_content),
            owner.eq(owner),
            mode.eq(mode),
            is_shared.eq(aqua_chain_db_data.is_shared),
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
        // .select(AquaChainDb::as_select()) // Explicitly specify the selection
        .load::<AquaChainDb>(db_connection)
        .map_err(|e| format!("Error fetching AquaChains by owner: {}", e))?;

    Ok(results)
}

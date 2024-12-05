use std::env;

use crate::models::{PagesTable, UserProfilesTable, DB_POOL};
use chrono::{NaiveDateTime, Utc};
use diesel::r2d2::{self, ConnectionManager, PooledConnection};
use diesel::SqliteConnection;
use serde::{Deserialize, Serialize};

use diesel::prelude::*;
use diesel::prelude::*;
use diesel::result::Error as DieselError;

pub fn insert_user_profile_data(
    address_par: String,
    db_connection: &mut PooledConnection<ConnectionManager<SqliteConnection>>,
) -> Result<UserProfilesTable, String> {
    let user_profile = fetch_user_profile(address_par.clone(), db_connection);

    println!("Checking if user exists: {:?}", user_profile);

    if user_profile.is_err(){
        if user_profile.clone().err().unwrap() == "No user profile found with filename".to_string(){
            tracing::info!("Profile not found");
        }
        else{
            return Err("Unable to query if user exists".to_string())
        }
    }

    if user_profile.is_ok(){
        return Ok(user_profile.unwrap())
    }

    let mut record = UserProfilesTable {
        id: None,
        address: address_par,
        theme: env::var("THEME").unwrap_or_default(),
        contract_address: env::var("CONTRACT_ADDRESS").unwrap_or_default(),
        domain_name: env::var("API_DOMAIN").unwrap_or_default(),
        chain: env::var("CHAIN").unwrap_or_default(),
        file_mode: env::var("FILE_MODE").unwrap_or_default(),
    };

    println!("Inserted record: {:?}", record);

    let inserted_id: i32 = diesel::insert_into(crate::schema::user_profiles::table)
        .values(&record)
        .returning(crate::schema::user_profiles::dsl::id)
        .get_result::<Option<i32>>(db_connection)
        .map_err(|e| format!("Error saving new siwe data: {}", e))?
        .unwrap_or(-1); // Provide a default value if None
    record.id = Some(inserted_id);
    println!("Returned record: {:?}", record);
    Ok(record)
}

pub fn fetch_user_profile(
    address_par: String,
    db_connection: &mut PooledConnection<ConnectionManager<SqliteConnection>>,
) -> Result<UserProfilesTable, String> {
    use crate::schema::user_profiles::dsl::*;

    let result = user_profiles
        .filter(address.eq(&address_par))
        .first::<UserProfilesTable>(db_connection)
        .map_err(|e| match e {
            DieselError::NotFound => format!("No user profile found with filename"),
            _ => format!("Error fetching page data: {}", e),
        })?;
    Ok(result)
}

pub fn fetch_all_profiles(
    db_connection: &mut PooledConnection<ConnectionManager<SqliteConnection>>,
) -> Result<Vec<UserProfilesTable>, String> {
    use crate::schema::user_profiles::dsl::*;

    let results = user_profiles
        .load::<UserProfilesTable>(db_connection) 
        .map_err(|e| format!("Error fetching user profiles: {}", e))?;

    Ok(results)
}

pub fn update_user_profile(
    data: UserProfilesTable,
    db_connection: &mut PooledConnection<ConnectionManager<SqliteConnection>>,
) -> Result<(), String> {
    use crate::schema::user_profiles::dsl::*;

    let res = diesel::update(user_profiles.filter(address.eq(&data.address)))
        .set((
            chain.eq(&data.chain),
            theme.eq(&data.theme),
            file_mode.eq(&data.file_mode),
            domain_name.eq(&data.domain_name),
            contract_address.eq(&data.contract_address),
        ))
        .execute(db_connection)
        .map_err(|e| format!("Error updating page data: {}", e))?;
    println!("Updating result is: {:#?}", res);
    Ok(())
}

pub fn delete_user_profile(
    address_par: String,
    db_connection: &mut PooledConnection<ConnectionManager<SqliteConnection>>,
) -> Result<i8, String> {
    use crate::schema::user_profiles::dsl::*;

    let deleted_count = diesel::delete(user_profiles.filter(address.eq(&address_par)))
        .execute(db_connection)
        .map_err(|e| format!("Error deleting page data: {}", e))?;

    Ok(deleted_count as i8)
}

pub fn delete_all_profiles(
    db_connection: &mut PooledConnection<ConnectionManager<SqliteConnection>>,
) -> Result<(), String> {
    use crate::schema::user_profiles::dsl::*;

    diesel::delete(user_profiles)
        .execute(db_connection)
        .map_err(|e| format!("Error deleting all page data: {}", e))?;

    Ok(())
}

use crate::models::database_models::{SettingsTable, UserTable};
use crate::schema::Settings;
use crate::schema::Settings::dsl::*;
use diesel::pg::PgConnection;
use diesel::prelude::*;
use diesel::r2d2::{ConnectionManager, PooledConnection};
use futures::future::ok;

pub fn create_setting(
    conn: &mut PooledConnection<ConnectionManager<PgConnection>>,
    new_setting: SettingsTable,
) -> Result<(), String> {
    // create user if not exist
    let user = UserTable {
        user: new_setting.user_pub_key.clone(),
    };
    // Insert or do nothing if the user already exists
    let result = diesel::insert_into(crate::schema::User::table)
        .values(&user)
        .on_conflict(crate::schema::User::dsl::user) // Specify the unique column(s)
        .do_nothing() // Do nothing if there's a conflict
        .execute(conn)
        .map_err(|e| format!("Error inserting user: {}", e));

    if result.is_err() {
        return Err(format!("Error inserting user: {}", result.unwrap_err()));
    }
    // create user setting
    let setting = diesel::insert_into(Settings::table)
        .values(&new_setting)
        .on_conflict(crate::schema::Settings::dsl::user_pub_key) // Specify the unique column(s)
        .do_nothing() // Do nothing if there's a conflict
        .execute(conn)
        .map_err(|e| format!("Error inserting settings : {}", e));

        // .get_result::<SettingsTable>(conn);

    if setting.is_err() {
        return Err(format!("Error inserting setting: {}", setting.unwrap_err()));
    }
    Ok(())
}

pub fn get_setting(
    conn: &mut PooledConnection<ConnectionManager<PgConnection>>,
    user_key: &str,
) -> QueryResult<SettingsTable> {
    Settings.filter(user_pub_key.eq(user_key)).first(conn)
}

pub fn update_setting(
    conn: &mut PooledConnection<ConnectionManager<PgConnection>>,
    user_key: &str,
    updated_setting: SettingsTable,
) -> QueryResult<SettingsTable> {
    diesel::update(Settings.filter(user_pub_key.eq(user_key)))
        .set((
            cli_pub_key.eq(updated_setting.cli_pub_key),
            cli_priv_key.eq(updated_setting.cli_priv_key),
            witness_network.eq(updated_setting.witness_network),
            witness_contract_address.eq(updated_setting.witness_contract_address),
            theme.eq(updated_setting.theme),
        ))
        .get_result(conn)
}

// /// Deletes a record from the `Settings` table based on the `user_pub_key`.
// ///
// /// # Arguments
// /// * `conn` - A reference to the database connection.
// /// * `user_key` - The `user_pub_key` of the record to delete.
// ///
// /// # Returns
// /// Returns the number of rows deleted (`usize`) or a `diesel::result::Error`.
pub fn delete_setting(
    conn: &mut PooledConnection<ConnectionManager<PgConnection>>,
    user_key: &str,
) -> QueryResult<usize> {
    diesel::delete(Settings.filter(user_pub_key.eq(user_key))).execute(conn)
}

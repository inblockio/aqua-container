use crate::models::database_models::SettingsTable;
use crate::schema::Settings;
use diesel::pg::PgConnection;
use diesel::prelude::*;
use diesel::r2d2::{ConnectionManager, PooledConnection};
use crate::schema::Settings::dsl::*;

pub fn create_setting(conn: &mut PooledConnection<ConnectionManager<PgConnection>>, new_setting: SettingsTable) -> QueryResult<SettingsTable> {
    diesel::insert_into(Settings::table)
        .values(&new_setting)
        .get_result(conn)
}

pub fn get_setting(conn: &mut PooledConnection<ConnectionManager<PgConnection>>, user_key: &str) -> QueryResult<SettingsTable> {
  
  
    Settings.filter(user_pub_key.eq(user_key))
        .first(conn)
}

pub fn update_setting(conn: &mut PooledConnection<ConnectionManager<PgConnection>>, user_key: &str, updated_setting: SettingsTable) -> QueryResult<SettingsTable> {
    
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
pub fn delete_setting(conn: &mut PooledConnection<ConnectionManager<PgConnection>>, user_key: &str) -> QueryResult<usize> {
    diesel::delete(Settings.filter(user_pub_key.eq(user_key)))
        .execute(conn)
}

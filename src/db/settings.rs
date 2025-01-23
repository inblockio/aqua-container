use crate::models::database_models::SettingsTable;
use crate::schema::Settings;
use diesel::pg::PgConnection;
use diesel::prelude::*;

pub fn create_setting(conn: &PgConnection, new_setting: SettingsTable) -> Result<i64, String> {
    // diesel::insert_into(Settings::table)
    //     .values(&new_setting)
    //     .get_result(conn)

    // let inserted_id: i32 = diesel::insert_into(crate::schema::Settings::table)
    //     .values(data)
    //     .returning(crate::schema::share_data::dsl::id)
    //     .get_result::<Option<i32>>(db_connection)
    //     .map_err(|e| format!("Error saving new settings data: {}", e))?
    //     .unwrap_or(-1);  // Provide a default value if None

    // Ok(inserted_id as i64)
    Ok(0)
}

// pub fn get_setting(conn: &PgConnection, user_key: &str) -> QueryResult<SettingsTable> {
//     Settings.filter(user_pub_key.eq(user_key))
//         .first(conn)
// }

// pub fn update_setting(conn: &PgConnection, user_key: &str, updated_setting: SettingsTable) -> QueryResult<SettingsTable> {
//     diesel::update(Settings.filter(user_pub_key.eq(user_key)))
//         .set(&updated_setting)
//         .get_result(conn)
// }

// /// Deletes a record from the `Settings` table based on the `user_pub_key`.
// ///
// /// # Arguments
// /// * `conn` - A reference to the database connection.
// /// * `user_key` - The `user_pub_key` of the record to delete.
// ///
// /// # Returns
// /// Returns the number of rows deleted (`usize`) or a `diesel::result::Error`.
// pub fn delete_setting(conn: &PgConnection, user_key: &str) -> QueryResult<usize> {
//     diesel::delete(Settings.filter(user_pub_key.eq(user_key)))
//         .execute(conn)
// }

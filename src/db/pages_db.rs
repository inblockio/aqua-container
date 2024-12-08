use crate::models::{PagesTable, DB_POOL};
use chrono::{NaiveDateTime, Utc};
use diesel::r2d2::{self, ConnectionManager, PooledConnection};
use diesel::SqliteConnection;
use serde::{Deserialize, Serialize};

use diesel::prelude::*;
use diesel::prelude::*;
use diesel::result::Error as DieselError;

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct db_data {
    pub id: i64,
    pub name: String,
    pub extension: String,
    pub page_data: String,
    pub mode: String,
    pub owner: String,
}

pub fn insert_page_data(
    data: db_data,
    db_connection: &mut PooledConnection<ConnectionManager<SqliteConnection>>,
) -> Result<i64, String> {
    // Use Utc to get the current time as a NaiveDateTime
    let naive_datetime: NaiveDateTime = Utc::now().naive_utc();

    let datetime_string = naive_datetime.format("%Y-%m-%d %H:%M:%S").to_string();
    let record = PagesTable {
        id: None,
        name: data.name,
        extension: data.extension,
        mode: data.mode,
        page_data: data.page_data,
        owner: data.owner,
        created_at: datetime_string,
    };

    let inserted_id: i32 = diesel::insert_into(crate::schema::pages::table)
        .values(&record)
        .returning(crate::schema::pages::dsl::id)
        .get_result::<Option<i32>>(db_connection)
        .map_err(|e| format!("Error saving new siwe data: {}", e))?
        .unwrap_or(-1); // Provide a default value if None

    Ok(inserted_id as i64)
}

// The existing db_data and PagesTable structs remain the same

pub fn fetch_page_data(
    filename: String,
    db_connection: &mut PooledConnection<ConnectionManager<SqliteConnection>>,
) -> Result<db_data, String> {
    use crate::schema::pages::dsl::*;

    let result = pages
        .filter(name.eq(&filename))
        .first::<PagesTable>(db_connection)
        .map_err(|e| match e {
            DieselError::NotFound => format!("No page found with filename: {:#?}", filename),
            _ => format!("Error fetching page data: {}", e),
        })?;
   
    Ok(db_data {
        id: result.id.unwrap_or(0) as i64,
        name: result.name,
        extension: result.extension,
        page_data: result.page_data,
        mode: result.mode,
        owner: result.owner,
    })
}

pub fn fetch_all_pages_data_per_user(
    user: String,
    db_connection: &mut PooledConnection<ConnectionManager<SqliteConnection>>,
) -> Result<Vec<PagesTable>, String> {
    use crate::schema::pages::dsl::*;

    let results = pages
        .filter(owner.eq(&user))
        .load::<PagesTable>(db_connection) // Replace `db_data` with the correct Diesel model type
        .map_err(|e| format!("Error fetching pages for user {}: {}", user, e))?;

    // Ok(db_data {
    //     id: result.id.unwrap_or(0) as i64,
    //     name: result.name,
    //     extension: result.extension,
    //     page_data: result.page_data,
    //     mode: result.mode,
    //     owner: result.owner,
    // })
    Ok(results)
}

pub fn update_page_data(
    data: db_data,
    db_connection: &mut PooledConnection<ConnectionManager<SqliteConnection>>,
) -> Result<(), String> {
    use crate::schema::pages::dsl::*;
    println!("Updating");
    let res = diesel::update(pages.filter(name.eq(&data.name)))
        .set((
            extension.eq(&data.extension),
            page_data.eq(&data.page_data),
            mode.eq(&data.mode),
            owner.eq(&data.owner),
        ))
        .execute(db_connection)
        .map_err(|e| format!("Error updating page data: {}", e))?;
    println!("Updating result is: {:#?}", res);
    Ok(())
}

pub fn delete_page_data(
    page_name: String,
    db_connection: &mut PooledConnection<ConnectionManager<SqliteConnection>>,
) -> Result<i8, String> {
    use crate::schema::pages::dsl::*;

    let deleted_count = diesel::delete(pages.filter(name.eq(&page_name)))
        .execute(db_connection)
        .map_err(|e| format!("Error deleting page data: {}", e))?;

    Ok(deleted_count as i8)
}

pub fn delete_all_data(
    db_connection: &mut PooledConnection<ConnectionManager<SqliteConnection>>,
) -> Result<(), String> {
    use crate::schema::pages::dsl::*;

    diesel::delete(pages)
        .execute(db_connection)
        .map_err(|e| format!("Error deleting all page data: {}", e))?;

    Ok(())
}

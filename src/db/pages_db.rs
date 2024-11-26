
use serde::{Deserialize, Serialize};
use crate::models::DB_POOL;
use diesel::SqliteConnection;
use diesel::r2d2::{self, ConnectionManager};

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct db_data {
    pub id : i64,
    pub name: String,
    pub extension: String,
    pub page_data: String,
    pub mode: String,
    pub owner: String,
}

// pub fn insert_page_data(data: db_data, db_connection : DB_POOL) -> Result<(i64), String> {
pub fn insert_page_data(data: db_data) -> Result<(i64), String> {
    
    
    return Ok(0);
}

pub fn fetch_page_data(name: String) -> Result<db_data, String> {
    let data = db_data {
        id :0,
        name: "".to_string(),
        extension: "".to_string(),
        page_data: "".to_string(),
        mode: "".to_string(),
        owner: "".to_string(),
    };

    return Ok(data);
}


pub fn update_page_data(data: db_data) -> Result<(), String> {
    return Ok(());
}

pub fn delete_page_data(name: String) -> Result<(i8), String> {
    return Ok(0);
}


pub fn delete_all_data() -> Result<(), String> {
    return Ok(());
}

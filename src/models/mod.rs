
use diesel::r2d2::{self, ConnectionManager};
use diesel::SqliteConnection;

pub mod file;
pub mod page_data;
pub mod input;


pub type DB_POOL = r2d2::Pool<ConnectionManager<SqliteConnection>>;
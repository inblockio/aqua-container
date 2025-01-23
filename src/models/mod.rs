pub mod api;
pub mod file;
pub mod input;
pub mod share_data;
pub mod user_profiles;
pub mod database_models;


pub type DB_POOL = r2d2::Pool<ConnectionManager<SqliteConnection>>;
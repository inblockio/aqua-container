use std::fs;
use std::path::Path;
use sqlx::{Pool, Sqlite};
use sqlx::sqlite::SqlitePoolOptions;
// use ethers::core::k256::SecretKey;
// use ethers::prelude::*;
// use rand::thread_rng;

pub async fn db_set_up() -> Pool<Sqlite> {
    let db_file_path = "./pages.db";
    let db_url = format!("sqlite://{}", db_file_path);

    // Check if the database file exists, if not, create it
    if !Path::new(db_file_path).exists() {
        println!("Database file not found, creating one...");

        // Create the directory if it doesn't exist
        let db_dir = Path::new(db_file_path).parent().unwrap();
        if !db_dir.exists() {
            fs::create_dir_all(db_dir).expect("Failed to create database directory");
        }

        // Create an empty database file
        fs::File::create(db_file_path).expect("Failed to create the database file");
    }else{
        println!("db exists {}", db_url)
    }

    // Initialize SQLite connection pool
    let pool = SqlitePoolOptions::new()
        .connect(db_url.as_str())
        .await
        .expect("Failed to connect to SQLite");

    // Create table if it doesn't exist
    sqlx::query(
        r#"

        "#,
    )
        .execute(&pool)
        .await
        .expect("Failed to create table");


    sqlx::query(
        r#"

        "#,
    )
        .execute(&pool)
        .await
        .expect("Failed to create key-value table");

    return pool;
}


pub fn check_or_generate_address(pool : &Pool<Sqlite>){



    // Generate a random private key
    // let mut rng = thread_rng();
    // let wallet = LocalWallet::new(&mut rng);
    //
    // // Get the wallet's address
    // let address = wallet.address();
    // let private_key = wallet.signer().to_string();
    //
    // // Output the wallet details
    // println!("Wallet Address: {:?}", address);
    // println!("Private Key: {:?}", private_key);


}
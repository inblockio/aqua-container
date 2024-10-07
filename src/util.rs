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

        println!("please run `sqlx database create` then `sqlx migrate run` ")
    }else{
        println!("db exists {}", db_url)
    }

    // Initialize SQLite connection pool
    let pool = SqlitePoolOptions::new()
        .connect(db_url.as_str())
        .await
        .expect("Failed to connect to SQLite");

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
use std::{env, fs};
use std::path::Path;
use rand::distributions::Alphanumeric;
use sqlx::{Pool, Sqlite};
use sqlx::sqlite::SqlitePoolOptions;
use ethers::core::k256::SecretKey;
use ethers::prelude::*;
use rand::{Rng, thread_rng};

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


pub fn check_or_generate_domain(){




    // Check if API_DOMAIN is set
    let api_domain = env::var("API_DOMAIN").unwrap_or_default();

    if api_domain.is_empty() {
        println!("API_DOMAIN is empty, generating a random one...");

        // Generate a random alphanumeric string
        let random_domain: String = thread_rng()
            .sample_iter(&Alphanumeric)
            .take(10)
            .map(char::from)
            .collect();

        // //Generate a random private key
        // let mut rng =  thread_rng();
        // let wallet = LocalWallet::new(&mut rng);
        //
        // // Get the wallet's address
        // let address = wallet.address();
        // let private_key = wallet.signer().to_string();
        //
        // // Output the wallet details
        // println!("Wallet Address: {:?}", address);
        // println!("Private Key: {:?}", private_key);

        println!("Generated API_DOMAIN: {:?}", random_domain);

        // Update the .env file with the new API_DOMAIN
        if let Err(e) = update_env_file("API_DOMAIN", &random_domain) {
            eprintln!("Failed to update .env file: {}", e);
        }
    } else {
        println!("API_DOMAIN is set: {}", api_domain);
    }
}

// Function to update the .env file with the new API_DOMAIN
fn update_env_file(key: &str, value: &str) -> std::io::Result<()> {
    // Read the existing .env file contents
    let mut env_content = std::fs::read_to_string(".env").unwrap_or_default();

    // Check if the key already exists in the file
    if env_content.contains(key) {
        // Update the existing key-value pair
        let new_content = env_content
            .lines()
            .map(|line| {
                if line.starts_with(key) {
                    format!("{}={}", key, value)
                } else {
                    line.to_string()
                }
            })
            .collect::<Vec<_>>()
            .join("\n");

        // Write the updated content back to the .env file
        std::fs::write(".env", new_content)?;
    } else {
        // If the key doesn't exist, append it to the file
        let mut file = std::fs::OpenOptions::new().append(true).open(".env")?;
        writeln!(file, "{}={}", key, value)?;
    }
    Ok(())
}
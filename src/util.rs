use std::{env, fs};
use std::path::Path;
use rand::distributions::Alphanumeric;
use sqlx::{Pool, Sqlite};
use sqlx::sqlite::SqlitePoolOptions;
use ethers::core::k256::SecretKey;
use ethers::prelude::*;
use rand::{Rng, thread_rng};
use std::io::Write;
use guardian_common::custom_types::{Hash, Revision, RevisionContent};
use sha3::{Digest, Sha3_512};
use serde_json::Value;
use std::collections::BTreeMap;

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



pub fn check_if_page_data_revision_are_okay(revisions: Vec<(Hash, Revision)>) ->bool {
    let mut is_valid= true;

    if revisions.len() <=1 {
        return is_valid
    }


    for (index, (hash,revision)) in revisions.iter().enumerate() {

    }


    return is_valid;
}


fn compute_content_hash(content: &RevisionContent) -> Result<String, String> {
    // Assuming content is serialized and hashed
    let mut file_hasher = Sha3_512::new();

    // Include the file's base64-encoded data
    if let Some(file) = &content.file {
        file_hasher.update(&*file.data.clone());
    }

    // Include the content map (sorted by key)
    let mut sorted_content = BTreeMap::new();
    for (key, value) in &content.content {
        sorted_content.insert(key.clone(), value.clone());
    }

    // Hash the content data
    let serialized_content = match serde_json::to_string(&sorted_content) {
        Ok(data) => data,
        Err(e) => return Err(format!("Failed to serialize content: {}", e)),
    };

    file_hasher.update(serialized_content);

    // Finalize the hash and return it as a hex string
    Ok(format!("{:x}", file_hasher.finalize()))
}



use ethers::core::k256::SecretKey;
use ethers::prelude::*;
use guardian_common::custom_types::{Hash, Revision, RevisionContent};
use rand::distributions::Alphanumeric;
use rand::{thread_rng, Rng};
use serde_json::Value;
use sha3::{Digest, Sha3_512};
use sqlx::sqlite::SqlitePoolOptions;
use sqlx::{Pool, Sqlite};
use std::collections::BTreeMap;
use std::io::Write;
use std::path::Path;
use std::{env, fs};

pub async fn db_set_up() -> Pool<Sqlite> {
    let db_file_path = "./pages.db";
    let db_url = format!("sqlite://{}", db_file_path);

    // Check if the database file exists, if not, create it
    if !Path::new(db_file_path).exists() {
        println!("please run `sqlx database create` then `sqlx migrate run` ")
    } else {
        println!("db exists {}", db_url)
    }

    // Initialize SQLite connection pool
    let pool = SqlitePoolOptions::new()
        .connect(db_url.as_str())
        .await
        .expect("Failed to connect to SQLite");

    return pool;
}

pub fn check_or_generate_domain() {
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

pub fn check_if_page_data_revision_are_okay(revisions: Vec<(Hash, Revision)>) -> (bool, String) {
    let mut is_valid = (true, "".to_string());
    let has_valid_genessis = revsions_has_valid_genesis(revisions.clone());
    tracing::debug!("revsions_has_valid_genesis {:#?}", has_valid_genessis);

    if has_valid_genessis.is_none() {
        return (false, "revisions do not contain a valid genesis".to_string());
    }

    // check if the revision > metadata > previous_verification_hash is among the hash in revsions par
    // if more that one is none return false
    // there is a broken revision chain
    let mut all_hashes: Vec<Hash> = Vec::new();
    revisions
        .iter()
        .for_each(|(hash, revision)| all_hashes.push(hash.clone()));

    let genesis_hash_str = format!("{:#?}", has_valid_genessis.unwrap());

    for (index, (current_hash, current_revision)) in revisions.iter().enumerate() {
        let current_hash_str = format!("{:#?}", current_hash);

        // check hash if match the newly generated one
        let recomputed_content_hash = compute_content_hash(current_revision.content);

        match recomputed_content_hash {
            Ok(data) => {
                if data == *current_hash {
                    tracing::error!("hashes match the generetaed one continue ...");
                }else{
                    tracing::error!("\n hashes do not match revision has {:#?} \n vs generated hash {:#?} \n",data,current_hash );
                    is_valid = (false, format!("a hash is not valid : {:#?}",  current_hash));

                    break;
                }
            }
            Err(error) => {
                tracing::error!("an error occured {}", error);
                is_valid = (false, "error generating a hash ".to_string());
                break;
            }
        }
        // let contnent_hash_str = format!("{:#?}", revision.content.content_hash);
        // let data_str = format!("{:#?}", revision.content.content_hash);
        // tracing::error!("returd conetnet is   {} \n  my json content hash is {} \n", data_str, contnent_hash_str);
        // matches = data ==revision.content.content_hash  ;//revision.content.content_hash;

        // chec if the hash chain is valid (ie if there any orphan revisions)
        if current_hash_str == genesis_hash_str {
            tracing::debug!("ignoring genessis hash is {:#?}", genesis_hash_str);
        } else {
            let contains = all_hashes.contains(current_hash);

            if contains == false {
                tracing::debug!("cannot find hash is {:#?}", current_hash_str);
                is_valid = (false, "Hash chain is invalid ".to_string());;
                break;
            }
        }
    }

    return is_valid;
}
pub fn revsions_has_valid_genesis(revisions: Vec<(Hash, Revision)>) -> Option<Hash> {
    // let mut is_valid= true;

    if revisions.len() <= 1 {
        tracing::debug!("The lengthe is equal to or ess than 1 {}", revisions.len());
        return None;
    }

    let mut revision_genesis: Vec<&Revision> = Vec::new();

    for (index, (hash, revision)) in revisions.iter().enumerate() {
        match revision.metadata.previous_verification_hash {
            Some(data) => {
                tracing::debug!("The previous hash is {:#?}", data);
            }
            None => {
                tracing::debug!("pushing revision to vector {:#?}", revision);
                revision_genesis.push(revision)
            }
        }
    }

    if revision_genesis.len() > 1 {
        tracing::debug!(
            "The genesis revision  length {} are {:#?}",
            revision_genesis.len(),
            revision_genesis
        );
        return None;
    }

    let res = revision_genesis.first();
    if res.is_none() {
        tracing::debug!("No genesis hash  (vec is empty)",);
        return None;
    }

    tracing::debug!("************************ {:#?}", res);
    // we use unwrapp becasue we are guaranteed the res has value due to the if check above
    return Some(res.unwrap().metadata.verification_hash);
}

pub fn compute_content_hash(contentPar: &RevisionContent) -> Result<Hash, String> {
    let b64 = contentPar.file.clone().unwrap().data;
    let mut file_hasher = sha3::Sha3_512::default();
    file_hasher.update(b64.clone());
    let file_hash_current = Hash::from(file_hasher.finalize());

    let mut content_current = BTreeMap::new();

    content_current.insert("file_hash".to_owned(), file_hash_current.to_string());

    // println!("{:#?}", content_current);
    tracing::debug!("{:#?}", content_current);

    let content_hash_current = verifier::v1_1::hashes::content_hash(&content_current.clone());

    tracing::debug!("{:#?}", content_hash_current);

    Ok(content_hash_current)
}

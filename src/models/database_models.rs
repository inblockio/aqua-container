use chrono::{DateTime, Utc};

use crate::util::vec_to_string;
use chrono::NaiveDateTime;
use diesel::expression::AsExpression;
use diesel::prelude::*;
use diesel::prelude::*;
use diesel::sql_types::Nullable;
use siwe::TimeStamp;
use aqua_verifier_rs_types::models::chain::AquaChain;
use aqua_verifier_rs_types::models::revision::Revision;
use diesel::prelude::*;
use diesel::r2d2::{self, ConnectionManager};
use diesel::SqliteConnection;
use serde::{Deserialize, Serialize};


#[derive(Queryable, Selectable, Deserialize, Serialize, Debug, Clone, Insertable)]
#[diesel(table_name = crate::schema::siwe_sessions)]
#[diesel(check_for_backend(diesel::sqlite::Sqlite))]
pub struct SiweSessionsTable {
    pub id: Option<i32>,
    pub address: String,
    pub nonce: String,
    pub issued_at: String,
    pub expiration_time: Option<String>,
}

#[derive(Queryable, Insertable, Selectable, Debug, Clone, Deserialize, Serialize)]
#[diesel(table_name = crate::schema::user)]
pub struct UserTable {
    pub user: String, // pubkey
}


#[derive(Queryable, Insertable, Selectable, Debug, Clone, Deserialize, Serialize)]
#[diesel(table_name = crate::schema::contract)]
pub struct ContractTable {
    pub hash: String,          // hash
    pub latest: Option<Vec<String>>, // TEXT[]
    pub sender: Option<String>, // pubkey
    pub receiver: Option<String>, // pubkey
    pub option: Option<String>, // TEXT
    pub reference_count: Option<i32>,
}

#[derive(Queryable, Insertable, Selectable, Debug, Clone, Deserialize, Serialize)]
#[diesel(table_name = crate::schema::latest)]
pub struct LatestTable {
    pub hash: String,  // hash
    pub owner: String, // pubkey
}


#[derive(Queryable, Insertable, Selectable, Debug, Clone, Deserialize, Serialize)]
#[diesel(table_name = crate::schema::revision)]
pub struct RevisionTable {
    pub hash: String,                 // hash
    pub owner: String,                // pubkey
    pub nonce: String,                // TEXT
    pub shared: Option<Vec<String>>,  // TEXT[]
    pub contract: Option<Vec<String>>, // TEXT[]
    pub previous: Option<String>,     // varchar
    pub children: Option<String>,     // TEXT
    pub local_timestamp: Option<chrono::NaiveDateTime>, // timestamp
    pub revision_type: Option<String>, // TEXT
    pub verification_leaves: Option<String>, // TEXT
}


#[derive(Queryable, Insertable, Selectable, Debug, Clone, Deserialize, Serialize)]
#[diesel(table_name = crate::schema::content)]
pub struct ContentTable {
    pub hash: String,         // hash
    pub content: Option<String>, // TEXT
    pub reference_count: Option<i32>,
}


#[derive(Queryable, Insertable, Selectable, Debug, Clone, Deserialize, Serialize)]
#[diesel(table_name = crate::schema::file_hash)]
pub struct FileHashTable {
    pub hash: String,         // hash
    pub file_hash: String,    // hash
    pub reference_count: Option<i32>,
}


#[derive(Queryable, Insertable, Selectable, Debug, Clone, Deserialize, Serialize)]
#[diesel(table_name = crate::schema::link)]
pub struct LinkTable {
    pub hash: String,                               // hash
    pub link_type: Option<String>,                 // TEXT
    pub link_require_indepth_verification: Option<bool>, // boolean
    pub link_verification_hash: Option<String>,    // TEXT
    pub reference_count: Option<i32>,
}

#[derive(Queryable, Insertable, Selectable, Debug, Clone, Deserialize, Serialize)]
#[diesel(table_name = crate::schema::index)]
pub struct IndexTable {
    pub hash: Option<Vec<String>>, // TEXT[]
    pub file_hash: String,         // hash
    pub uri: Option<String>,       // TEXT
}


#[derive(Queryable, Insertable, Selectable, Debug, Clone, Deserialize, Serialize)]
#[diesel(table_name = crate::schema::signature)]
pub struct SignatureTable {
    pub hash: String,                   // hash
    pub signature_digest: Option<String>, // TEXT
    pub signature_wallet_address: Option<String>, // varchar
    pub signature_type: Option<String>, // hash
}


#[derive(Queryable, Insertable, Selectable, Debug, Clone, Deserialize, Serialize)]
#[diesel(table_name = crate::schema::witness)]
pub struct WitnessTable {
    pub hash: String,               // hash
    pub witness_merkle_root: String, // hash
}


#[derive(Queryable, Insertable, Selectable, Debug, Clone, Deserialize, Serialize)]
#[diesel(table_name = crate::schema::witness_event)]
pub struct WitnessEventTable {
    pub witness_merkle_root: String,       // hash
    pub witness_timestamp: chrono::NaiveDateTime, // timestamp
    pub witness_network: Option<String>,  // TEXT
    pub witness_smart_contract_address: Option<String>, // hash
    pub witness_transaction_hash: Option<String>, // TEXT
    pub witness_sender_account_address: Option<String>, // TEXT
}

#[derive(Queryable, Insertable, Selectable, Debug, Clone, Deserialize, Serialize)]
#[diesel(table_name = crate::schema::merkle_nodes)]
pub struct MerkleNodesTable {
    pub node_hash: Option<String>,         // TEXT
    pub parent_hash: Option<String>,       // TEXT
    pub height: Option<i32>,               // INTEGER
    pub is_leaf: Option<bool>,             // BOOLEAN
    pub left_child_hash: Option<String>,   // TEXT
    pub right_child_hash: Option<String>,  // TEXT
}

#[derive(Queryable, Insertable, Selectable, Debug, Clone, Deserialize, Serialize)]
#[diesel(table_name = crate::schema::aqua_forms)]
pub struct AquaFormsTable {
    pub hash: String,         // hash
    pub key: Option<String>,  // TEXT
    pub value: Option<String>, // TEXT (Assuming object is stored as JSON-encoded string)
    pub r#type: Option<String>, // TEXT
}

#[derive(Queryable, Insertable, Selectable, Debug, Clone, Deserialize, Serialize)]
#[diesel(table_name = crate::schema::settings)]
pub struct SettingsTable {
    pub user_pub_key: String,           // pubkey
    pub cli_pub_key: Option<String>,    // pubkey
    pub cli_priv_key: Option<String>,   // private_key (Assuming TEXT or similar)
    pub witness_network: Option<String>, // chain_id (Assuming TEXT)
    pub witness_contract_address: Option<String>, // hash
    pub theme: Option<String>,          // TEXT
}

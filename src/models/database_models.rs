use chrono::{DateTime, Utc};

use crate::util::vec_to_string;
use aqua_verifier_rs_types::models::chain::AquaChain;
use aqua_verifier_rs_types::models::revision::Revision;
use chrono::NaiveDateTime;
use diesel::expression::AsExpression;
use diesel::prelude::*;
use diesel::prelude::*;
use diesel::prelude::*;
use diesel::r2d2::{self, ConnectionManager};
use diesel::sql_types::Nullable;
use serde::{Deserialize, Serialize};
use siwe::TimeStamp;

#[derive(Queryable, Selectable, Deserialize, Serialize, Debug, Clone, Insertable)]
#[diesel(table_name = crate::schema::siwe_sessions)]
pub struct SiweSessionsTable {
    pub id: i32,                                // SERIAL in PostgreSQL maps to i32 in Rust
    pub address: String,                        // TEXT in PostgreSQL maps to String in Rust
    pub nonce: String,                          // TEXT in PostgreSQL maps to String in Rust
    pub issued_at: DateTime<Utc>, // TIMESTAMPTZ in PostgreSQL maps to DateTime<Utc> in Rust
    pub expiration_time: Option<DateTime<Utc>>, // Nullable<TIMESTAMPTZ> maps to Option<DateTime<Utc>>
}
// pub struct SiweSessionsTable {
// pub id: Option<i32>,
// pub address: String,
// pub nonce: String,
// pub issued_at: String,
// pub expiration_time: Option<String>,
// }

#[derive(Queryable, Insertable, Selectable, Debug, Clone, Deserialize, Serialize)]
#[diesel(table_name = crate::schema::User)]
pub struct UserTable {
    pub user: String, // pubkey
}

#[derive(Queryable, Insertable, Selectable, Debug, Clone, Deserialize, Serialize)]
#[diesel(table_name = crate::schema::Contract)]
pub struct ContractTable {
    pub hash: String,                // hash
    pub latest: Option<Vec<String>>, // TEXT[]
    pub sender: Option<String>,      // pubkey
    pub receiver: Option<String>,    // pubkey
    pub option: Option<String>,      // TEXT
    pub reference_count: Option<i32>,
}

#[derive(Queryable, Insertable, Selectable, Debug, Clone, Deserialize, Serialize)]
#[diesel(table_name = crate::schema::Latest)]
pub struct LatestTable {
    pub hash: String,  // hash
    pub owner: String, // pubkey
}

#[derive(Queryable, Insertable, Selectable, Debug, Clone, Deserialize, Serialize)]
#[diesel(table_name = crate::schema::Revision)]
pub struct RevisionTable {
    pub hash: String,                                   // hash
    pub owner: String,                                  // pubkey
    pub nonce: String,                                  // TEXT
    pub shared: Option<Vec<String>>,                    // TEXT[]
    pub contract: Option<Vec<String>>,                  // TEXT[]
    pub previous: Option<String>,                       // varchar
    pub children: Option<String>,                       // TEXT
    pub local_timestamp: Option<chrono::NaiveDateTime>, // timestamp
    pub revision_type: Option<String>,                  // TEXT
    pub verification_leaves: Option<String>,            // TEXT
}

#[derive(Queryable, Insertable, Selectable, Debug, Clone, Deserialize, Serialize)]
#[diesel(table_name = crate::schema::Content)]
pub struct ContentTable {
    pub hash: String,            // hash
    pub content: Option<String>, // TEXT
    pub reference_count: Option<i32>,
}

#[derive(Queryable, Insertable, Selectable, Debug, Clone, Deserialize, Serialize)]
#[diesel(table_name = crate::schema::FileHash)]
pub struct FileHashTable {
    pub hash: String,      // hash
    pub file_hash: String, // hash
    pub reference_count: Option<i32>,
}

#[derive(Queryable, Insertable, Selectable, Debug, Clone, Deserialize, Serialize)]
#[diesel(table_name = crate::schema::Link)]
pub struct LinkTable {
    pub hash: String,                                    // hash
    pub link_type: Option<String>,                       // TEXT
    pub link_require_indepth_verification: Option<bool>, // boolean
    pub link_verification_hash: Option<String>,          // TEXT
    pub reference_count: Option<i32>,
}

#[derive(Queryable, Insertable, Selectable, Debug, Clone, Deserialize, Serialize)]
#[diesel(table_name = crate::schema::Index)]
pub struct IndexTable {
    pub hash: String,        //Option<Vec<String>>, // TEXT[]
    pub file_hash: String,   // hash
    pub uri: Option<String>, // TEXT
}

#[derive(Queryable, Insertable, Selectable, Debug, Clone, Deserialize, Serialize)]
#[diesel(table_name = crate::schema::Signature)]
pub struct SignatureTable {
    pub hash: String,                             // hash
    pub signature_digest: Option<String>,         // TEXT
    pub signature_wallet_address: Option<String>, // varchar
    pub signature_type: Option<String>,           // hash
}

#[derive(Queryable, Insertable, Selectable, Debug, Clone, Deserialize, Serialize)]
#[diesel(table_name = crate::schema::Witness)]
pub struct WitnessTable {
    pub hash: String,                // hash
    pub witness_merkle_root: String, // hash
}

#[derive(Queryable, Insertable, Selectable, Debug, Clone, Deserialize, Serialize)]
#[diesel(table_name = crate::schema::WitnessEvent)]
pub struct WitnessEventTable {
    pub witness_merkle_root: String,                    // hash
    pub witness_timestamp: chrono::NaiveDateTime,       // timestamp
    pub witness_network: Option<String>,                // TEXT
    pub witness_smart_contract_address: Option<String>, // hash
    pub witness_transaction_hash: Option<String>,       // TEXT
    pub witness_sender_account_address: Option<String>, // TEXT
}

#[derive(Queryable, Insertable, Selectable, Debug, Clone, Deserialize, Serialize)]
#[diesel(table_name = crate::schema::MerkleNodes)]
pub struct MerkleNodesTable {
    pub node_hash: Option<String>,        // TEXT
    pub parent_hash: Option<String>,      // TEXT
    pub height: Option<i32>,              // INTEGER
    pub is_leaf: Option<bool>,            // BOOLEAN
    pub left_child_hash: Option<String>,  // TEXT
    pub right_child_hash: Option<String>, // TEXT
}

#[derive(Queryable, Insertable, Selectable, Debug, Clone, Deserialize, Serialize)]
#[diesel(table_name = crate::schema::AquaForms)]
pub struct AquaFormsTable {
    pub hash: String,               // hash
    pub key: Option<String>,        // TEXT
    pub value: Option<String>, //Option<serde_json::Value>, //Option<String>, // TEXT (Assuming object is stored as JSON-encoded string)
    pub value_type: Option<String>, // TEXT
}

#[derive(Queryable, Insertable, Selectable, Debug, Clone, Deserialize, Serialize)]
#[diesel(table_name = crate::schema::Settings)]
pub struct SettingsTable {
    pub user_pub_key: String,                     // pubkey
    pub cli_pub_key: Option<String>,              // pubkey
    pub cli_priv_key: Option<String>,             // private_key (Assuming TEXT or similar)
    pub witness_network: Option<String>,          // chain_id (Assuming TEXT)
    pub witness_contract_address: Option<String>, // hash
    pub theme: Option<String>,                    // TEXT
}

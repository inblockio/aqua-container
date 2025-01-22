use aqua_verifier_rs_types::models::chain::AquaChain;
use aqua_verifier_rs_types::models::revision::Revision;
use diesel::prelude::*;
use diesel::r2d2::{self, ConnectionManager};
use diesel::SqliteConnection;
use serde::{Deserialize, Serialize};
pub mod api;
pub mod file;
pub mod input;
pub mod share_data;
pub mod user_profiles;
use chrono::{DateTime, Utc};

pub type DB_POOL = r2d2::Pool<ConnectionManager<SqliteConnection>>;
use crate::util::vec_to_string;
use chrono::NaiveDateTime;
use diesel::expression::AsExpression;
use diesel::prelude::*;
use diesel::prelude::*;
use diesel::sql_types::Nullable;
use siwe::TimeStamp;

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


#[derive(Queryable, Selectable, Deserialize, Serialize, Debug, Clone, Insertable)]
#[diesel(table_name = crate::schema::user_profiles)]
#[diesel(check_for_backend(diesel::sqlite::Sqlite))]
pub struct UserProfilesTable {
    pub id: i32,
    pub address: String,
    pub chain: String,
    pub theme: String,
    pub contract_address: String,
    pub file_mode: String,
    pub domain_name: String,
}

#[derive(Queryable, Insertable, Selectable, Serialize, Deserialize, Debug, Clone)]
#[diesel(table_name = crate::schema::aqua_chain)]
#[diesel(check_for_backend(diesel::sqlite::Sqlite))]
pub struct AquaChainDb {
    pub id: i32,
    pub file_hash: String,
    pub file_name: String,
    pub revisions: String, // id of revision
    pub file_content: String,
    pub owner: String,
    pub mode: String,
    pub share_code: Option<String>,
    pub is_shared: bool,
    #[diesel(sql_type = Timestamp)]
    pub updated_at: NaiveDateTime,
    #[diesel(sql_type = Timestamp)]
    pub created_at: NaiveDateTime,
}

pub fn aqua_chain_to_aqua_chain_db(
    file_hash: String,
    file_name: String,
    file_content: String,
    owner: String,
    mode: String,
    revision_id: Vec<i32>,
) -> AquaChainDb {
    AquaChainDb {
        id: 0,
        file_hash,
        file_name,
        revisions: vec_to_string(revision_id),
        file_content,
        owner,
        mode,
        share_code: None,
        is_shared: false,
        updated_at: Utc::now().naive_utc(),
        created_at: Utc::now().naive_utc(),
    }
}

#[derive(Queryable, Insertable, Serialize, Deserialize, Debug, Clone, Selectable)]
#[diesel(table_name = crate::schema::revisions)]
#[diesel(check_for_backend(diesel::sqlite::Sqlite))]
pub struct RevisionDb {
    pub id: i32,
    pub previous_verification_hash: String,
    pub nonce: String,
    pub local_timestamp: String,
    pub revision_type: String,
    pub file_hash: Option<String>,
    pub content: Option<String>,
    pub link_type: Option<String>,
    pub link_require_indepth_verification: Option<bool>,
    pub link_verification_hash: Option<String>,
    pub link_uri: Option<String>,
    pub signature_data: Option<String>,
    pub signature_public_key: Option<String>,
    pub signature_wallet_address: Option<String>,
    pub signature_type: Option<String>,
    pub witness_merkle_root: Option<String>,
    pub witness_timestamp: Option<String>,
    pub witness_network: Option<String>,
    pub witness_smart_contract_address: Option<String>,
    pub witness_transaction_hash: Option<String>,
    pub witness_sender_account_address: Option<String>,
    pub leaves: Option<String>, // Serialized as JSON string
    #[diesel(sql_type = Timestamp)]
    pub created_at: NaiveDateTime,
    #[diesel(sql_type = Timestamp)]
    pub updated_at: NaiveDateTime,
}

pub fn revision_to_revision_db(
    revision: Revision,
    created_at: NaiveDateTime,
    updated_at: NaiveDateTime,
) -> RevisionDb {
    RevisionDb {
        id: 0,
        previous_verification_hash: revision.previous_verification_hash,
        nonce: revision.nonce,
        local_timestamp: revision.local_timestamp.to_string(),
        revision_type: revision.revision_type,
        file_hash: revision.file_hash,
        content: revision.content,
        link_type: revision.link_type,
        link_require_indepth_verification: revision.link_require_indepth_verification,
        link_verification_hash: revision.link_verification_hash,
        link_uri: revision.link_uri,
        signature_data: revision.signature,
        signature_public_key: revision.signature_public_key,
        signature_wallet_address: revision.signature_wallet_address,
        signature_type: revision.signature_type,
        witness_merkle_root: revision.witness_merkle_root,
        witness_timestamp: if revision.witness_timestamp.is_some(){
            Some(revision.witness_timestamp.unwrap().to_string())
        }else{
            None
        },
        witness_network: revision.witness_network,
        witness_smart_contract_address: revision.witness_smart_contract_address,
        witness_transaction_hash: revision.witness_transaction_hash,
        witness_sender_account_address: revision.witness_sender_account_address,
        leaves: revision
            .leaves
            .map(|leaves| serde_json::to_string(&leaves).unwrap_or_else(|_| "[]".to_string())),
        created_at,
        updated_at,
    }
}

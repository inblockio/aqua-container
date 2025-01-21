use diesel::prelude::*;
use diesel::r2d2::{self, ConnectionManager};
use diesel::SqliteConnection;
use serde::{Deserialize, Serialize};
pub mod file;
pub mod input;
pub mod api;
pub mod share_data;
pub mod user_profiles;
use chrono::{DateTime, Utc};

pub type DB_POOL = r2d2::Pool<ConnectionManager<SqliteConnection>>;

use chrono::NaiveDateTime;
use diesel::expression::AsExpression;
use diesel::prelude::*;
use diesel::prelude::*;
use diesel::sql_types::Nullable;
use siwe::TimeStamp;

// #[derive(
//     Queryable,
//     Selectable,
//     Serialize,
//     Deserialize,
//     Debug,
//     Clone,
//     Insertable,
//     Identifiable,
//     AsChangeset,
// )]
// #[diesel(table_name = crate::schema::pages)]
// pub struct PagesTable {
//     pub id: i32,
//     pub name: String,
//     pub extension: String,
//     pub page_data: String,
//     pub owner: String,
//     pub mode: String,
//     pub created_at: String,
//     pub is_shared: bool,
// }

// #[derive(Debug, Serialize, Deserialize, Clone, Insertable)]
// #[diesel(table_name = crate::schema::pages)]
// pub struct NewPagesTable {
//     pub name: String,
//     pub extension: String,
//     pub page_data: String,
//     pub owner: String,
//     pub mode: String,
//     pub created_at: String,
//     pub is_shared: bool,
// }

// impl From<PagesTable> for NewPagesTable {
//     fn from(page: PagesTable) -> Self {
//         NewPagesTable {
//             name: page.name,
//             extension: page.extension,
//             page_data: page.page_data,
//             owner: page.owner,
//             mode: page.mode,
//             created_at: page.created_at,
//             is_shared: page.is_shared,
//         }
//     }
// }

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
#[diesel(table_name = crate::schema::share_data)]
#[diesel(check_for_backend(diesel::sqlite::Sqlite))]
pub struct ShareDataTable {
    pub id: Option<i32>,
    pub file_id: i32,
    pub identifier: String,
    pub created_time: String,
}

#[derive(Queryable, Selectable, Deserialize, Serialize, Debug, Clone, Insertable)]
#[diesel(table_name = crate::schema::user_profiles)]
#[diesel(check_for_backend(diesel::sqlite::Sqlite))]
pub struct UserProfilesTable {
    pub id: Option<i32>,
    pub address: String,
    pub chain: String,
    pub theme: String,
    pub contract_address: String,
    pub file_mode: String,
    pub domain_name: String,
}


#[derive(Queryable, Insertable, Serialize, Deserialize, Debug, Clone)]

#[diesel(table_name = crate::schema::aqua_chain)]
pub struct AquaChainDb {
    pub id: Option<i32>,
    pub file_hash: String,
    pub file_name: String,
    pub revisions: String, // Stored as a JSON string
    pub file_content: String,
    pub owner: String,
    pub mode: String,
    pub share_code: Option<String>,
    pub is_shared: bool,
    pub updated_at: NaiveDateTime,
    pub created_at: NaiveDateTime,
}

pub fn aqua_chain_to_aqua_chain_db(aqua_chain : AquaChain, file_hash: String, file_name: String, file_content: String, owner: String, mode: String) -> AquaChainDb {
    AquaChainDb {
        id: None,
        file_hash,
        file_name,
        revisions: serde_json::to_string(&self.revisions).unwrap(),
        file_content,
        owner,
        mode,
        share_code: None,
        is_shared: false,
        updated_at: Utc::now().naive_utc(),
        created_at: Utc::now().naive_utc(),
    }
}



#[derive(Queryable, Insertable, Serialize, Deserialize, Debug, Clone)]
#[table_name = "revisions"]
pub struct RevisionDb {
    pub id: Option<i32>,
    pub previous_verification_hash: String,
    pub nonce: String,
    pub local_timestamp: NaiveDateTime,
    pub revision_type: String,
    pub file_hash: Option<String>,
    pub content: Option<String>,
    pub link_type: Option<String>,
    pub link_require_indepth_verification: Option<bool>,
    pub link_verification_hash: Option<String>,
    pub link_uri: Option<String>,
    pub signature: Option<String>,
    pub signature_public_key: Option<String>,
    pub signature_wallet_address: Option<String>,
    pub signature_type: Option<String>,
    pub witness_merkle_root: Option<String>,
    pub witness_timestamp: Option<NaiveDateTime>,
    pub witness_network: Option<String>,
    pub witness_smart_contract_address: Option<String>,
    pub witness_transaction_hash: Option<String>,
    pub witness_sender_account_address: Option<String>,
    pub leaves: Option<String>, // Serialized as JSON string
    pub created_at: NaiveDateTime,
    pub updated_at: NaiveDateTime,
}

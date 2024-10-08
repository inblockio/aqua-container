use guardian_common::custom_types::HashChain;
use crate::models::file::FileInfo;
use serde::{Deserialize, Serialize};
use bonsaidb::core::schema::{Collection, SerializedCollection};
#[derive(Debug, Serialize, Deserialize, Collection, Clone)]
#[collection(name = "page")]
pub struct PageData {
    pub pages: Vec<HashChain>,
}
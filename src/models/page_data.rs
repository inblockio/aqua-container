use guardian_common::custom_types::HashChain;
use crate::models::file::FileInfo;
use serde::{Deserialize, Serialize};
use bonsaidb::core::schema::{Collection, SerializedCollection};
#[derive(Debug, Serialize, Deserialize, Collection, Clone)]
#[collection(name = "page")]
pub struct PageDataContainer {
    pub pages: Vec<HashChain>,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct ApiResponse {
    pub logs :  Vec<String>,
    pub file :  Option<FileInfo>,
    pub files : Vec<FileInfo>,
}


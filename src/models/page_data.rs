// use guardian_common::custom_types::HashChain;
use crate::models::file::FileInfo;
use serde::{Deserialize, Serialize};
use bonsaidb::core::schema::{Collection, SerializedCollection};

use aqua_verifier_rs_types::models::page_data::HashChain;

#[derive(Debug, Serialize, Deserialize, Collection, Clone)]
#[collection(name = "page")]
pub struct PageDataContainer<HashChain: std::marker::Sync + std::marker::Send> {
    pub pages: Vec<HashChain>,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct ApiResponse {
    pub logs :  Vec<String>,
    pub file :  Option<FileInfo>,
    pub files : Vec<FileInfo>,
}


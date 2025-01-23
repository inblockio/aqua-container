use aqua_verifier_rs_types::models::chain::AquaChain;
use serde::{Deserialize, Serialize};

use crate::models::{AquaChainDb, RevisionDb};

// use aqua_verifier_rs_types::models::page_data::HashChain;

// #[derive(Debug, Serialize, Deserialize, Clone)]
// pub struct PageDataContainer<HashChain: std::marker::Sync + std::marker::Send> {
//     pub pages: Vec<HashChain>,
// }

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct ApiResponse {
    pub logs: Vec<String>,
    pub chains: Vec<AquaChainDb>, 
    pub revisions: Option<RevisionDb>, 
}

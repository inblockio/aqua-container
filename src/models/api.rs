use aqua_verifier_rs_types::models::chain::AquaChain;
use serde::{Deserialize, Serialize};



#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct ApiResponse {
    pub logs: Vec<String>,
    pub chains: Vec<AquaChain>
}

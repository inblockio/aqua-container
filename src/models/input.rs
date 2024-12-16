use serde::{Deserialize, Serialize};

#[derive(Deserialize, Serialize, Debug)]
#[allow(dead_code)]
pub struct RevisionInput {
    pub file_id: i32,
    pub signature: String,
    pub publickey: String,
    pub wallet_address: String,
}

#[derive(Deserialize, Serialize, Debug)]
#[allow(dead_code)]
pub struct WitnessInput {
    pub file_id: i32,
    pub tx_hash: String,
    pub wallet_address: String,
    pub network: String,
}

#[derive(Deserialize, Serialize, Debug, Clone)]
#[allow(dead_code)]
pub struct DeleteInput {
    pub file_id: i32,
   
}



#[derive(Deserialize, Serialize, Debug)]
#[allow(dead_code)]
pub struct UpdateConfigurationInput {
    pub chain: String,
    pub domain: String,
    pub mode: String,   
    pub contract: String,   
}

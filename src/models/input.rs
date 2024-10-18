use serde::{Deserialize, Serialize};

#[derive(Deserialize, Serialize, Debug)]
#[allow(dead_code)]
pub struct RevisionInput {
    pub filename: String,
    pub signature: String,
    pub publickey: String,
    pub wallet_address: String,
}

#[derive(Deserialize, Serialize, Debug)]
#[allow(dead_code)]
pub struct WitnessInput {
    pub filename: String,
    pub tx_hash: String,
    pub wallet_address: String,
}

#[derive(Deserialize, Serialize, Debug)]
#[allow(dead_code)]
pub struct DeleteInput {
    pub filename: String,
   
}

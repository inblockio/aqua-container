use serde::{Deserialize, Serialize};
#[derive(Deserialize, Serialize, Debug)]
#[allow(dead_code)]
pub struct SInput {
   pub  filename: String,
    pub  signature: String,
    pub publickey: String,
    pub wallet_address: String,
}
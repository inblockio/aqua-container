use serde::{Deserialize, Serialize};
#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct FileInfo {
    pub id : i64,
    pub name : String,
    pub extension: String,
    pub page_data : String,
    pub mode : String,
    pub owner: String
}

#[derive(Debug,  Serialize, Deserialize, Clone)]
pub struct FileDataInformation {
   pub file_type: String,
   pub size_bytes: usize,
   pub  mime_type: String,
}
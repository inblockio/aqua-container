use serde::{Deserialize, Serialize};
#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct FileInfo {
    pub id : i64,
    pub name : String,
    pub extension: String,
    pub page_data : String,
}
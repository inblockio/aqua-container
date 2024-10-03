use serde::{Deserialize, Serialize};
#[derive(Debug, Serialize, Deserialize)]
pub struct FileInfo {
    id : i16,
    name : String,
    file_type: String,
    size : i16,
    page_data : String,
}
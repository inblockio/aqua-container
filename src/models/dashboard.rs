use crate::models::file::FileInfo;
use serde::{Deserialize, Serialize};
#[derive(Debug, Serialize, Deserialize)]
pub struct Dashboard {
    files : Vec<FileInfo>

}
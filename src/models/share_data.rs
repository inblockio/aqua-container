use serde::{Deserialize, Serialize};

use crate::db::pages_db::db_data;

use super::{file::FileInfo, ShareDataTable};

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct ShareDataResponse{

    pub logs :  Vec<String>,
    pub share_data : Option<ShareDataTable>,
    pub file_data : Option<db_data>,

}

#[derive(Deserialize, Serialize, Debug, Clone)]
#[allow(dead_code)]
pub struct CreateShareData {
    pub filename: String,
    pub identifier : String
}
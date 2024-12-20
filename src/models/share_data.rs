use serde::{Deserialize, Serialize};


use super::{file::FileInfo, PagesTable, ShareDataTable};

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct ShareDataResponse{

    pub logs :  Vec<String>,
    pub share_data : Option<ShareDataTable>,
    pub file_data : Option<PagesTable>,

}

#[derive(Deserialize, Serialize, Debug, Clone)]
#[allow(dead_code)]
pub struct CreateShareData {
    pub file_id: i32,
    pub identifier : String
}
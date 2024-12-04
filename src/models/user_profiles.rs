use serde::{Deserialize, Serialize};

use super::UserProfilesTable;


#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct UserProfileApiResponse {
    pub logs :  Vec<String>,
    pub user_profile : Option<UserProfilesTable>,
}
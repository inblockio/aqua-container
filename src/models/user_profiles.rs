use crate::models::database_models::SettingsTable;
use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct UseSettingsApiResponse {
    pub logs: Vec<String>,
    pub user_settings: Option<SettingsTable>,
}

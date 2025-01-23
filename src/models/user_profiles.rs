use serde::{Deserialize, Serialize};
use crate::models::database_models::SettingsTable;

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct UseSettingsApiResponse {
    pub logs: Vec<String>,
    pub user_settings: Option<SettingsTable>,
}

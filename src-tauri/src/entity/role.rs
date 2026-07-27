use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub(crate) struct Role {
    pub(crate) id: Option<i32>,
    pub(crate) account_id: Option<i32>,
    pub(crate) role_id: String,
    pub(crate) server: String,
    pub(crate) last_update_time: Option<String>,
}

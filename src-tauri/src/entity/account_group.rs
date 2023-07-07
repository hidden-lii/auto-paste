use serde::{Serialize, Deserialize};

#[derive(Debug, Serialize, Deserialize)]
pub(crate) struct AccountGroup {
    pub(crate) id: Option<i32>,
    pub(crate) account_id: i32,
    pub(crate) group_id: i32,
    pub(crate) last_update_time: Option<String>,
}
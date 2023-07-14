use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize)]
pub(crate) struct Account {
    pub(crate) id: Option<i32>,
    pub(crate) name: String,
    pub(crate) username: String,
    pub(crate) password: String,
    pub(crate) sequence: Option<i32>,
    pub(crate) liked: Option<bool>,
    pub(crate) description: Option<String>,
    pub(crate) last_update_time: Option<String>,
    // 外部字段
    pub(crate) account_category_ids: Option<Vec<i32>>,
}

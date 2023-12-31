use serde::{Serialize, Deserialize};

#[derive(Debug, Serialize, Deserialize)]
pub(crate) struct Category {
    pub(crate) id: Option<i32>,
    pub(crate) name: String,
    pub(crate) sequence: Option<i32>,
    pub(crate) last_update_time: Option<String>,
    // 外部字段
    pub(crate) account_ids: Option<Vec<i32>>,
}
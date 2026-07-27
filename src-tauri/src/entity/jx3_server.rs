use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub(crate) struct Jx3Server {
    pub(crate) id: Option<i32>,
    pub(crate) zone: String,
    pub(crate) server: String,
    pub(crate) status: String,
    pub(crate) last_update_time: Option<String>,
}

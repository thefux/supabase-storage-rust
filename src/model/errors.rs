use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Error {
    pub status_code: String,
    pub error: String,
    pub message: String,
}

use serde::{Deserialize, Serialize};
use serde_json::Value;

#[derive(Serialize)]
pub struct Request<'a> {
    pub jsonrpc: &'a str,
    pub method: String,
    pub params: Vec<Value>,
    pub id: String,
}

#[derive(Debug, Deserialize)]
pub struct Response {
    pub jsonrpc: String,
    pub id: String,
    pub result: Option<Value>,
    pub error: Option<Error>,
}

#[derive(Debug, Deserialize)]
pub struct Error {
    pub code: i32,
    pub message: String,
}

#[derive(Serialize, Debug, Default)]
#[serde(rename_all = "kebab-case")]
pub struct TaskOptions {
    pub dir: Option<String>,
    pub out: Option<String>,
    pub r#continue: Option<bool>,
    pub allow_overwrite: Option<bool>,
    pub auto_file_renaming: Option<bool>,
    pub rpc_save_upload_metadata: Option<bool>,
}

impl From<&TaskOptions> for Value {
    fn from(val: &TaskOptions) -> Self {
        serde_json::to_value(val).unwrap()
    }
}

#[derive(Deserialize, Debug)]
#[serde(rename_all = "lowercase")]
pub enum TaskStatus {
    Active,
    Waiting,
    Paused,
    Error,
    Complete,
    Removed,
}

#[derive(Deserialize, Debug)]
#[serde(rename_all = "camelCase")]
pub struct Status {
    pub gid: String,
    pub status: TaskStatus,
    pub total_length: String,
    pub completed_length: String,
    pub download_speed: String,
    pub error_code: Option<String>,
    pub error_message: Option<String>,
}

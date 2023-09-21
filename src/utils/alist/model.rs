use serde::{Deserialize, Serialize};

#[derive(Debug, Deserialize)]
pub(super) struct ResponseModel<T> {
    pub code: i32,
    pub message: String,
    pub data: Option<T>,
}

#[derive(Debug, Deserialize)]
pub struct File {
    pub name: String,
    #[serde(skip)]
    pub path: String,
    pub size: u64,
    pub is_dir: bool,
    pub modified: String,

    #[serde(default)]
    pub raw_url: String,
}

#[derive(Debug, Serialize)]
pub(super) struct ListRequest<'a> {
    pub path: &'a str,
    pub refresh: bool,
    pub page: u32,
    pub per_page: u32,
    pub password: &'a str,
}

#[derive(Debug, Deserialize)]
pub(super) struct ListResponse {
    pub content: Vec<File>,
}

#[derive(Debug, Serialize)]
pub(super) struct GetRequest<'a> {
    pub path: &'a str,
    pub password: &'a str,
}

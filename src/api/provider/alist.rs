use actix_web::{get, web};
use serde::{Deserialize, Serialize};
use utoipa::{IntoParams, ToSchema};

use crate::{
    api::error::{ok, ApiResult},
    common::state::AppState,
    utils::alist,
};

#[derive(Debug, Deserialize, IntoParams)]
pub struct ListFilesRequest {
    path: Option<String>,
}

#[derive(Debug, Serialize, ToSchema)]
#[schema(as = provider::alist::File)]
pub struct File {
    name: String,
    path: String,
    is_dir: bool,
}

#[utoipa::path(
    get,
    context_path = "/api",
    tag = "provider",
    params(
        ListFilesRequest
    ),
    responses(
        (status = 200, body = Vec<provider::alist::File>),
    )
)]
#[get("/provider/alist/files")]
pub async fn list_files(state: web::Data<AppState>, req: web::Query<ListFilesRequest>) -> ApiResult<Vec<File>> {
    let path = req.path.as_deref().unwrap_or("").trim();
    ok(alist::Client::try_from(state.as_ref())?
        .list(if path.is_empty() { "/" } else { path })
        .await?
        .iter()
        .map(|f| File {
            name: f.name.clone(),
            path: f.path.clone(),
            is_dir: f.is_dir,
        })
        .collect())
}

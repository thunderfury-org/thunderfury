use actix_web::{get, web};

use super::TvDetail;
use crate::{
    api::error::{ok, ApiResult},
    common::state::AppState,
    entity::tv::query,
};

#[utoipa::path(
    get,
    context_path = "/api",
    responses(
        (status = 200, body = Vec<TvDetail>),
    )
)]
#[get("/tvs")]
pub async fn list_tvs(state: web::Data<AppState>) -> ApiResult<Vec<TvDetail>> {
    ok(query::find_all_tvs(&state.db)
        .await?
        .into_iter()
        .map(Into::into)
        .collect())
}

use actix_web::{get, web};

use super::MovieDetail;
use crate::{
    api::error::{ok, ApiResult},
    common::state::AppState,
    entity::movie::query,
};

#[utoipa::path(
    get,
    context_path = "/api",
    responses(
        (status = 200, body = Vec<MovieDetail>),
    )
)]
#[get("/movies")]
pub async fn list_movies(state: web::Data<AppState>) -> ApiResult<Vec<MovieDetail>> {
    ok(query::find_all_movies(&state.db)
        .await?
        .into_iter()
        .map(Into::into)
        .collect())
}

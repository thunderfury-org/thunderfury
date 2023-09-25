use actix_web::{post, web};

use crate::{
    api::{
        error::{ok, ApiResult},
        model::EmptyResponse,
    },
    common::state::AppState,
    entity::subscription::update,
};

#[utoipa::path(
    post,
    context_path = "/api",
    responses(
        (status = 200),
    )
)]
#[post("/subscriptions/{id}/run")]
pub async fn run_subscription(state: web::Data<AppState>, request: web::Path<(i32,)>) -> ApiResult<EmptyResponse> {
    update::mark_run_immediately(&state.db, request.0).await?;
    ok(EmptyResponse {})
}

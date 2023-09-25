use actix_web::{get, web};

use super::SubscriptionDetail;
use crate::{
    api::error::{ok, ApiResult},
    common::state::AppState,
    entity::subscription::query,
};

#[utoipa::path(
    get,
    context_path = "/api",
    responses(
        (status = 200, body = Vec<SubscriptionDetail>),
    )
)]
#[get("/subscriptions")]
pub async fn list_subscriptions(state: web::Data<AppState>) -> ApiResult<Vec<SubscriptionDetail>> {
    ok(query::find_all_subscriptions(&state.db)
        .await?
        .into_iter()
        .map(Into::into)
        .collect())
}

use actix_web::{get, post, web};
use chrono::Utc;
use sea_orm::{ColumnTrait, EntityTrait, QueryFilter, Set};
use url::Url;

use crate::{
    api::{
        error::{ok, ApiResult},
        model::{
            subscription::{NewSubscriptionRequest, SubscriptionDetail},
            EmptyResponse,
        },
    },
    common::{error::Error, state::AppState},
    entity::{enums::Provider, subscription},
    subscription::{create, query},
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
    ok(query::find_subscriptions(&state.db)
        .await?
        .into_iter()
        .map(Into::into)
        .collect())
}

#[utoipa::path(
    post,
    context_path = "/api",
    request_body = NewSubscriptionRequest,
    responses(
        (status = 200, body = SubscriptionDetail),
    )
)]
#[post("/subscriptions")]
pub async fn new_subscription(
    state: web::Data<AppState>,
    request: web::Json<NewSubscriptionRequest>,
) -> ApiResult<SubscriptionDetail> {
    if let Some(s) = request.season_number {
        if s < 1 {
            return Err(Error::InvalidArgument(
                "season number must be greater than 0".to_string(),
            ));
        }
    }

    let resource_provider: Provider = request.resource_provider.trim().try_into()?;
    let resource_url = request.resource_url.as_ref().unwrap().trim();
    match resource_provider {
        Provider::Rss => match Url::parse(resource_url) {
            Ok(_) => (),
            Err(_) => return Err(Error::InvalidArgument(format!("invalid rss url: {}", resource_url))),
        },
        Provider::Alist => {
            if !resource_url.starts_with('/') {
                return Err(Error::InvalidArgument(format!(
                    "invalid alist dir path: {}",
                    resource_url
                )));
            }
        }
    };

    let req = subscription::Model {
        media_type: request.media_type.as_str().try_into()?,
        tmdb_id: request.tmdb_id,
        resource_provider,
        resource_url: Some(resource_url.to_owned()),
        season_number: request.season_number,
        resolutions: request.resolutions.clone().map(|v| (v).into()),
        subtitles: request.subtitles.clone().map(|v| v.into()),
        ..Default::default()
    };

    ok(create::create_subscription(state.as_ref(), &req).await?.into())
}

#[utoipa::path(
    post,
    context_path = "/api",
    responses(
        (status = 200),
    )
)]
#[post("/subscriptions/{id}/run")]
pub async fn run_subscription(state: web::Data<AppState>, request: web::Path<(u32,)>) -> ApiResult<EmptyResponse> {
    subscription::Entity::update(subscription::ActiveModel {
        id: Set(request.0),
        next_run_time: Set(Utc::now()),
        ..Default::default()
    })
    .filter(subscription::Column::Status.eq(subscription::Status::Running))
    .exec(&state.db)
    .await?;

    ok(EmptyResponse {})
}

impl From<subscription::Model> for SubscriptionDetail {
    fn from(val: subscription::Model) -> Self {
        Self {
            id: val.id,
            media_type: val.media_type.to_string(),
            media_id: val.media_id,
            resource_provider: val.resource_provider.to_string(),
            resource_url: val.resource_url.clone(),
            season_number: val.season_number,
            resolutions: val.resolutions.map(|v| v.0),
            subtitles: val.subtitles.map(|v| v.0),
            status: val.status.to_string(),
            tv_detail: None,
            movie_detail: None,
        }
    }
}

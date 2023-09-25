use std::collections::HashMap;

use actix_web::{get, web};

use super::SubscriptionDetail;
use crate::{
    api::error::{ok, ApiResult},
    common::{enums::MediaType, state::AppState},
    entity::{
        movie,
        subscription::{query, Model},
        tv,
    },
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
    let subs = query::find_all_subscriptions(&state.db).await?;

    let movies = movie::query::find_movies_by_ids(
        &state.db,
        subs.iter()
            .filter(|s| s.media_type == MediaType::Movie)
            .map(|s| s.media_id)
            .collect(),
    )
    .await?;

    let tvs = tv::query::find_tvs_by_ids(
        &state.db,
        subs.iter()
            .filter(|s| s.media_type == MediaType::Tv)
            .map(|s| s.media_id)
            .collect(),
    )
    .await?;

    ok(convert_subscriptions(subs, movies, tvs))
}

fn convert_subscriptions(subs: Vec<Model>, movies: Vec<movie::Model>, tvs: Vec<tv::Model>) -> Vec<SubscriptionDetail> {
    let movie_map: HashMap<i32, &movie::Model> = movies.iter().map(|m| (m.id, m)).collect();
    let tv_map: HashMap<i32, &tv::Model> = tvs.iter().map(|m| (m.id, m)).collect();

    subs.into_iter()
        .map(|s| {
            let mut res: SubscriptionDetail = s.into();
            match res.media_type {
                MediaType::Movie => res.movie_detail = movie_map.get(&res.media_id).map(|m| (*m).clone().into()),
                MediaType::Tv => res.tv_detail = tv_map.get(&res.media_id).map(|m| (*m).clone().into()),
            }
            res
        })
        .collect()
}

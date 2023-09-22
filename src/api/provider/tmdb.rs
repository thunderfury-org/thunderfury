use actix_web::{get, web};
use serde::{Deserialize, Serialize};
use utoipa::{IntoParams, ToSchema};

use crate::{
    api::error::{ok, ApiResult},
    common::{error::Error, state::AppState},
    utils::tmdb,
};

#[derive(Debug, Deserialize, IntoParams)]
pub struct SearchRequest {
    query: String,
}

#[derive(Debug, Serialize, ToSchema)]
#[schema(as = provider::tmdb::MediaInfo)]
pub struct MediaInfo {
    media_type: String,
    tmdb_id: i32,
    name: String,
    year: String,
    overview: String,
    poster_path: String,
    backdrop_path: String,
}

#[utoipa::path(
    get,
    context_path = "/api",
    tag = "provider",
    params(
        SearchRequest
    ),
    responses(
        (status = 200, body = Vec<provider::tmdb::MediaInfo>),
    )
)]
#[get("/provider/tmdb/tv/search")]
pub async fn search_tv(state: web::Data<AppState>, req: web::Query<SearchRequest>) -> ApiResult<Vec<MediaInfo>> {
    let query = req.query.trim();
    if query.is_empty() {
        return Err(Error::InvalidArgument("query can't be empty".to_string()));
    }

    let results = tmdb::Client::try_from(state.as_ref())?.search_tv(query).await?;

    ok(results
        .iter()
        .map(|r| MediaInfo {
            media_type: "tv".to_owned(),
            tmdb_id: r.id,
            name: r.name.to_owned(),
            year: r.first_air_date.split('-').next().unwrap().to_owned(),
            overview: r.overview.to_owned(),
            poster_path: r.poster_path.to_owned(),
            backdrop_path: r.backdrop_path.to_owned(),
        })
        .collect())
}

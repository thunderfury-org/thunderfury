use serde::{Deserialize, Serialize};
use utoipa::ToSchema;

use super::library::{MovieDetail, TvDetail};

#[derive(Debug, Serialize, ToSchema)]
pub struct SubscriptionDetail {
    pub id: i32,
    pub media_type: String,
    pub media_id: i32,
    pub resource_provider: String,
    pub resource_url: Option<String>,
    pub season_number: Option<i32>,
    pub resolutions: Option<Vec<String>>,
    pub subtitles: Option<Vec<String>>,
    pub status: String,
    pub tv_detail: Option<TvDetail>,
    pub movie_detail: Option<MovieDetail>,
}

#[derive(Debug, Deserialize, ToSchema)]
pub struct NewSubscriptionRequest {
    pub media_type: String,
    pub tmdb_id: i32,
    pub resource_provider: String,
    pub resource_url: Option<String>,
    pub season_number: Option<i32>,
    pub resolutions: Option<Vec<String>>,
    pub subtitles: Option<Vec<String>>,
}

use serde::{Deserialize, Serialize};
use utoipa::ToSchema;

use super::library::{MovieDetail, TvDetail};

#[derive(Debug, Serialize, ToSchema)]
pub struct SubscriptionDetail {
    pub id: u32,
    pub media_type: String,
    pub media_id: u32,
    pub resource_provider: String,
    pub resource_url: Option<String>,
    pub season_number: Option<u32>,
    pub resolutions: Option<Vec<String>>,
    pub subtitles: Option<Vec<String>>,
    pub status: String,
    pub tv_detail: Option<TvDetail>,
    pub movie_detail: Option<MovieDetail>,
}

#[derive(Debug, Deserialize, ToSchema)]
pub struct NewSubscriptionRequest {
    pub media_type: String,
    pub tmdb_id: u32,
    pub resource_provider: String,
    pub resource_url: Option<String>,
    pub season_number: Option<u32>,
    pub resolutions: Option<Vec<String>>,
    pub subtitles: Option<Vec<String>>,
}

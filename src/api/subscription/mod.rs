use serde::{Deserialize, Serialize};
use utoipa::ToSchema;

use super::{movie::MovieDetail, tv::TvDetail};
use crate::{
    common::{
        enums::{MediaType, Provider},
        types::StringVec,
    },
    entity::subscription,
};

pub mod create;
pub mod query;
pub mod run;

#[derive(Debug, Serialize, ToSchema)]
pub struct SubscriptionDetail {
    pub id: i32,
    pub media_type: MediaType,
    pub media_id: i32,
    pub resource_provider: Provider,
    pub resource_url: Option<String>,
    pub season_number: Option<i32>,
    pub resolutions: Option<StringVec>,
    pub subtitles: Option<StringVec>,
    pub status: String,
    pub tv_detail: Option<TvDetail>,
    pub movie_detail: Option<MovieDetail>,
}

#[derive(Debug, Deserialize, ToSchema)]
pub struct NewSubscriptionRequest {
    pub media_type: MediaType,
    pub tmdb_id: i32,
    pub resource_provider: Provider,
    pub resource_url: Option<String>,
    pub season_number: Option<i32>,
    pub resolutions: Option<StringVec>,
    pub subtitles: Option<StringVec>,
}

impl From<subscription::Model> for SubscriptionDetail {
    fn from(val: subscription::Model) -> Self {
        Self {
            id: val.id,
            media_type: val.media_type,
            media_id: val.media_id,
            resource_provider: val.resource_provider,
            resource_url: val.resource_url.clone(),
            season_number: val.season_number,
            resolutions: val.resolutions,
            subtitles: val.subtitles,
            status: val.status.to_string(),
            tv_detail: None,
            movie_detail: None,
        }
    }
}

impl NewSubscriptionRequest {
    pub fn unique_id(&self) -> String {
        let parts = vec![
            self.media_type.to_string(),
            self.tmdb_id.to_string(),
            self.resource_provider.to_string(),
            self.resource_url.as_ref().map_or("".to_owned(), |s| s.to_string()),
            self.season_number.map_or("".to_owned(), |n| n.to_string()),
        ];
        sha256::digest(parts.join(":"))
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn unique_id() {
        let request = NewSubscriptionRequest {
            media_type: MediaType::Movie,
            tmdb_id: 1,
            resource_provider: Provider::Rss,
            resource_url: None,
            season_number: None,
            resolutions: None,
            subtitles: None,
        };

        assert_eq!(
            request.unique_id(),
            "847260174378ac6ca75ba961da71350ddca05beaada842704dc45025cf0ac3fe"
        );
    }
}

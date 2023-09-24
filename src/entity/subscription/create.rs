use chrono::Utc;
use sea_orm::{ActiveModelTrait, ActiveValue::NotSet, ConnectionTrait, Set};

use super::{ActiveModel, Model, Status};
use crate::common::{
    enums::{MediaType, Provider},
    error::Result,
    types::StringVec,
};

pub struct NewSubscription {
    pub unique_id: String,
    pub media_type: MediaType,
    pub media_id: i32,
    pub tmdb_id: i32,
    pub resource_provider: Provider,
    pub resource_url: Option<String>,
    pub season_number: Option<i32>,
    pub resolutions: Option<StringVec>,
    pub subtitles: Option<StringVec>,
}

pub async fn create_subscription<C>(db: &C, new_sub: NewSubscription) -> Result<Model>
where
    C: ConnectionTrait,
{
    Ok(ActiveModel {
        unique_id: Set(new_sub.unique_id),
        media_type: Set(new_sub.media_type),
        media_id: Set(new_sub.media_id),
        tmdb_id: Set(new_sub.tmdb_id),
        resource_provider: Set(new_sub.resource_provider),
        resource_url: Set(new_sub.resource_url),
        season_number: Set(new_sub.season_number),
        resolutions: Set(new_sub.resolutions),
        subtitles: Set(new_sub.subtitles),
        status: Set(Status::Running),
        last_run_time: NotSet,
        next_run_time: Set(Utc::now()),
        create_time: Set(Utc::now()),
        ..Default::default()
    }
    .insert(db)
    .await?)
}

use chrono::Utc;
use sea_orm::{ActiveModelTrait, ConnectionTrait, Set};

use crate::common::error::Result;

use super::{ActiveModel, Model};

pub struct NewSeason {
    pub tv_id: i32,
    pub season_number: i32,
    pub air_date: String,
    pub number_of_episodes: i32,
    pub overview: String,
    pub poster_path: String,
}

pub async fn create_season<C>(db: &C, new_season: NewSeason) -> Result<Model>
where
    C: ConnectionTrait,
{
    Ok(ActiveModel {
        tv_id: Set(new_season.tv_id),
        season_number: Set(new_season.season_number),
        air_date: Set(new_season.air_date),
        number_of_episodes: Set(new_season.number_of_episodes),
        overview: Set(new_season.overview),
        poster_path: Set(new_season.poster_path),
        create_time: Set(Utc::now()),
        ..Default::default()
    }
    .insert(db)
    .await?)
}

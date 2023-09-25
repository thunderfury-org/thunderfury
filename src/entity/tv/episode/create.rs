use chrono::Utc;
use sea_orm::{ConnectionTrait, EntityTrait, Set};

use crate::common::error::Result;

use super::{ActiveModel, Entity, Status};

pub struct NewEpisode<'a> {
    pub tv_id: i32,
    pub season_number: i32,
    pub episode_number: i32,
    pub name: &'a str,
    pub air_date: &'a str,
    pub overview: &'a str,
    pub still_path: &'a str,
}

pub async fn batch_create_episodes<C>(db: &C, new_episodes: Vec<NewEpisode<'_>>) -> Result<()>
where
    C: ConnectionTrait,
{
    Entity::insert_many(new_episodes.iter().map(|e| ActiveModel {
        tv_id: Set(e.tv_id),
        season_number: Set(e.season_number),
        episode_number: Set(e.episode_number),
        name: Set(e.name.to_owned()),
        air_date: Set(e.air_date.to_owned()),
        status: Set(Status::Waiting),
        overview: Set(e.overview.to_owned()),
        still_path: Set(e.still_path.to_owned()),
        create_time: Set(Utc::now()),
        ..Default::default()
    }))
    .exec(db)
    .await?;

    Ok(())
}

use chrono::Utc;
use sea_orm::{ColumnTrait, ConnectionTrait, EntityTrait, QueryFilter, Set};

use super::{ActiveModel, Column, Entity, Status};
use crate::common::error::Result;

pub async fn update_status_to_waiting<C>(db: &C, tv_id: i32, season_number: i32, episode_number: i32) -> Result<()>
where
    C: ConnectionTrait,
{
    Entity::update_many()
        .set(ActiveModel {
            status: Set(Status::Waiting),
            ..Default::default()
        })
        .filter(Column::TvId.eq(tv_id))
        .filter(Column::SeasonNumber.eq(season_number))
        .filter(Column::EpisodeNumber.eq(episode_number))
        .filter(Column::Status.eq(Status::Downloading))
        .exec(db)
        .await?;

    Ok(())
}

pub async fn update_status_to_queued<C>(db: &C, tv_id: i32, season_number: i32, episode_number: i32) -> Result<u64>
where
    C: ConnectionTrait,
{
    let result = Entity::update_many()
        .set(ActiveModel {
            status: Set(Status::Queued),
            ..Default::default()
        })
        .filter(Column::TvId.eq(tv_id))
        .filter(Column::SeasonNumber.eq(season_number))
        .filter(Column::EpisodeNumber.eq(episode_number))
        .filter(Column::Status.eq(Status::Waiting))
        .exec(db)
        .await?;

    Ok(result.rows_affected)
}

pub async fn update_status_to_downloading<C>(
    db: &C,
    tv_id: i32,
    season_number: i32,
    episode_number: i32,
    gid: &str,
) -> Result<()>
where
    C: ConnectionTrait,
{
    Entity::update_many()
        .set(ActiveModel {
            status: Set(Status::Downloading),
            external_task_id: Set(Some(gid.to_owned())),
            ..Default::default()
        })
        .filter(Column::TvId.eq(tv_id))
        .filter(Column::SeasonNumber.eq(season_number))
        .filter(Column::EpisodeNumber.eq(episode_number))
        .filter(Column::Status.eq(Status::Queued))
        .exec(db)
        .await?;

    Ok(())
}

pub async fn update_status_to_downloaded<C>(
    db: &C,
    tv_id: i32,
    season_number: i32,
    episode_number: i32,
    episode_file_path: &str,
) -> Result<()>
where
    C: ConnectionTrait,
{
    Entity::update_many()
        .set(ActiveModel {
            status: Set(Status::Downloaded),
            download_time: Set(Some(Utc::now())),
            file_path: Set(Some(episode_file_path.to_owned())),
            ..Default::default()
        })
        .filter(Column::TvId.eq(tv_id))
        .filter(Column::SeasonNumber.eq(season_number))
        .filter(Column::EpisodeNumber.eq(episode_number))
        .filter(Column::Status.eq(Status::Downloading))
        .exec(db)
        .await?;

    Ok(())
}

pub async fn update_external_task_id<C>(
    db: &C,
    tv_id: i32,
    season_number: i32,
    episode_number: i32,
    gid: &str,
) -> Result<()>
where
    C: ConnectionTrait,
{
    Entity::update_many()
        .set(ActiveModel {
            external_task_id: Set(Some(gid.to_owned())),
            ..Default::default()
        })
        .filter(Column::TvId.eq(tv_id))
        .filter(Column::SeasonNumber.eq(season_number))
        .filter(Column::EpisodeNumber.eq(episode_number))
        .filter(Column::Status.eq(Status::Downloading))
        .exec(db)
        .await?;

    Ok(())
}

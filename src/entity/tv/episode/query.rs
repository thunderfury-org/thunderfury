use sea_orm::{ColumnTrait, ConnectionTrait, EntityTrait, PaginatorTrait, QueryFilter, QuerySelect};

use crate::common::error::Result;

use super::{Column, Entity, Status};

pub async fn count_downloaded_episode_of_season<C>(db: &C, tv_id: i32, season_number: i32) -> Result<i32>
where
    C: ConnectionTrait,
{
    Ok(Entity::find()
        .filter(Column::TvId.eq(tv_id))
        .filter(Column::SeasonNumber.eq(season_number))
        .filter(Column::Status.eq(Status::Downloaded))
        .count(db)
        .await? as i32)
}

pub async fn find_waiting_episode_numbers_by_season_numbers<C>(
    db: &C,
    tv_id: i32,
    season_numbers: Vec<i32>,
) -> Result<Vec<(i32, i32)>>
where
    C: ConnectionTrait,
{
    Ok(Entity::find()
        .select_only()
        .column(Column::SeasonNumber)
        .column(Column::EpisodeNumber)
        .filter(Column::TvId.eq(tv_id))
        .filter(Column::SeasonNumber.is_in(season_numbers))
        .filter(Column::Status.eq(Status::Waiting))
        .into_tuple()
        .all(db)
        .await?)
}

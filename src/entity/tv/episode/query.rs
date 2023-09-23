use sea_orm::{ColumnTrait, ConnectionTrait, EntityTrait, PaginatorTrait, QueryFilter};

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

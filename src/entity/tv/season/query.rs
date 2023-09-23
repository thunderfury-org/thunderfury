use sea_orm::{ColumnTrait, ConnectionTrait, EntityTrait, QueryFilter, QuerySelect};

use crate::common::error::{Error, NotFoundCode, Result};

use super::{Column, Entity};

pub async fn get_season_episode_number<C>(db: &C, tv_id: i32, season_number: i32) -> Result<i32>
where
    C: ConnectionTrait,
{
    let res = Entity::find()
        .select_only()
        .column(Column::NumberOfEpisodes)
        .filter(Column::TvId.eq(tv_id))
        .filter(Column::SeasonNumber.eq(season_number))
        .into_tuple()
        .one(db)
        .await?;

    res.ok_or(Error::NotFound(
        NotFoundCode::Season,
        format!("tv {tv_id} season {season_number} episode number not found"),
    ))
}

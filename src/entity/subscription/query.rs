use sea_orm::{ColumnTrait, ConnectionTrait, EntityTrait, QueryFilter, QuerySelect};

use super::{Column, Entity, Status};
use crate::common::{enums::MediaType, error::Result};

pub async fn get_all_running_tv_subscription_ids<C>(db: &C, tv_id: i32, season_number: i32) -> Result<Vec<i32>>
where
    C: ConnectionTrait,
{
    Ok(Entity::find()
        .select_only()
        .column(Column::Id)
        .filter(Column::MediaType.eq(MediaType::Tv))
        .filter(Column::MediaId.eq(tv_id))
        .filter(Column::SeasonNumber.eq(season_number))
        .filter(Column::Status.eq(Status::Running))
        .into_tuple()
        .all(db)
        .await?)
}

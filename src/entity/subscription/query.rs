use chrono::Utc;
use sea_orm::{ColumnTrait, ConnectionTrait, EntityTrait, QueryFilter, QueryOrder, QuerySelect};

use super::{Column, Entity, Model, Status};
use crate::common::{enums::MediaType, error::Result};

pub async fn get_by_unique_id<C>(db: &C, unique_id: &str) -> Result<Option<Model>>
where
    C: ConnectionTrait,
{
    Ok(Entity::find().filter(Column::UniqueId.eq(unique_id)).one(db).await?)
}

pub async fn find_all_subscriptions<C>(db: &C) -> Result<Vec<Model>>
where
    C: ConnectionTrait,
{
    Ok(Entity::find().order_by_desc(Column::Id).all(db).await?)
}

pub async fn find_all_subscriptions_need_to_run<C>(db: &C) -> Result<Vec<Model>>
where
    C: ConnectionTrait,
{
    Ok(Entity::find()
        .filter(Column::Status.eq(Status::Running))
        .filter(Column::NextRunTime.lte(Utc::now()))
        .all(db)
        .await?)
}

pub async fn find_all_running_tv_subscription_ids<C>(db: &C, tv_id: i32, season_number: i32) -> Result<Vec<i32>>
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

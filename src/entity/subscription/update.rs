use chrono::{Duration, Utc};
use sea_orm::{ColumnTrait, ConnectionTrait, EntityTrait, QueryFilter, Set};

use super::{ActiveModel, Column, Entity, Status};
use crate::common::error::Result;

pub async fn mark_subscriptions_done<C>(db: &C, sub_ids: Vec<i32>) -> Result<()>
where
    C: ConnectionTrait,
{
    Entity::update_many()
        .set(ActiveModel {
            status: Set(Status::Done),
            finish_time: Set(Some(Utc::now())),
            ..Default::default()
        })
        .filter(Column::Id.is_in(sub_ids))
        .filter(Column::Status.eq(Status::Running))
        .exec(db)
        .await?;

    Ok(())
}

pub async fn update_next_run_time<C>(db: &C, sub_id: i32, period: Duration) -> Result<()>
where
    C: ConnectionTrait,
{
    Entity::update(ActiveModel {
        id: Set(sub_id),
        last_run_time: Set(Some(Utc::now())),
        next_run_time: Set(Utc::now() + period),
        ..Default::default()
    })
    .exec(db)
    .await?;

    Ok(())
}

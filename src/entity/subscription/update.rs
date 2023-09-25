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

pub async fn mark_run_finished<C>(db: &C, sub_id: i32, next_run_after: Duration) -> Result<()>
where
    C: ConnectionTrait,
{
    Entity::update(ActiveModel {
        id: Set(sub_id),
        last_run_time: Set(Some(Utc::now())),
        next_run_time: Set(Utc::now() + next_run_after),
        ..Default::default()
    })
    .filter(Column::Status.eq(Status::Running))
    .exec(db)
    .await?;

    Ok(())
}

pub async fn mark_run_immediately<C>(db: &C, sub_id: i32) -> Result<()>
where
    C: ConnectionTrait,
{
    Entity::update(ActiveModel {
        id: Set(sub_id),
        next_run_time: Set(Utc::now()),
        ..Default::default()
    })
    .filter(Column::Status.eq(Status::Running))
    .exec(db)
    .await?;

    Ok(())
}

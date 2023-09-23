use chrono::Utc;
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

use chrono::Utc;
use sea_orm::{ColumnTrait, Condition, ConnectionTrait, EntityTrait, QueryFilter, QueryOrder};

use crate::common::error::Result;

use super::{Action, Column, Entity, Model, Status};

pub async fn find_all_tasks_need_run<C>(db: &C, action: Action) -> Result<Vec<Model>>
where
    C: ConnectionTrait,
{
    Ok(Entity::find()
        .filter(Column::Action.eq(action))
        .filter(Column::Status.is_in([Status::Queued, Status::Running]))
        .filter(
            Condition::any()
                .add(Column::NextRetryTime.is_null())
                .add(Column::NextRetryTime.lte(Utc::now())),
        )
        .order_by_asc(Column::Id)
        .all(db)
        .await?)
}

use std::ops::Add;

use chrono::Utc;
use sea_orm::{ColumnTrait, ConnectionTrait, EntityTrait, QueryFilter, Set};

use crate::{common::error::Result, entity::task};

pub async fn update_status_to_running<C>(db: &C, task_id: i32, gid: &str) -> Result<()>
where
    C: ConnectionTrait,
{
    task::Entity::update(task::ActiveModel {
        id: Set(task_id),
        status: Set(task::Status::Running),
        external_task_id: Set(Some(gid.to_owned())),
        begin_time: Set(Some(Utc::now())),
        ..Default::default()
    })
    .filter(task::Column::Status.eq(task::Status::Queued))
    .exec(db)
    .await?;

    Ok(())
}

pub async fn update_external_task_id<C>(db: &C, task_id: i32, gid: &str) -> Result<()>
where
    C: ConnectionTrait,
{
    task::Entity::update(task::ActiveModel {
        id: Set(task_id),
        external_task_id: Set(Some(gid.to_owned())),
        ..Default::default()
    })
    .filter(task::Column::Status.eq(task::Status::Running))
    .exec(db)
    .await?;

    Ok(())
}

pub async fn update_status_to_done<C>(db: &C, task_id: i32) -> Result<()>
where
    C: ConnectionTrait,
{
    task::Entity::update(task::ActiveModel {
        id: Set(task_id),
        status: Set(task::Status::Done),
        end_time: Set(Some(Utc::now())),
        error_msg: Set(None),
        ..Default::default()
    })
    .filter(task::Column::Status.eq(task::Status::Running))
    .exec(db)
    .await?;

    Ok(())
}

pub async fn update_status_to_failed<C>(db: &C, task_id: i32, error_msg: &str) -> Result<()>
where
    C: ConnectionTrait,
{
    task::Entity::update(task::ActiveModel {
        id: Set(task_id),
        status: Set(task::Status::Failed),
        end_time: Set(Some(Utc::now())),
        error_msg: Set(Some(error_msg.to_owned())),
        ..Default::default()
    })
    .filter(task::Column::Status.eq(task::Status::Running))
    .exec(db)
    .await?;

    Ok(())
}

pub async fn update_status_to_retry<C>(db: &C, task_id: i32, retry_count: i32, error_msg: &str) -> Result<()>
where
    C: ConnectionTrait,
{
    task::Entity::update(task::ActiveModel {
        id: Set(task_id),
        status: Set(task::Status::Queued),
        error_msg: Set(Some(error_msg.to_owned())),
        retry_count: Set(Some(retry_count)),
        next_retry_time: Set(Some(
            Utc::now().add(chrono::Duration::seconds((retry_count * 60).into())),
        )),
        ..Default::default()
    })
    .filter(task::Column::Status.eq(task::Status::Running))
    .exec(db)
    .await?;

    Ok(())
}

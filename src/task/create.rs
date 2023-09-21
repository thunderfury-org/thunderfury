use chrono::Utc;
use sea_orm::{ActiveModelTrait, ConnectionTrait, Set};
use serde::Serialize;

use super::param::{DownloadMediaFileParam, PushMessageParam};
use crate::{
    common::error::Result,
    entity::task::{self, Action},
};

async fn create_task<C, T>(db: &C, action: Action, param: &T) -> Result<()>
where
    C: ConnectionTrait,
    T: Serialize,
{
    let param_string = serde_json::to_string(param)?;

    task::ActiveModel {
        action: Set(action),
        status: Set(task::Status::Queued),
        param: Set(param_string),
        create_time: Set(Utc::now()),
        ..Default::default()
    }
    .insert(db)
    .await?;

    Ok(())
}

pub async fn create_download_media_file_task<C>(db: &C, task_param: &DownloadMediaFileParam) -> Result<()>
where
    C: ConnectionTrait,
{
    create_task(db, Action::DownloadMediaFile, task_param).await
}

pub async fn create_send_message_task<C>(db: &C, message: &PushMessageParam) -> Result<()>
where
    C: ConnectionTrait,
{
    create_task(db, Action::PushMessage, message).await
}

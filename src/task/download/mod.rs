use std::collections::HashMap;

use sea_orm::{
    ColumnTrait, DatabaseConnection, DatabaseTransaction, EntityTrait, QueryFilter, QueryOrder, TransactionTrait,
};
use tracing::{error, info};

use crate::{
    common::{
        error::{Error, Result},
        state::AppState,
    },
    entity::{enums, task},
    task::{
        create,
        param::{DownloadMediaFileParam, PushMessageParam},
        status,
    },
    utils::aria2,
};

mod dir;
mod media;
mod submit;
mod subscription;

lazy_static::lazy_static!(
    static ref DOWNLOAD_TASK_LIMIT_MAP: HashMap<enums::Downloader, u32> = HashMap::from([
        (enums::Downloader::Alist, 2),
        (enums::Downloader::Bt, 5),
    ]);
);

pub async fn find_download_tasks_not_done(db: &DatabaseConnection) -> Result<Vec<task::Model>> {
    Ok(task::Entity::find()
        .filter(task::Column::Action.eq(task::Action::DownloadMediaFile))
        .filter(task::Column::Status.is_in([task::Status::Queued, task::Status::Running]))
        .order_by_asc(task::Column::Id)
        .all(db)
        .await?)
}

pub async fn run_tasks(state: &AppState, tasks: Vec<task::Model>) {
    let mut current_downloading_count_map: HashMap<enums::Downloader, u32> =
        DOWNLOAD_TASK_LIMIT_MAP.keys().map(|d| (*d, 0)).collect();

    for t in &tasks {
        let task_param: DownloadMediaFileParam = t.deserialize_param().unwrap();
        let current = current_downloading_count_map[&task_param.file_downloader];

        if current >= DOWNLOAD_TASK_LIMIT_MAP[&task_param.file_downloader] {
            continue;
        }

        match run_one_task(state, t, &task_param).await {
            Ok(downloading) => {
                if downloading {
                    current_downloading_count_map.insert(task_param.file_downloader, current + 1);
                }
            }
            Err(e) => {
                error!(task_id = t.id, "run download task error: {}", e);
                break;
            }
        }
    }
}

async fn run_one_task(state: &AppState, t: &task::Model, task_param: &DownloadMediaFileParam) -> Result<bool> {
    match t.status {
        task::Status::Queued => Ok(handle_queued_task(state, t.id, task_param).await?),
        task::Status::Running => Ok(handle_running_task(state, t, task_param).await?),
        task::Status::Done | task::Status::Failed => {
            error!(task_id = t.id, "task alread done or failed, shoud not go to here");
            Ok(false)
        }
    }
}

async fn handle_queued_task(state: &AppState, task_id: u32, task_param: &DownloadMediaFileParam) -> Result<bool> {
    info!(task_id, "submit queued task to downloader, {:?}", task_param);

    let gid = submit::submit_task(state, task_param).await?;

    let txn = state.db.begin().await?;
    status::update_status_to_running(&txn, task_id, &gid).await?;
    media::update_status_to_downloading(&txn, task_param, &gid).await?;
    txn.commit().await?;

    Ok(true)
}

async fn handle_running_task(state: &AppState, t: &task::Model, task_param: &DownloadMediaFileParam) -> Result<bool> {
    let res = aria2::Client::try_from(state)?
        .tell_status(t.external_task_id.as_ref().unwrap())
        .await;
    match res {
        Err(Error::NotFound(_, _)) => {
            info!(
                task_id = t.id,
                "task not found in downloader, submit again, {:?}", task_param
            );

            let gid = submit::submit_task(state, task_param).await?;

            let txn = state.db.begin().await?;
            status::update_external_task_id(&txn, t.id, &gid).await?;
            media::update_external_task_id(&txn, task_param, &gid).await?;
            txn.commit().await?;

            Ok(true)
        }
        Err(e) => Err(e),
        Ok(s) => handle_aria2_task_status(state, t.id, task_param, s).await,
    }
}

async fn handle_aria2_task_status(
    state: &AppState,
    task_id: u32,
    task_param: &DownloadMediaFileParam,
    task_status: aria2::Status,
) -> Result<bool> {
    match task_status.status {
        aria2::TaskStatus::Active | aria2::TaskStatus::Waiting => Ok(true),
        aria2::TaskStatus::Paused => {
            info!(task_id, "download task paused, unpause it");
            aria2::Client::try_from(state)?.unpause(&task_status.gid).await?;

            Ok(true)
        }
        aria2::TaskStatus::Error => {
            let error_msg = format!(
                "error {}, {}",
                task_status.error_code.unwrap(),
                task_status.error_message.unwrap()
            );
            on_task_error(state, task_id, task_param, &error_msg).await?;

            Ok(false)
        }
        aria2::TaskStatus::Complete => {
            on_task_complete(state, task_id, task_param).await?;
            Ok(false)
        }
        aria2::TaskStatus::Removed => Ok(false),
    }
}

async fn on_task_error(
    state: &AppState,
    task_id: u32,
    task_param: &DownloadMediaFileParam,
    error_msg: &str,
) -> Result<()> {
    info!(task_id, "download task error: {}", error_msg);

    let txn = state.db.begin().await?;
    status::update_status_to_failed(&txn, task_id, error_msg).await?;
    media::update_status_to_waiting(&txn, task_param).await?;
    txn.commit().await?;

    Ok(())
}

async fn on_task_complete(state: &AppState, task_id: u32, task_param: &DownloadMediaFileParam) -> Result<()> {
    info!(task_id, "download task complete");

    let p = dir::link_downloaded_files(state.config.get_library_root(), task_param)?;

    let txn = state.db.begin().await?;
    status::update_status_to_done(&txn, task_id).await?;
    media::update_status_to_downloaded(&txn, &p).await?;
    subscription::mark_subscription_done_if_complete(&txn, &p).await?;
    send_task_complete_message(&txn, &p).await?;
    txn.commit().await?;
    Ok(())
}

async fn send_task_complete_message(db: &DatabaseTransaction, p: &DownloadMediaFileParam) -> Result<()> {
    match p.media_type {
        enums::MediaType::Tv => {
            create::create_send_message_task(
                db,
                &PushMessageParam::EpisodeDownloaded {
                    tv_id: p.media_id,
                    season_number: p.season_number.unwrap(),
                    episode_number: p.episode_number.unwrap(),
                },
            )
            .await
        }
        enums::MediaType::Movie => {
            create::create_send_message_task(db, &PushMessageParam::MovieDownloaded { movie_id: p.media_id }).await
        }
    }
}

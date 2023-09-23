use tracing::error;

use crate::{common::state::AppState, entity::task};

mod download;
mod message;

pub async fn run_download_tasks(state: &AppState) {
    match task::query::find_all_tasks_need_run(&state.db, task::Action::DownloadMediaFile).await {
        Ok(tasks) => download::run_tasks(state, tasks).await,
        Err(e) => error!("find download media file tasks need run error: {}", e),
    }
}

pub async fn run_message_tasks(state: &AppState) {
    match task::query::find_all_tasks_need_run(&state.db, task::Action::PushMessage).await {
        Ok(tasks) => message::run_tasks(state, tasks).await,
        Err(e) => error!("find push message tasks need run error: {}", e),
    }
}

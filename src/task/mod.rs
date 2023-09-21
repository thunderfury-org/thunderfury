use tracing::error;

use crate::common::state::AppState;

pub mod create;
pub mod param;

mod delete;
mod download;
mod message;
mod status;

pub async fn run_download_tasks(state: &AppState) {
    match download::find_download_tasks_not_done(&state.db).await {
        Ok(tasks) => download::run_tasks(state, tasks).await,
        Err(e) => error!("find tasks not done error: {}", e),
    }
}

pub async fn run_message_tasks(state: &AppState) {
    match message::find_message_tasks_not_done(&state.db).await {
        Ok(tasks) => message::run_tasks(state, tasks).await,
        Err(e) => error!("find tasks not done error: {}", e),
    }
}

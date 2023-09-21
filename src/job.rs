use tokio::time::Duration;

use crate::{common::state::AppState, subscription, task};

pub fn start_job(state: &AppState) {
    if let Some(disable) = state.config.get_server_config().disable_background_task {
        if disable {
            // background task disabled
            return;
        }
    }

    start_subscription_job(state);
    start_download_job(state);
    start_message_job(state);
}

fn start_subscription_job(state: &AppState) {
    let state = state.clone();
    tokio::spawn(async move {
        loop {
            subscription::job::run_subscriptions(&state).await;
            tokio::time::sleep(Duration::from_secs(5)).await;
        }
    });
}

fn start_download_job(state: &AppState) {
    let state = state.clone();
    tokio::spawn(async move {
        loop {
            task::run_download_tasks(&state).await;
            tokio::time::sleep(Duration::from_secs(3)).await;
        }
    });
}

fn start_message_job(state: &AppState) {
    let state = state.clone();
    tokio::spawn(async move {
        loop {
            task::run_message_tasks(&state).await;
            tokio::time::sleep(Duration::from_secs(5)).await;
        }
    });
}

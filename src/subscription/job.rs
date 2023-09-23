use chrono::Duration;
use tracing::{error, info};

use super::query;
use crate::{
    common::{enums::MediaType, state::AppState},
    entity::subscription,
    subscription::{tv, update},
};

pub async fn run_subscriptions(state: &AppState) {
    match query::find_subscriptions_need_run(&state.db).await {
        Ok(subs) => {
            for sub in &subs {
                run_one_subscription(state, sub).await;
            }
        }
        Err(e) => error!("find subscriptions need run error: {}", e),
    };
}

async fn run_one_subscription(state: &AppState, sub: &subscription::Model) {
    info!(sub_id = sub.id, "start to run subscription");

    if sub.media_type == MediaType::Movie {
        return;
    }

    match tv::run_tv_subscription(state, sub).await {
        Ok(_) => {
            info!(sub_id = sub.id, "run subscription success");
            update::update_next_run_time(&state.db, sub.id, Duration::minutes(30)).await;
        }
        Err(e) => {
            error!(sub_id = sub.id, "run subscription error: {}", e);
            update::update_next_run_time(&state.db, sub.id, Duration::minutes(10)).await;
        }
    };
}

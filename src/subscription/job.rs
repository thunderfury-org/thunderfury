use chrono::Duration;
use tracing::{error, info};

use crate::{
    common::{enums::MediaType, state::AppState},
    entity::subscription::{query, update, Model},
    subscription::tv,
};

pub async fn run_subscriptions(state: &AppState) {
    match query::find_all_subscriptions_need_to_run(&state.db).await {
        Ok(subs) => {
            for sub in &subs {
                run_one_subscription(state, sub).await;
            }
        }
        Err(e) => error!("find subscriptions need run error: {}", e),
    };
}

async fn run_one_subscription(state: &AppState, sub: &Model) {
    info!(sub_id = sub.id, "start to run subscription");

    if sub.media_type == MediaType::Movie {
        return;
    }

    let duration = match tv::run_tv_subscription(state, sub).await {
        Ok(_) => {
            info!(sub_id = sub.id, "run subscription success");
            Duration::minutes(30)
        }
        Err(e) => {
            error!(sub_id = sub.id, "run subscription error: {}", e);
            Duration::minutes(10)
        }
    };

    if let Err(e) = update::update_next_run_time(&state.db, sub.id, Duration::minutes(30)).await {
        error!(sub_id = sub.id, "update next run time error: {}", e);
    }
}

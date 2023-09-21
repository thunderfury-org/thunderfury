use chrono::{Duration, Utc};
use sea_orm::{DatabaseConnection, EntityTrait, Set};
use tracing::error;

use crate::entity::subscription;

pub async fn update_next_run_time(db: &DatabaseConnection, sub_id: u32, period: Duration) {
    match subscription::Entity::update(subscription::ActiveModel {
        id: Set(sub_id),
        last_run_time: Set(Some(Utc::now())),
        next_run_time: Set(Utc::now() + period),
        ..Default::default()
    })
    .exec(db)
    .await
    {
        Ok(_) => {}
        Err(e) => {
            error!(sub_id, "update subscription next run time error: {}", e);
        }
    }
}

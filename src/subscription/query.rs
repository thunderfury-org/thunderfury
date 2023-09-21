use chrono::Utc;
use sea_orm::{ColumnTrait, DatabaseConnection, EntityTrait, QueryFilter, QueryOrder};

use crate::{common::error::Result, entity::subscription};

pub async fn get_subscription_by_unique_id(
    db: &DatabaseConnection,
    unique_id: String,
) -> Result<Option<subscription::Model>> {
    Ok(subscription::Entity::find()
        .filter(subscription::Column::UniqueId.eq(unique_id))
        .one(db)
        .await?)
}

pub async fn find_subscriptions(db: &DatabaseConnection) -> Result<Vec<subscription::Model>> {
    Ok(subscription::Entity::find()
        .order_by_desc(subscription::Column::Id)
        .all(db)
        .await?)
}

pub async fn find_subscriptions_need_run(db: &DatabaseConnection) -> Result<Vec<subscription::Model>> {
    Ok(subscription::Entity::find()
        .filter(subscription::Column::Status.eq(subscription::Status::Running))
        .filter(subscription::Column::NextRunTime.lte(Utc::now()))
        .all(db)
        .await?)
}

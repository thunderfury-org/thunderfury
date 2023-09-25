use sea_orm::{ConnectionTrait, EntityTrait};

use crate::common::error::Result;

pub async fn delete_task<C>(db: &C, task_id: i32) -> Result<()>
where
    C: ConnectionTrait,
{
    super::Entity::delete_by_id(task_id).exec(db).await?;
    Ok(())
}

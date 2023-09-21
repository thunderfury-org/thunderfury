use sea_orm::{ConnectionTrait, EntityTrait};

use crate::{common::error::Result, entity::task};

pub async fn delete_task<C>(db: &C, task_id: u32) -> Result<()>
where
    C: ConnectionTrait,
{
    task::Entity::delete_by_id(task_id).exec(db).await?;
    Ok(())
}

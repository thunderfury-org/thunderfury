use sea_orm::{ConnectionTrait, EntityTrait};

use super::{Entity, Model};
use crate::common::error::{Error, NotFoundCode, Result};

pub async fn get_or_fail<C>(db: &C, tv_id: i32) -> Result<Model>
where
    C: ConnectionTrait,
{
    Entity::find_by_id(tv_id)
        .one(db)
        .await?
        .ok_or(Error::NotFound(NotFoundCode::Tv, format!("can not find tv {}", tv_id)))
}

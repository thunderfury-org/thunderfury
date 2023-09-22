use sea_orm::{ConnectionTrait, EntityTrait};

use crate::{
    common::error::{Error, NotFoundCode, Result},
    entity::tv,
};

pub async fn get_or_fail<C>(db: &C, tv_id: i32) -> Result<tv::Model>
where
    C: ConnectionTrait,
{
    tv::Entity::find_by_id(tv_id)
        .one(db)
        .await?
        .ok_or(Error::NotFound(NotFoundCode::Tv, format!("can not find tv {}", tv_id)))
}

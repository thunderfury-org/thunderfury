use sea_orm::{ConnectionTrait, EntityTrait};

use crate::{
    common::error::{Error, NotFoundCode, Result},
    entity::movie,
};

pub async fn get_or_fail<C>(db: &C, movie_id: u32) -> Result<movie::Model>
where
    C: ConnectionTrait,
{
    movie::Entity::find_by_id(movie_id)
        .one(db)
        .await?
        .ok_or(Error::NotFound(
            NotFoundCode::Movie,
            format!("can not find movie {}", movie_id),
        ))
}

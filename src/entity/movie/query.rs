use sea_orm::{ColumnTrait, ConnectionTrait, EntityTrait, QueryFilter};

use super::{Column, Entity, Model};
use crate::common::error::{Error, NotFoundCode, Result};

pub async fn get_or_fail<C>(db: &C, movie_id: i32) -> Result<Model>
where
    C: ConnectionTrait,
{
    Entity::find_by_id(movie_id).one(db).await?.ok_or(Error::NotFound(
        NotFoundCode::Movie,
        format!("can not find movie {}", movie_id),
    ))
}

pub async fn find_all_movies<C>(db: &C) -> Result<Vec<Model>>
where
    C: ConnectionTrait,
{
    Ok(Entity::find().all(db).await?)
}

pub async fn find_movies_by_ids<C>(db: &C, ids: Vec<i32>) -> Result<Vec<Model>>
where
    C: ConnectionTrait,
{
    if ids.is_empty() {
        return Ok(Vec::new());
    }
    Ok(Entity::find().filter(Column::Id.is_in(ids)).all(db).await?)
}

use chrono::Utc;
use sea_orm::{ConnectionTrait, Set};

use super::{ActiveModel, Model};
use crate::common::error::Result;

pub struct NewTv {
    pub name: String,
    pub year: i32,
    pub status: String,
    pub first_air_date: String,
    pub number_of_seasons: i32,
    pub tmdb_id: i32,
    pub original_language: String,
    pub original_name: String,
    pub overview: String,
    pub poster_path: String,
    pub backdrop_path: String,
}

pub async fn create_tv<C>(db: &C, new_tv: NewTv) -> Result<Model>
where
    C: ConnectionTrait,
{
    Ok(ActiveModel {
        name: Set(new_tv.name),
        year: Set(new_tv.year),
        status: Set(new_tv.status),
        first_air_date: Set(new_tv.first_air_date),
        number_of_seasons: Set(new_tv.number_of_seasons),
        tmdb_id: Set(new_tv.tmdb_id),
        original_language: Set(new_tv.original_language),
        original_name: Set(new_tv.original_name),
        overview: Set(new_tv.overview),
        poster_path: Set(new_tv.poster_path),
        backdrop_path: Set(new_tv.backdrop_path),
        create_time: Set(Utc::now()),
        ..Default::default()
    }
    .insert(db)
    .await?)
}

use serde::Serialize;
use utoipa::ToSchema;

use crate::entity::tv::Model;

pub mod query;

#[derive(Debug, Serialize, ToSchema)]
pub struct TvDetail {
    pub id: i32,
    pub name: String,
    pub year: i32,
    pub status: String,
    pub first_air_date: String,
    pub number_of_seasons: i32,
    pub tmdb_id: i32,
    pub overview: String,
    pub poster_path: String,
    pub backdrop_path: String,
}

impl From<Model> for TvDetail {
    fn from(val: Model) -> Self {
        Self {
            id: val.id,
            name: val.name,
            year: val.year,
            status: val.status,
            first_air_date: val.first_air_date,
            number_of_seasons: val.number_of_seasons,
            tmdb_id: val.tmdb_id,
            overview: val.overview,
            poster_path: val.poster_path,
            backdrop_path: val.backdrop_path,
        }
    }
}

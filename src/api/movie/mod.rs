use serde::Serialize;
use utoipa::ToSchema;

use crate::entity::movie::Model;

pub mod query;

#[derive(Debug, Serialize, ToSchema)]
pub struct MovieDetail {
    pub id: i32,
    pub name: String,
    pub year: i32,
    pub status: String,
    pub release_date: String,
    pub tmdb_id: i32,
    pub overview: String,
}

impl From<Model> for MovieDetail {
    fn from(val: Model) -> Self {
        Self {
            id: val.id,
            name: val.name,
            year: val.year,
            status: val.status,
            release_date: val.release_data,
            tmdb_id: val.tmdb_id,
            overview: val.overview,
        }
    }
}

use serde::Serialize;
use utoipa::ToSchema;

use super::genre::Genre;

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
    pub genres: Vec<Genre>,
}

#[derive(Debug, Serialize, ToSchema)]
pub struct MovieDetail {
    pub id: i32,
    pub name: String,
    pub year: i32,
    pub status: String,
    pub release_date: String,
    pub tmdb_id: i32,
    pub overview: String,
    pub genres: Vec<Genre>,
}

use actix_web::{get, web};
use sea_orm::EntityTrait;

use crate::{
    api::{
        error::{ok, ApiResult},
        model::library::{MovieDetail, TvDetail},
    },
    common::state::AppState,
    entity::{movie, tv},
};

#[utoipa::path(
    get,
    context_path = "/api",
    responses(
        (status = 200, body = Vec<TvDetail>),
    )
)]
#[get("/library/tvs")]
pub async fn list_tvs(state: web::Data<AppState>) -> ApiResult<Vec<TvDetail>> {
    ok(tv::Entity::find()
        .all(&state.db)
        .await?
        .into_iter()
        .map(Into::into)
        .collect())
}

#[utoipa::path(
    get,
    context_path = "/api",
    responses(
        (status = 200, body = Vec<MovieDetail>),
    )
)]
#[get("/library/movies")]
pub async fn list_movies(state: web::Data<AppState>) -> ApiResult<Vec<MovieDetail>> {
    ok(movie::Entity::find()
        .all(&state.db)
        .await?
        .into_iter()
        .map(Into::into)
        .collect())
}

impl From<movie::Model> for MovieDetail {
    fn from(val: movie::Model) -> Self {
        Self {
            id: val.id,
            name: val.name,
            year: val.year,
            status: val.status,
            release_date: val.release_data,
            tmdb_id: val.tmdb_id,
            overview: val.overview,
            genres: vec![],
        }
    }
}

impl From<tv::Model> for TvDetail {
    fn from(val: tv::Model) -> Self {
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
            genres: vec![],
        }
    }
}

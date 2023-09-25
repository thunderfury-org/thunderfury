use sea_orm::entity::prelude::*;

pub mod create;
pub mod episode;
pub mod query;
pub mod season;

#[derive(Clone, Debug, PartialEq, DeriveEntityModel, Eq)]
#[sea_orm(table_name = "tv")]
pub struct Model {
    #[sea_orm(primary_key)]
    pub id: i32,
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
    pub create_time: DateTimeUtc,
}

#[derive(Copy, Clone, Debug, EnumIter, DeriveRelation)]
pub enum Relation {}

impl ActiveModelBehavior for ActiveModel {}

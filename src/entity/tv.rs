//! `SeaORM` Entity. Generated by sea-orm-codegen 0.11.2

use sea_orm::entity::prelude::*;

#[derive(Clone, Debug, PartialEq, DeriveEntityModel, Eq)]
#[sea_orm(table_name = "tv")]
pub struct Model {
    #[sea_orm(primary_key)]
    pub id: u32,
    pub name: String,
    pub year: u32,
    pub status: String,
    pub first_air_date: String,
    pub number_of_seasons: u32,
    pub tmdb_id: u32,
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

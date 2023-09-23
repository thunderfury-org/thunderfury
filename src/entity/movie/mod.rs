use sea_orm::entity::prelude::*;

pub mod query;

#[derive(Clone, Debug, PartialEq, DeriveEntityModel, Eq)]
#[sea_orm(table_name = "movie")]
pub struct Model {
    #[sea_orm(primary_key)]
    pub id: i32,
    pub name: String,
    pub year: i32,
    pub status: String,
    pub release_data: String,
    pub tmdb_id: i32,
    pub overview: String,
    pub create_time: DateTimeUtc,
}

#[derive(Copy, Clone, Debug, EnumIter, DeriveRelation)]
pub enum Relation {}

impl ActiveModelBehavior for ActiveModel {}

use sea_orm::entity::prelude::*;

#[derive(Clone, Debug, PartialEq, DeriveEntityModel, Eq)]
#[sea_orm(table_name = "season")]
pub struct Model {
    #[sea_orm(primary_key)]
    pub id: i32,
    pub tv_id: i32,
    pub season_number: i32,
    pub air_date: String,
    pub number_of_episodes: i32,
    pub overview: String,
    pub poster_path: String,
    pub create_time: DateTimeUtc,
}

#[derive(Copy, Clone, Debug, EnumIter, DeriveRelation)]
pub enum Relation {}

impl ActiveModelBehavior for ActiveModel {}

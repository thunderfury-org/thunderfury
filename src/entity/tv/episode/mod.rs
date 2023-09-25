use sea_orm::entity::prelude::*;

pub mod create;
pub mod query;
pub mod update;

#[derive(Clone, Debug, PartialEq, DeriveEntityModel, Eq)]
#[sea_orm(table_name = "episode")]
pub struct Model {
    #[sea_orm(primary_key)]
    pub id: i32,
    pub tv_id: i32,
    pub season_number: i32,
    pub episode_number: i32,
    pub name: String,
    pub air_date: String,
    pub status: Status,
    pub overview: String,
    pub still_path: String,
    pub create_time: DateTimeUtc,
    pub external_task_id: Option<String>,
    pub download_time: Option<DateTimeUtc>,
    pub file_path: Option<String>,
}

#[derive(Copy, Clone, Debug, EnumIter, DeriveRelation)]
pub enum Relation {}

impl ActiveModelBehavior for ActiveModel {}

#[derive(Debug, Clone, Default, PartialEq, Eq, EnumIter, DeriveActiveEnum, DeriveDisplay)]
#[sea_orm(rs_type = "String", db_type = "String(Some(1))")]
pub enum Status {
    #[default]
    #[sea_orm(string_value = "waiting", display_value = "waiting")]
    Waiting,

    #[sea_orm(string_value = "queued", display_value = "queued")]
    Queued,

    #[sea_orm(string_value = "downloading", display_value = "downloading")]
    Downloading,

    #[sea_orm(string_value = "downloaded", display_value = "downloaded")]
    Downloaded,
}

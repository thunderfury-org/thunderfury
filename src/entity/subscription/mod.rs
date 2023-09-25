use sea_orm::entity::prelude::*;

use crate::common::{
    enums::{MediaType, Provider},
    types::StringVec,
};

pub mod create;
pub mod query;
pub mod update;

#[derive(Clone, Debug, PartialEq, DeriveEntityModel, Eq)]
#[sea_orm(table_name = "subscription")]
pub struct Model {
    #[sea_orm(primary_key)]
    pub id: i32,
    pub unique_id: String,
    pub media_type: MediaType,
    pub media_id: i32,
    pub tmdb_id: i32,
    pub resource_provider: Provider,
    pub resource_url: Option<String>,
    pub season_number: Option<i32>,
    pub resolutions: Option<StringVec>,
    pub subtitles: Option<StringVec>,
    pub status: Status,
    pub last_run_time: Option<DateTimeUtc>,
    pub next_run_time: DateTimeUtc,
    pub create_time: DateTimeUtc,
    pub finish_time: Option<DateTimeUtc>,
}

#[derive(Copy, Clone, Debug, EnumIter, DeriveRelation)]
pub enum Relation {}

impl ActiveModelBehavior for ActiveModel {}

#[derive(Debug, Clone, Default, PartialEq, Eq, EnumIter, DeriveActiveEnum, strum::Display)]
#[sea_orm(rs_type = "String", db_type = "String(Some(1))")]
#[strum(serialize_all = "snake_case")]
pub enum Status {
    #[default]
    #[sea_orm(string_value = "running")]
    Running,

    #[sea_orm(string_value = "done")]
    Done,
}

use sea_orm::entity::prelude::*;
use serde::{Deserialize, Serialize};

#[derive(
    Debug, Default, Clone, Copy, PartialEq, Eq, EnumIter, DeriveActiveEnum, strum::Display, Serialize, Deserialize,
)]
#[sea_orm(rs_type = "String", db_type = "String(Some(1))")]
#[serde(rename_all = "snake_case")]
#[strum(serialize_all = "snake_case")]
pub enum MediaType {
    #[default]
    #[sea_orm(string_value = "tv")]
    Tv,

    #[sea_orm(string_value = "movie")]
    Movie,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, EnumIter, DeriveActiveEnum, strum::Display, Serialize, Deserialize)]
#[sea_orm(rs_type = "String", db_type = "String(Some(1))")]
#[serde(rename_all = "snake_case")]
#[strum(serialize_all = "snake_case")]
pub enum Provider {
    #[sea_orm(string_value = "rss")]
    Rss,

    #[sea_orm(string_value = "alist")]
    Alist,
}

#[derive(Debug, Default, Clone, Copy, PartialEq, Eq, strum::Display, Serialize, Deserialize, Hash)]
#[serde(rename_all = "snake_case")]
#[strum(serialize_all = "snake_case")]
pub enum Downloader {
    #[default]
    Bt,
    Alist,
}

#[derive(Debug, Clone, Copy, Default, PartialEq, Eq, strum::Display, Serialize, Deserialize, Hash)]
#[serde(rename_all = "snake_case")]
#[strum(serialize_all = "snake_case")]
pub enum FileType {
    #[default]
    Unknown,
    Video,
    Subtitle,
}

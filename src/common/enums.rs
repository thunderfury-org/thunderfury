use sea_orm::entity::prelude::*;
use serde::{Deserialize, Serialize};

use crate::common::error;

#[derive(
    Debug, Clone, Copy, Default, PartialEq, Eq, EnumIter, DeriveActiveEnum, strum::Display, Serialize, Deserialize,
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

impl TryInto<MediaType> for &str {
    type Error = error::Error;

    fn try_into(self) -> Result<MediaType, Self::Error> {
        match self {
            "tv" => Ok(MediaType::Tv),
            "movie" => Ok(MediaType::Movie),
            _ => Err(error::Error::InvalidArgument(
                "invalid media type, must be tv or movie".to_string(),
            )),
        }
    }
}

#[derive(Debug, Clone, Copy, Default, PartialEq, Eq, EnumIter, DeriveActiveEnum, strum::Display)]
#[sea_orm(rs_type = "String", db_type = "String(Some(1))")]
#[strum(serialize_all = "snake_case")]
pub enum Provider {
    #[default]
    #[sea_orm(string_value = "rss")]
    Rss,

    #[sea_orm(string_value = "alist")]
    Alist,
}

impl TryInto<Provider> for &str {
    type Error = error::Error;

    fn try_into(self) -> Result<Provider, Self::Error> {
        match self {
            "rss" => Ok(Provider::Rss),
            "alist" => Ok(Provider::Alist),
            _ => Err(error::Error::InvalidArgument(format!(
                "invalid provider value {}",
                self
            ))),
        }
    }
}

#[derive(Debug, Clone, Copy, Default, PartialEq, Eq, strum::Display, Serialize, Deserialize, Hash)]
#[serde(rename_all = "snake_case")]
#[strum(serialize_all = "snake_case")]
pub enum Downloader {
    #[default]
    Bt,

    Alist,
}

#[derive(Debug, Clone, Copy, Default, PartialEq, Eq, Hash, Serialize, Deserialize, strum::Display)]
#[serde(rename_all = "snake_case")]
#[strum(serialize_all = "snake_case")]
pub enum FileType {
    #[default]
    Unknown,

    Video,

    Subtitle,
}

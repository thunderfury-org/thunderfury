use sea_orm::entity::prelude::*;

use super::{
    column_type::StringVec,
    enums::{MediaType, Provider},
};

#[derive(Clone, Debug, Default, PartialEq, DeriveEntityModel, Eq)]
#[sea_orm(table_name = "subscription")]
pub struct Model {
    #[sea_orm(primary_key)]
    pub id: u32,
    pub unique_id: String,
    pub media_type: MediaType,
    pub media_id: u32,
    pub tmdb_id: u32,
    pub resource_provider: Provider,
    pub resource_url: Option<String>,
    pub season_number: Option<u32>,
    pub resolutions: Option<StringVec>,
    pub subtitles: Option<StringVec>,
    pub status: Status,
    pub last_run_time: Option<DateTimeUtc>,
    pub next_run_time: DateTimeUtc,
    pub create_time: DateTimeUtc,
    pub finish_time: Option<DateTimeUtc>,
}

impl Model {
    pub fn unique_id(&self) -> String {
        let parts = vec![
            self.media_type.to_string(),
            self.tmdb_id.to_string(),
            self.resource_provider.to_string(),
            self.resource_url.as_ref().map_or("".to_owned(), |s| s.to_string()),
            self.season_number.map_or("".to_owned(), |n| n.to_string()),
        ];
        sha256::digest(parts.join(":"))
    }
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

#[cfg(test)]
mod tests {
    use super::{MediaType, Model, Provider};

    #[test]
    fn unique_id() {
        let request = Model {
            media_type: MediaType::Movie,
            tmdb_id: 1,
            resource_provider: Provider::Rss,
            resource_url: None,
            season_number: None,
            resolutions: None,
            subtitles: None,
            ..Default::default()
        };

        assert_eq!(
            request.unique_id(),
            "847260174378ac6ca75ba961da71350ddca05beaada842704dc45025cf0ac3fe"
        );
    }
}

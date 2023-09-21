use sea_orm::entity::prelude::*;

#[derive(Clone, Debug, Default, PartialEq, DeriveEntityModel, Eq)]
#[sea_orm(table_name = "task")]
pub struct Model {
    #[sea_orm(primary_key)]
    pub id: u32,
    pub action: Action,
    pub status: Status,
    pub param: String,
    pub create_time: DateTimeUtc,
    pub external_task_id: Option<String>,
    pub begin_time: Option<DateTimeUtc>,
    pub end_time: Option<DateTimeUtc>,
    pub error_msg: Option<String>,
    pub retry_count: Option<u32>,
    pub next_retry_time: Option<DateTimeUtc>,
}

#[derive(Copy, Clone, Debug, EnumIter, DeriveRelation)]
pub enum Relation {}

impl ActiveModelBehavior for ActiveModel {}

#[derive(Debug, Clone, Default, PartialEq, Eq, EnumIter, DeriveActiveEnum, strum::Display)]
#[sea_orm(rs_type = "String", db_type = "String(Some(1))")]
#[strum(serialize_all = "snake_case")]
pub enum Action {
    #[default]
    #[sea_orm(string_value = "download_media_file")]
    DownloadMediaFile,

    #[sea_orm(string_value = "push_message")]
    PushMessage,
}

#[derive(Debug, Clone, Default, PartialEq, Eq, EnumIter, DeriveActiveEnum, strum::Display)]
#[sea_orm(rs_type = "String", db_type = "String(Some(1))")]
#[strum(serialize_all = "snake_case")]
pub enum Status {
    #[default]
    #[sea_orm(string_value = "queued")]
    Queued,

    #[sea_orm(string_value = "running")]
    Running,

    #[sea_orm(string_value = "done")]
    Done,

    #[sea_orm(string_value = "failed")]
    Failed,
}

use sea_orm_migration::prelude::*;

#[derive(DeriveMigrationName)]
pub struct Migration;

#[async_trait::async_trait]
impl MigrationTrait for Migration {
    async fn up(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        manager
            .create_table(
                Table::create()
                    .table(Subscription::Table)
                    .if_not_exists()
                    .col(
                        ColumnDef::new(Subscription::Id)
                            .integer()
                            .not_null()
                            .auto_increment()
                            .primary_key(),
                    )
                    .col(ColumnDef::new(Subscription::MediaType).string().not_null())
                    .col(ColumnDef::new(Subscription::MediaId).integer().not_null())
                    .col(ColumnDef::new(Subscription::TmdbId).integer().not_null())
                    .col(ColumnDef::new(Subscription::ResourceProvider).string().not_null())
                    .col(ColumnDef::new(Subscription::ResourceUrl).string().not_null())
                    .col(ColumnDef::new(Subscription::SeasonNumber).integer())
                    .col(ColumnDef::new(Subscription::Resolutions).string())
                    .col(ColumnDef::new(Subscription::Subtitles).string())
                    .col(ColumnDef::new(Subscription::Status).string().not_null())
                    .col(ColumnDef::new(Subscription::LastRunTime).timestamp())
                    .col(ColumnDef::new(Subscription::NextRunTime).timestamp().not_null())
                    .col(ColumnDef::new(Subscription::CreateTime).timestamp().not_null())
                    .col(ColumnDef::new(Subscription::FinishTime).timestamp())
                    .col(ColumnDef::new(Subscription::UniqueId).string().not_null())
                    .index(
                        Index::create()
                            .unique()
                            .name("uk_unique_id")
                            .col(Subscription::UniqueId),
                    )
                    .to_owned(),
            )
            .await
    }

    async fn down(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        manager
            .drop_table(Table::drop().table(Subscription::Table).to_owned())
            .await
    }
}

/// Learn more at https://docs.rs/sea-query#iden
#[derive(Iden)]
enum Subscription {
    Table,
    Id,
    MediaType,
    MediaId,
    TmdbId,
    ResourceProvider,
    ResourceUrl,
    SeasonNumber,
    Resolutions,
    Subtitles,
    Status,
    LastRunTime,
    NextRunTime,
    CreateTime,
    FinishTime,
    UniqueId,
}

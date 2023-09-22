use sea_orm_migration::prelude::*;

#[derive(DeriveMigrationName)]
pub struct Migration;

#[async_trait::async_trait]
impl MigrationTrait for Migration {
    async fn up(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        manager
            .create_table(
                Table::create()
                    .table(Task::Table)
                    .if_not_exists()
                    .col(
                        ColumnDef::new(Task::Id)
                            .integer()
                            .not_null()
                            .auto_increment()
                            .primary_key(),
                    )
                    .col(ColumnDef::new(Task::Action).string().not_null())
                    .col(ColumnDef::new(Task::Status).string().not_null())
                    .col(ColumnDef::new(Task::Param).text().not_null())
                    .col(ColumnDef::new(Task::CreateTime).timestamp().not_null())
                    .col(ColumnDef::new(Task::ExternalTaskId).string())
                    .col(ColumnDef::new(Task::BeginTime).timestamp())
                    .col(ColumnDef::new(Task::EndTime).timestamp())
                    .col(ColumnDef::new(Task::ErrorMsg).text())
                    .col(ColumnDef::new(Task::RetryCount).integer())
                    .col(ColumnDef::new(Task::NextRetryTime).timestamp())
                    .to_owned(),
            )
            .await
    }

    async fn down(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        manager.drop_table(Table::drop().table(Task::Table).to_owned()).await
    }
}

/// Learn more at https://docs.rs/sea-query#iden
#[derive(Iden)]
enum Task {
    Table,
    Id,
    Action,
    Status,
    Param,
    CreateTime,
    ExternalTaskId,
    BeginTime,
    EndTime,
    ErrorMsg,
    RetryCount,
    NextRetryTime,
}

use sea_orm::{EnumIter, Iterable};
#[allow(unused)]
use sea_orm_migration::prelude::*;

#[derive(DeriveMigrationName)]
pub struct Migration;

#[derive(DeriveIden)]
enum Requests {
    Table,
    // Auto Inc ID
    Id,
    // Request ID
    RequestId,
    // Batch ID
    BatchId,
    // Value received
    Value,
    // Informs whether request was successfull
    Status,
}

#[derive(Iden, EnumIter)]
pub enum Status {
    Table,
    #[iden = "Success"]
    Success,
    #[iden = "Failure"]
    Failure,
}

#[async_trait::async_trait]
impl MigrationTrait for Migration {
    async fn up(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        manager
            .create_table(
                Table::create()
                    .table(Requests::Table)
                    .if_not_exists()
                    .col(
                        ColumnDef::new(Requests::Id)
                            .integer()
                            .not_null()
                            .auto_increment()
                            .primary_key(),
                    )
                    .col(ColumnDef::new(Requests::RequestId).string().not_null())
                    .col(ColumnDef::new(Requests::BatchId).string().not_null())
                    .col(ColumnDef::new(Requests::Value).string().not_null())
                    .col(
                        ColumnDef::new(Requests::Status)
                            .enumeration(Status::Table, Status::iter().skip(1))
                            .not_null(),
                    )
                    .to_owned(),
            )
            .await
    }

    async fn down(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        manager
            .drop_table(Table::drop().table(Requests::Table).if_exists().to_owned())
            .await
    }
}

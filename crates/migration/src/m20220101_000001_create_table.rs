use sea_orm::{EnumIter, Iterable};
#[allow(unused)]
use sea_orm_migration::prelude::*;

#[derive(DeriveMigrationName)]
pub struct Migration;

#[derive(DeriveIden)]
enum Request {
    Table,
    Id,
    Value,
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
                    .table(Request::Table)
                    .if_not_exists()
                    .col(
                        ColumnDef::new(Request::Id)
                            .integer()
                            .not_null()
                            .auto_increment()
                            .primary_key(),
                    )
                    .col(ColumnDef::new(Request::Value).string().not_null())
                    .col(
                        ColumnDef::new(Request::Status)
                            .enumeration(Status::Table, Status::iter().skip(1)),
                    )
                    .to_owned(),
            )
            .await
    }

    async fn down(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        manager
            .drop_table(Table::drop().table(Request::Table).if_exists().to_owned())
            .await
    }
}

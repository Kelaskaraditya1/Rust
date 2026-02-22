use sea_orm_migration::{prelude::*, schema::*};

#[derive(DeriveMigrationName)]
pub struct Migration;

#[async_trait::async_trait]
impl MigrationTrait for Migration {
    async fn up(&self, manager: &SchemaManager) -> Result<(), DbErr> {


        manager
            .create_table(
                Table::create()
                    .table(Users::Table)
                    .if_not_exists()
                    .col(uuid(Users::UserId).not_null().primary_key())
                    .col(string(Users::Name).not_null())
                    .col(string_uniq(Users::Contact).not_null())
                    .col(string_uniq(Users::Email).not_null())
                    .col(string_uniq(Users::Username).not_null())
                    .col(string(Users::Password).not_null())
                    .col(timestamp(Users::CreatedAt).not_null().default(Expr::current_timestamp()))
                    .to_owned(),
            )
            .await
    }

    async fn down(&self, manager: &SchemaManager) -> Result<(), DbErr> {


        manager
            .drop_table(Table::drop().table(Users::Table).to_owned())
            .await
    }
}

#[derive(DeriveIden)]
pub enum Users {
    Table,
    UserId,
    Name,
    Contact,
    Email,
    Username,
    Password,
    CreatedAt
}

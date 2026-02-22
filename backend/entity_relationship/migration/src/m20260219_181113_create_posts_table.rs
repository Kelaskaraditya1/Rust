use sea_orm_migration::{prelude::*, schema::*};

use crate::m20220101_000001_create_table::Users;

#[derive(DeriveMigrationName)]
pub struct Migration;

#[async_trait::async_trait]
impl MigrationTrait for Migration {
    async fn up(&self, manager: &SchemaManager) -> Result<(), DbErr> {


        manager
            .create_table(
                Table::create()
                    .table(Post::Table)
                    .if_not_exists()
                    .col(uuid(Post::PostId).not_null().primary_key())
                    .col(string(Post::Title).not_null())
                    .col(string(Post::Content).not_null())
                    .col(string(Post::MediaUrl))
                    .col(timestamp(Post::CreatedAt).not_null().default(Expr::current_timestamp()))
                    .col(uuid(Post::UserId).not_null())
                    .foreign_key(
                        ForeignKey::create()
                            .name("fk-post-user")
                            .from(Post::Table, Post::UserId)
                            .to(Users::Table,Users::UserId)       
                    )
                    .to_owned(),
            )
            .await
    }

    async fn down(&self, manager: &SchemaManager) -> Result<(), DbErr> {


        manager
            .drop_table(Table::drop().table(Post::Table).to_owned())
            .await
    }
}

#[derive(DeriveIden)]
enum Post {
    Table,
    PostId,
    Title,
    Content,
    MediaUrl,
    CreatedAt,
    UserId

}

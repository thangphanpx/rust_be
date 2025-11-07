use sea_orm_migration::prelude::*;

#[derive(DeriveMigrationName)]
pub struct Migration;

#[async_trait::async_trait]
impl MigrationTrait for Migration {
    async fn up(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        // Create the posts table
        manager
            .create_table(
                Table::create()
                    .table(Posts::Table)
                    .if_not_exists()
                    .col(
                        ColumnDef::new(Posts::Id)
                            .integer()
                            .not_null()
                            .auto_increment()
                            .primary_key(),
                    )
                    .col(ColumnDef::new(Posts::Title).string().not_null())
                    .col(ColumnDef::new(Posts::Content).text().not_null())
                    .col(
                        ColumnDef::new(Posts::UserId)
                            .integer()
                            .not_null()
                            .foreign_key(ForeignKey::create()
                                .name("fk_posts_user_id")
                                .from(Posts::Table, Posts::UserId)
                                .to(Users::Table, Users::Id)
                                .on_delete(ForeignKeyAction::Cascade)),
                    )
                    .col(ColumnDef::new(Posts::IsPublished).boolean().not_null().default(false))
                    .to_owned(),
            )
            .await?;

        // Create indexes for better performance
        manager
            .create_index(
                Index::create()
                    .table(Posts::Table)
                    .name("idx_posts_user_id")
                    .col(Posts::UserId)
                    .to_owned(),
            )
            .await?;

        manager
            .create_index(
                Index::create()
                    .table(Posts::Table)
                    .name("idx_posts_is_published")
                    .col(Posts::IsPublished)
                    .to_owned(),
            )
            .await?;

        Ok(())
    }

    async fn down(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        manager
            .drop_table(Table::drop().table(Posts::Table).to_owned())
            .await
    }
}

#[derive(DeriveIden)]
enum Posts {
    Table,
    Id,
    Title,
    Content,
    UserId,
    IsPublished,
}

#[derive(DeriveIden)]
enum Users {
    Table,
    Id,
}
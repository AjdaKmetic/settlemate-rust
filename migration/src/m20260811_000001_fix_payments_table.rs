use sea_orm_migration::{prelude::*, schema::*};

#[derive(DeriveMigrationName)]
pub struct Migration;

#[async_trait::async_trait]
impl MigrationTrait for Migration {
    async fn up(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        manager
            .drop_table(
                Table::drop()
                    .table(Payments::Table)
                    .to_owned(),
            )
            .await?;

        manager
            .create_table(
                Table::create()
                    .table(Payments::Table)
                    .col(pk_auto(Payments::Id))
                    .col(integer(Payments::FromId))
                    .col(integer(Payments::ToId))
                    .col(integer(Payments::AmountCents))
                    .col(integer_null(Payments::GroupId))
                    .col(
                        timestamp(Payments::CreatedAt)
                            .default(Expr::current_timestamp()),
                    )
                    .foreign_key(
                        ForeignKey::create()
                            .from(Payments::Table, Payments::FromId)
                            .to(Users::Table, Users::Id),
                    )
                    .foreign_key(
                        ForeignKey::create()
                            .from(Payments::Table, Payments::ToId)
                            .to(Users::Table, Users::Id),
                    )
                    .foreign_key(
                        ForeignKey::create()
                            .from(Payments::Table, Payments::GroupId)
                            .to(Groups::Table, Groups::Id)
                            .on_delete(ForeignKeyAction::SetNull),
                    )
                    .to_owned(),
            )
            .await?;

        Ok(())
    }

    async fn down(&self, _manager: &SchemaManager) -> Result<(), DbErr> {
        Err(DbErr::Migration(
            "Payments table cannot be safely reverted after direct settlements."
                .to_string(),
        ))
    }
}

#[derive(DeriveIden)]
enum Payments {
    Table,
    Id,
    FromId,
    ToId,
    AmountCents,
    GroupId,
    CreatedAt,
}

#[derive(DeriveIden)]
enum Users {
    Table,
    Id,
}

#[derive(DeriveIden)]
enum Groups {
    Table,
    Id,
}
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
                    .col(pk_auto(Users::Id))
                    .col(string(Users::Name).not_null())
                    .col(string(Users::Email).not_null().unique_key())
                    .col(string(Users::PasswordHash).not_null())
                    .col(
                        timestamp(Users::CreatedAt)
                            .not_null()
                            .default(Expr::current_timestamp()),
                    )
                    .to_owned(),
            )
            .await?;

        manager
            .create_table(
                Table::create()
                    .table(Groups::Table)
                    .if_not_exists()
                    .col(pk_auto(Groups::Id))
                    .col(string(Groups::Name).not_null())
                    .col(
                        timestamp(Groups::CreatedAt)
                            .not_null()
                            .default(Expr::current_timestamp()),
                    )
                    .to_owned(),
            )
            .await?;

        manager
            .create_table(
                Table::create()
                    .table(GroupMembers::Table)
                    .if_not_exists()
                    .col(pk_auto(GroupMembers::Id))
                    .col(integer(GroupMembers::GroupId).not_null())
                    .col(integer(GroupMembers::UserId).not_null())
                    .foreign_key(
                        ForeignKey::create()
                            .from(GroupMembers::Table, GroupMembers::GroupId)
                            .to(Groups::Table, Groups::Id)
                            .on_delete(ForeignKeyAction::Cascade),
                    )
                    .foreign_key(
                        ForeignKey::create()
                            .from(GroupMembers::Table, GroupMembers::UserId)
                            .to(Users::Table, Users::Id)
                            .on_delete(ForeignKeyAction::Cascade),
                    )
                    .to_owned(),
            )
            .await?;

        manager
            .create_table(
                Table::create()
                    .table(Expenses::Table)
                    .if_not_exists()
                    .col(pk_auto(Expenses::Id))
                    .col(string(Expenses::Description).not_null())
                    .col(double(Expenses::Amount).not_null())
                    .col(integer(Expenses::PaidBy).not_null())
                    .col(integer_null(Expenses::GroupId))
                    .col(string(Expenses::SplitType).not_null())
                    .col(
                        timestamp(Expenses::CreatedAt)
                            .not_null()
                            .default(Expr::current_timestamp()),
                    )
                    .foreign_key(
                        ForeignKey::create()
                            .from(Expenses::Table, Expenses::PaidBy)
                            .to(Users::Table, Users::Id),
                    )
                    .foreign_key(
                        ForeignKey::create()
                            .from(Expenses::Table, Expenses::GroupId)
                            .to(Groups::Table, Groups::Id)
                            .on_delete(ForeignKeyAction::SetNull),
                    )
                    .to_owned(),
            )
            .await?;

        manager
            .create_table(
                Table::create()
                    .table(ExpenseSplits::Table)
                    .if_not_exists()
                    .col(pk_auto(ExpenseSplits::Id))
                    .col(integer(ExpenseSplits::ExpenseId).not_null())
                    .col(integer(ExpenseSplits::UserId).not_null())
                    .col(double(ExpenseSplits::Amount))
                    .foreign_key(
                        ForeignKey::create()
                            .from(ExpenseSplits::Table, ExpenseSplits::ExpenseId)
                            .to(Expenses::Table, Expenses::Id)
                            .on_delete(ForeignKeyAction::Cascade),
                    )
                    .foreign_key(
                        ForeignKey::create()
                            .from(ExpenseSplits::Table, ExpenseSplits::UserId)
                            .to(Users::Table, Users::Id),
                    )
                    .to_owned(),
            )
            .await?;

        manager
            .create_table(
                Table::create()
                    .table(Payments::Table)
                    .if_not_exists()
                    .col(pk_auto(Payments::Id))
                    .col(integer(Payments::FromId).not_null())
                    .col(integer(Payments::ToId).not_null())
                    .col(double(Payments::Amount).not_null())
                    .col(integer(Payments::GroupId))
                    .col(
                        timestamp(Payments::CreatedAt)
                            .not_null()
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

    async fn down(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        manager.drop_table(Table::drop().table(Payments::Table).to_owned()).await?;
        manager.drop_table(Table::drop().table(ExpenseSplits::Table).to_owned()).await?;
        manager.drop_table(Table::drop().table(Expenses::Table).to_owned()).await?;
        manager.drop_table(Table::drop().table(GroupMembers::Table).to_owned()).await?;
        manager.drop_table(Table::drop().table(Groups::Table).to_owned()).await?;
        manager.drop_table(Table::drop().table(Users::Table).to_owned()).await?;

        Ok(())
    }
}

#[derive(DeriveIden)]
enum Users {
    Table,
    Id,
    Name,
    Email,
    PasswordHash,
    CreatedAt,
}

#[derive(DeriveIden)]
enum Groups {
    Table,
    Id,
    Name,
    CreatedAt,
}

#[derive(DeriveIden)]
enum GroupMembers {
    Table,
    Id,
    GroupId,
    UserId,
}

#[derive(DeriveIden)]
enum Expenses {
    Table,
    Id,
    Description,
    Amount,
    PaidBy,
    GroupId,
    SplitType,
    CreatedAt,
}

#[derive(DeriveIden)]
enum ExpenseSplits {
    Table,
    Id,
    ExpenseId,
    UserId,
    Amount,
}

#[derive(DeriveIden)]
enum Payments {
    Table,
    Id,
    FromId,
    ToId,
    Amount,
    GroupId,
    CreatedAt,
}
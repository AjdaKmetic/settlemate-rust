use sea_orm_migration::{
    prelude::*,
    schema::{double, integer_null},
};

#[derive(DeriveMigrationName)]
pub struct Migration;

#[async_trait::async_trait]
impl MigrationTrait for Migration {
    async fn up(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        manager
            .alter_table(
                Table::alter()
                    .table(Expenses::Table)
                    .add_column(integer_null(Expenses::AmountCents))
                    .to_owned(),
            )
            .await?;

        manager
            .alter_table(
                Table::alter()
                    .table(ExpenseSplits::Table)
                    .add_column(integer_null(ExpenseSplits::AmountCents))
                    .to_owned(),
            )
            .await?;

        manager
            .alter_table(
                Table::alter()
                    .table(Payments::Table)
                    .add_column(integer_null(Payments::AmountCents))
                    .to_owned(),
            )
            .await?;

        manager
            .get_connection()
            .execute_unprepared(
                "UPDATE expenses
                 SET amount_cents = CAST(ROUND(amount * 100) AS INTEGER)",
            )
            .await?;

        manager
            .get_connection()
            .execute_unprepared(
                "UPDATE expense_splits
                 SET amount_cents = CAST(ROUND(amount * 100) AS INTEGER)",
            )
            .await?;

        manager
            .get_connection()
            .execute_unprepared(
                "UPDATE payments
                 SET amount_cents = CAST(ROUND(amount * 100) AS INTEGER)",
            )
            .await?;

        manager
            .alter_table(
                Table::alter()
                    .table(Expenses::Table)
                    .drop_column(Expenses::Amount)
                    .to_owned(),
            )
            .await?;

        manager
            .alter_table(
                Table::alter()
                    .table(ExpenseSplits::Table)
                    .drop_column(ExpenseSplits::Amount)
                    .to_owned(),
            )
            .await?;

        manager
            .alter_table(
                Table::alter()
                    .table(Payments::Table)
                    .drop_column(Payments::Amount)
                    .to_owned(),
            )
            .await?;

        Ok(())
    }

    async fn down(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        manager
            .alter_table(
                Table::alter()
                    .table(Expenses::Table)
                    .add_column(double(Expenses::Amount).default(0.0))
                    .to_owned(),
            )
            .await?;

        manager
            .alter_table(
                Table::alter()
                    .table(ExpenseSplits::Table)
                    .add_column(double(ExpenseSplits::Amount).default(0.0))
                    .to_owned(),
            )
            .await?;

        manager
            .alter_table(
                Table::alter()
                    .table(Payments::Table)
                    .add_column(double(Payments::Amount).default(0.0))
                    .to_owned(),
            )
            .await?;

        manager
            .get_connection()
            .execute_unprepared(
                "UPDATE expenses
                 SET amount = amount_cents / 100.0",
            )
            .await?;

        manager
            .get_connection()
            .execute_unprepared(
                "UPDATE expense_splits
                 SET amount = amount_cents / 100.0",
            )
            .await?;

        manager
            .get_connection()
            .execute_unprepared(
                "UPDATE payments
                 SET amount = amount_cents / 100.0",
            )
            .await?;

        manager
            .alter_table(
                Table::alter()
                    .table(Payments::Table)
                    .drop_column(Payments::AmountCents)
                    .to_owned(),
            )
            .await?;

        manager
            .alter_table(
                Table::alter()
                    .table(ExpenseSplits::Table)
                    .drop_column(ExpenseSplits::AmountCents)
                    .to_owned(),
            )
            .await?;

        manager
            .alter_table(
                Table::alter()
                    .table(Expenses::Table)
                    .drop_column(Expenses::AmountCents)
                    .to_owned(),
            )
            .await?;

        Ok(())
    }
}

#[derive(DeriveIden)]
enum Expenses {
    Table,
    Amount,
    AmountCents,
}

#[derive(DeriveIden)]
enum ExpenseSplits {
    Table,
    Amount,
    AmountCents,
}

#[derive(DeriveIden)]
enum Payments {
    Table,
    Amount,
    AmountCents,
}
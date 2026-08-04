use sea_orm::{
    ActiveModelTrait, ColumnTrait, DatabaseConnection, EntityTrait, QueryFilter, Set,
    TransactionTrait,
};

use crate::entities::{expense_splits, expenses};

pub struct NewSplit {
    pub user_id: i32,
    pub amount_cents: i64,
}

pub async fn create_expense(
    db: &DatabaseConnection,
    description: String,
    amount_cents: i64,
    paid_by: i32,
    splits: Vec<NewSplit>,
) -> Result<(), sea_orm::DbErr> {
    let transaction = db.begin().await?;

    let expense = expenses::ActiveModel {
        description: Set(description),
        amount_cents: Set(amount_cents),
        paid_by: Set(paid_by),
        group_id: Set(None),
        split_type: Set("equal".to_string()),
        created_at: Set(chrono::Utc::now()),
        ..Default::default()
    };

    let expense = expense.insert(&transaction).await?;

    for split in splits {
        let split_model = expense_splits::ActiveModel {
            expense_id: Set(expense.id),
            user_id: Set(split.user_id),
            amount_cents: Set(split.amount_cents),
            ..Default::default()
        };

        split_model.insert(&transaction).await?;
    }

    transaction.commit().await?;

    Ok(())
}

pub async fn get_balance(db: &DatabaseConnection, user_id: i32) -> Result<i64, sea_orm::DbErr> {
    let paid: i64 = expenses::Entity::find()
        .filter(expenses::Column::PaidBy.eq(user_id))
        .all(db)
        .await?
        .iter()
        .map(|expense| expense.amount_cents)
        .sum();

    let owed: i64 = expense_splits::Entity::find()
        .filter(expense_splits::Column::UserId.eq(user_id))
        .all(db)
        .await?
        .iter()
        .map(|split| split.amount_cents)
        .sum();

    Ok(paid - owed)
}

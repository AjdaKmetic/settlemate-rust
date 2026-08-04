use sea_orm::{ActiveModelTrait, ColumnTrait, DatabaseConnection, EntityTrait, QueryFilter, Set};

use crate::entities::{expense_splits, expenses};

pub struct NewSplit {
    pub user_id: i32,
    pub amount: f64,
}

pub async fn create_expense(
    db: &DatabaseConnection,
    description: String,
    amount: f64,
    paid_by: i32,
    splits: Vec<NewSplit>,
) -> Result<(), sea_orm::DbErr> {
    let expense = expenses::ActiveModel {
        description: Set(description),
        amount: Set(amount),
        paid_by: Set(paid_by),
        group_id: Set(None),
        split_type: Set("equal".to_string()),
        created_at: Set(chrono::Utc::now()),
        ..Default::default()
    };

    let expense = expense.insert(db).await?;

    for split in splits {
        let split_model = expense_splits::ActiveModel {
            expense_id: Set(expense.id),
            user_id: Set(split.user_id),
            amount: Set(split.amount),
            ..Default::default()
        };
        split_model.insert(db).await?;
    }

    Ok(())
}

pub async fn get_balance(db: &DatabaseConnection, user_id: i32) -> Result<f64, sea_orm::DbErr> {
    let paid: f64 = expenses::Entity::find()
        .filter(expenses::Column::PaidBy.eq(user_id))
        .all(db)
        .await?
        .iter()
        .map(|e| e.amount)
        .sum();

    let owed: f64 = expense_splits::Entity::find()
        .filter(expense_splits::Column::UserId.eq(user_id))
        .all(db)
        .await?
        .iter()
        .map(|s| s.amount)
        .sum();
    println!("get_balance: paid = {}, owed = {}", paid, owed);
    Ok(paid - owed)
}

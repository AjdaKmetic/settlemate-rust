use sea_orm::{ActiveModelTrait, DatabaseConnection, Set};

use crate::entities::payments;

pub async fn create_payment(
    db: &DatabaseConnection,
    from_id: i32,
    to_id: i32,
    amount_cents: i64,
) -> Result<(), sea_orm::DbErr> {
    let payment = payments::ActiveModel {
        from_id: Set(from_id),
        to_id: Set(to_id),
        amount_cents: Set(amount_cents),
        group_id: Set(None),
        created_at: Set(chrono::Utc::now()),
        ..Default::default()
    };

    payment.insert(db).await?;

    Ok(())
}

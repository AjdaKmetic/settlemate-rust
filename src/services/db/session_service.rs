use chrono::{Duration, Utc};
use sea_orm::{ActiveModelTrait, DatabaseConnection, DbErr, Set};
use uuid::Uuid;

use crate::entities::sessions;

pub async fn create_session(db: &DatabaseConnection, user_id: i32) -> Result<String, DbErr> {
    let token = Uuid::new_v4().to_string();
    let now = Utc::now();

    let session = sessions::ActiveModel {
        token: Set(token.clone()),
        user_id: Set(user_id),
        created_at: Set(now),
        expires_at: Set(now + Duration::days(7)),
    };

    session.insert(db).await?;

    Ok(token)
}

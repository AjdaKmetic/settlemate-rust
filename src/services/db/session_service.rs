use chrono::{Duration, Utc};
use sea_orm::{ActiveModelTrait, DatabaseConnection, DbErr, EntityTrait, Set};
use uuid::Uuid;

use crate::entities::{sessions, users};

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

pub async fn find_user_by_session_token(
    db: &DatabaseConnection,
    token: &str,
) -> Result<Option<users::Model>, DbErr> {
    let session = sessions::Entity::find_by_id(token.to_string())
        .one(db)
        .await?;

    let Some(session) = session else {
        return Ok(None);
    };

    if session.expires_at <= Utc::now() {
        sessions::Entity::delete_by_id(session.token)
            .exec(db)
            .await?;

        return Ok(None);
    }

    users::Entity::find_by_id(session.user_id).one(db).await
}

pub async fn delete_session(db: &DatabaseConnection, token: &str) -> Result<(), DbErr> {
    sessions::Entity::delete_by_id(token.to_string())
        .exec(db)
        .await?;

    Ok(())
}

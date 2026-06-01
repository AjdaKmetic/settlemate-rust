use crate::services::db::auth_service::{hash_password, verify_password};
use crate::{entities::users, models::user::UserId};
use sea_orm::{
    ActiveModelTrait, ColumnTrait, DatabaseConnection, DeleteResult, EntityTrait, IntoActiveModel,
    QueryFilter, Set,
};
use users::Entity as Users;

// sign up
pub async fn create_user(
    db: &DatabaseConnection,
    name: &str,
    email: &str,
    password: &str,
) -> Result<users::Model, sea_orm::DbErr> {
    let password_hash = hash_password(password);
    let new_user = users::ActiveModel {
        name: Set(name.to_string()),
        email: Set(email.to_string()),
        password_hash: Set(password_hash),
        ..Default::default()
    };

    new_user.insert(db).await
}

pub async fn get_all_users(db: &DatabaseConnection) -> Result<Vec<users::Model>, sea_orm::DbErr> {
    Users::find().all(db).await
}

pub async fn find_user_by_id(
    db: &DatabaseConnection,
    id: UserId,
) -> Result<Option<users::Model>, sea_orm::DbErr> {
    Users::find_by_id(id).one(db).await
}

pub async fn find_user_by_email(
    db: &DatabaseConnection,
    email: &str,
) -> Result<Option<users::Model>, sea_orm::DbErr> {
    Users::find()
        .filter(users::Column::Email.eq(email))
        .one(db)
        .await
}

// log in
pub async fn login_user(
    db: &DatabaseConnection,
    email: &str,
    password: &str,
) -> Result<Option<users::Model>, sea_orm::DbErr> {
    // poišči userja po emailu
    let user = find_user_by_email(db, email).await;

    // če user obstaja, preveri password (vrni user ali None)
    match user {
        Ok(Some(user)) => {
            if verify_password(password, &user.password_hash) {
                return Ok(Some(user));
            } else {
                return Ok(None);
            }
        }

        Ok(None) => {
            return Ok(None);
        }

        Err(error) => {
            return Err(error);
        }
    }
}

pub async fn delete_user(
    db: &DatabaseConnection,
    id: UserId,
) -> Result<DeleteResult, sea_orm::DbErr> {
    Users::delete_by_id(id).exec(db).await
}

pub async fn update_user_name(
    db: &DatabaseConnection,
    id: UserId,
    new_name: &str,
) -> Result<Option<users::Model>, sea_orm::DbErr> {
    let user = find_user_by_id(db, id).await?;

    match user {
        Some(user) => {
            let mut active_user = user.into_active_model();
            active_user.name = Set(new_name.to_string());

            let updated_user = active_user.update(db).await?;
            Ok(Some(updated_user))
        }
        None => Ok(None),
    }
}

pub async fn update_user_email(
    db: &DatabaseConnection,
    id: UserId,
    new_email: &str,
) -> Result<Option<users::Model>, sea_orm::DbErr> {
    let user = find_user_by_id(db, id).await?;

    match user {
        Some(user) => {
            let mut active_user = user.into_active_model();
            active_user.email = Set(new_email.to_string());

            let updated_user = active_user.update(db).await?;
            Ok(Some(updated_user))
        }
        None => Ok(None),
    }
}

pub async fn update_user_password(
    db: &DatabaseConnection,
    id: UserId,
    new_password: &str,
) -> Result<Option<users::Model>, sea_orm::DbErr> {
    let user = find_user_by_id(db, id).await?;

    match user {
        Some(user) => {
            let mut active_user = user.into_active_model();
            active_user.password_hash = Set(hash_password(new_password));

            let updated_user = active_user.update(db).await?;
            Ok(Some(updated_user))
        }
        None => Ok(None),
    }
}

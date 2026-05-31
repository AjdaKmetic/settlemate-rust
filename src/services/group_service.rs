use crate::entities::{group_members, groups, users};
use sea_orm::{
    ActiveModelTrait, ColumnTrait, DatabaseConnection, DeleteResult, EntityTrait, IntoActiveModel,
    QueryFilter, Set,
};

use groups::Entity as Groups;
use group_members::Entity as GroupMembers;
use users::Entity as Users;

pub async fn create_group(
    db: &DatabaseConnection,
    name: &str,
) -> Result<groups::Model, sea_orm::DbErr> {
    let new_group = groups::ActiveModel {
        name: Set(name.to_string()),
        ..Default::default()
    };

    new_group.insert(db).await
}

pub async fn get_all_groups(
    db: &DatabaseConnection,
) -> Result<Vec<groups::Model>, sea_orm::DbErr> {
    Groups::find().all(db).await
}

pub async fn find_group_by_id(
    db: &DatabaseConnection,
    id: i32,
) -> Result<Option<groups::Model>, sea_orm::DbErr> {
    Groups::find_by_id(id).one(db).await
}

pub async fn delete_group(
    db: &DatabaseConnection,
    id: i32,
) -> Result<DeleteResult, sea_orm::DbErr> {
    Groups::delete_by_id(id).exec(db).await
}

pub async fn update_group_name(
    db: &DatabaseConnection,
    id: i32,
    new_name: &str,
) -> Result<Option<groups::Model>, sea_orm::DbErr> {
    let group = find_group_by_id(db, id).await?;

    match group {
        Some(group) => {
            let mut active_group = group.into_active_model();
            active_group.name = Set(new_name.to_string());

            let updated_group = active_group.update(db).await?;
            Ok(Some(updated_group))
        }
        None => Ok(None),
    }
}

pub async fn add_member_to_group(
    db: &DatabaseConnection,
    group_id: i32,
    user_id: i32,
) -> Result<group_members::Model, sea_orm::DbErr> {
    let new_member = group_members::ActiveModel {
        group_id: Set(group_id),
        user_id: Set(user_id),
        ..Default::default()
    };

    new_member.insert(db).await
}

pub async fn remove_member_from_group(
    db: &DatabaseConnection,
    group_id: i32,
    user_id: i32,
) -> Result<DeleteResult, sea_orm::DbErr> {
    GroupMembers::delete_many()
        .filter(group_members::Column::GroupId.eq(group_id))
        .filter(group_members::Column::UserId.eq(user_id))
        .exec(db)
        .await
}

pub async fn get_group_members(
    db: &DatabaseConnection,
    group_id: i32,
) -> Result<Vec<users::Model>, sea_orm::DbErr> {
    Users::find()
        .inner_join(group_members::Entity)
        .filter(group_members::Column::GroupId.eq(group_id))
        .all(db)
        .await
}
use sea_orm::{
    ActiveModelTrait, ColumnTrait, Condition, DatabaseConnection, DbErr, EntityTrait, QueryFilter,
    QueryOrder, Set,
};

use crate::entities::{friendships, users};

fn ordered_pair(first_user_id: i32, second_user_id: i32) -> (i32, i32) {
    if first_user_id < second_user_id {
        (first_user_id, second_user_id)
    } else {
        (second_user_id, first_user_id)
    }
}

pub async fn friendship_exists(
    db: &DatabaseConnection,
    first_user_id: i32,
    second_user_id: i32,
) -> Result<bool, DbErr> {
    let (user_id, friend_id) = ordered_pair(first_user_id, second_user_id);

    let friendship = friendships::Entity::find()
        .filter(friendships::Column::UserId.eq(user_id))
        .filter(friendships::Column::FriendId.eq(friend_id))
        .one(db)
        .await?;

    Ok(friendship.is_some())
}

pub async fn add_friendship(
    db: &DatabaseConnection,
    first_user_id: i32,
    second_user_id: i32,
) -> Result<bool, DbErr> {
    if first_user_id == second_user_id {
        return Ok(false);
    }

    if friendship_exists(db, first_user_id, second_user_id).await? {
        return Ok(false);
    }

    let (user_id, friend_id) = ordered_pair(first_user_id, second_user_id);

    let friendship = friendships::ActiveModel {
        user_id: Set(user_id),
        friend_id: Set(friend_id),
        ..Default::default()
    };

    friendship.insert(db).await?;

    Ok(true)
}

pub async fn get_friends(
    db: &DatabaseConnection,
    user_id: i32,
) -> Result<Vec<users::Model>, DbErr> {
    let friendships = friendships::Entity::find()
        .filter(
            Condition::any()
                .add(friendships::Column::UserId.eq(user_id))
                .add(friendships::Column::FriendId.eq(user_id)),
        )
        .all(db)
        .await?;

    let friend_ids: Vec<i32> = friendships
        .into_iter()
        .map(|friendship| {
            if friendship.user_id == user_id {
                friendship.friend_id
            } else {
                friendship.user_id
            }
        })
        .collect();

    if friend_ids.is_empty() {
        return Ok(Vec::new());
    }

    users::Entity::find()
        .filter(users::Column::Id.is_in(friend_ids))
        .order_by_asc(users::Column::Name)
        .all(db)
        .await
}

pub async fn delete_friendship(
    db: &DatabaseConnection,
    first_user_id: i32,
    second_user_id: i32,
) -> Result<bool, DbErr> {
    let (user_id, friend_id) = ordered_pair(first_user_id, second_user_id);

    let result = friendships::Entity::delete_many()
        .filter(friendships::Column::UserId.eq(user_id))
        .filter(friendships::Column::FriendId.eq(friend_id))
        .exec(db)
        .await?;

    Ok(result.rows_affected > 0)
}

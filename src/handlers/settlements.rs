use axum::{
    extract::{Path, State},
    http::{HeaderName, HeaderValue, StatusCode},
    response::{IntoResponse, Response},
};

use crate::app::state::AppState;
use crate::handlers::current_user::CurrentUser;
use crate::services::db::expense_service::get_balance_with_friend;
use crate::services::db::friendship_service::friendship_exists;
use crate::services::db::payment_service::create_payment;

pub async fn settle_up(
    current_user: CurrentUser,
    State(state): State<AppState>,
    Path(friend_id): Path<i32>,
) -> Response {
    let is_friend = match friendship_exists(&state.db, current_user.id, friend_id).await {
        Ok(is_friend) => is_friend,

        Err(error) => {
            return (
                StatusCode::INTERNAL_SERVER_ERROR,
                format!("Error checking friendship: {error}"),
            )
                .into_response();
        }
    };

    if !is_friend {
        return (StatusCode::BAD_REQUEST, "This user is not your friend.").into_response();
    }

    let balance = match get_balance_with_friend(&state.db, current_user.id, friend_id).await {
        Ok(balance) => balance,

        Err(error) => {
            return (
                StatusCode::INTERNAL_SERVER_ERROR,
                format!("Error loading balance: {error}"),
            )
                .into_response();
        }
    };

    if balance == 0 {
        return (StatusCode::BAD_REQUEST, "You are already settled up.").into_response();
    }

    let (from_id, to_id) = if balance < 0 {
        (current_user.id, friend_id)
    } else {
        (friend_id, current_user.id)
    };

    match create_payment(&state.db, from_id, to_id, balance.abs()).await {
        Ok(_) => {
            let mut response = StatusCode::OK.into_response();

            response.headers_mut().insert(
                HeaderName::from_static("hx-refresh"),
                HeaderValue::from_static("true"),
            );

            response
        }

        Err(error) => (
            StatusCode::INTERNAL_SERVER_ERROR,
            format!("Error recording settlement: {error}"),
        )
            .into_response(),
    }
}

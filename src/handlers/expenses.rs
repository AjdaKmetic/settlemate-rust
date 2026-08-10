use askama::Template;
use axum::{
    extract::State,
    http::StatusCode,
    response::{Html, IntoResponse},
};
use axum_extra::extract::Form;
use serde::Deserialize;

use crate::app::state::AppState;
use crate::entities::users;
use crate::handlers::current_user::CurrentUser;
use crate::services::db::expense_service::{NewSplit, create_expense};
use crate::services::db::friendship_service::get_friends;

#[derive(Template)]
#[template(path = "new_expense.html")]
struct NewExpenseTemplate {
    friends: Vec<users::Model>,
}

#[derive(Deserialize)]
pub struct NewExpenseForm {
    description: String,
    amount: f64,
    #[serde(default)]
    friend_ids: Vec<i32>,
}

pub async fn new_expense(
    current_user: CurrentUser,
    State(state): State<AppState>,
) -> impl IntoResponse {
    let users = get_friends(&state.db, current_user.id)
        .await
        .unwrap_or_default();

    let template = NewExpenseTemplate { friends: users };

    Html(template.render().unwrap())
}

pub async fn add_expense(
    current_user: CurrentUser,
    State(state): State<AppState>,
    Form(form): Form<NewExpenseForm>,
) -> impl IntoResponse {
    if form.amount <= 0.0 {
        return (StatusCode::BAD_REQUEST, "Amount must be a positive number").into_response();
    }

    let friends = match get_friends(&state.db, current_user.id).await {
        Ok(friends) => friends,
        Err(_) => {
            return (StatusCode::INTERNAL_SERVER_ERROR, "Error fetching friends").into_response();
        }
    };

    if form.friend_ids.is_empty() {
    return (
        StatusCode::BAD_REQUEST,
        "Select at least one friend",
    )
        .into_response();
}

    let all_are_friends = form
        .friend_ids
        .iter()
        .all(|id| friends.iter().any(|friend| friend.id == *id));

    if !all_are_friends {
        return (
            StatusCode::BAD_REQUEST,
            "One or more selected friends are not in your friend list",
        )
            .into_response();
    }

    let amount_cents = (form.amount * 100.0).round() as i64;

    let participant_counts = form.friend_ids.len() + 1;
    let friends_share = amount_cents / participant_counts as i64;

    let current_user_share = amount_cents - (friends_share * form.friend_ids.len() as i64);

    let mut splits = Vec::new();
    splits.push(NewSplit {
        user_id: current_user.id,
        amount_cents: current_user_share,
    });

    for friends_id in &form.friend_ids {
        splits.push(NewSplit {
            user_id: *friends_id,
            amount_cents: friends_share,
        });
    }

    match create_expense(
        &state.db,
        form.description,
        amount_cents,
        current_user.id,
        splits,
    )
    .await
    {
        Ok(_) => (StatusCode::OK, "").into_response(),

        Err(error) => (
            StatusCode::INTERNAL_SERVER_ERROR,
            format!("Error adding expense: {error}"),
        )
            .into_response(),
    }
}

pub async fn close_expense_modal(_current_user: CurrentUser) -> Html<&'static str> {
    Html("")
}

use askama::Template;
use axum::{
    Form,
    extract::{Query, State},
    http::StatusCode,
    response::{Html, IntoResponse},
};
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
    current_user_id: i32,
}

#[derive(Deserialize)]
pub struct NewExpenseForm {
    description: String,
    amount: f64,
    paid_by: i32,
    split_with: i32,
}

pub async fn new_expense(
    current_user: CurrentUser,
    State(state): State<AppState>,
) -> impl IntoResponse {
    let users = get_friends(&state.db, current_user.id)
        .await
        .unwrap_or_default();

    let template = NewExpenseTemplate {
        friends: users,
        current_user_id: current_user.id,
    };

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

    let split_user_is_friend = friends.iter().any(|friend| friend.id == form.split_with);

    if !split_user_is_friend {
        return (
            StatusCode::BAD_REQUEST,
            "The user you are trying to split with is not your friend",
        )
            .into_response();
    }

    if form.paid_by != current_user.id && form.paid_by != form.split_with {
        return (
            StatusCode::BAD_REQUEST,
            "The payer must be you or the selected friend.",
        )
            .into_response();
    }

    let amount_cents = (form.amount * 100.0).round() as i64;

    let current_user_share = amount_cents / 2;
    let friend_share = amount_cents - current_user_share;

    let splits = vec![
        NewSplit {
            user_id: current_user.id,
            amount_cents: current_user_share,
        },
        NewSplit {
            user_id: form.split_with,
            amount_cents: friend_share,
        },
    ];

    match create_expense(
        &state.db,
        form.description,
        amount_cents,
        form.paid_by,
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

#[derive(Template)]
#[template(path = "payer_select.html")]
struct PayerSelectTemplate {
    current_user_id: i32,
    friend: users::Model,
}

#[derive(Deserialize)]
pub struct PayerOptionsQuery {
    split_with: i32,
}

pub async fn payer_options(
    State(state): State<AppState>,
    current_user: CurrentUser,
    Query(query): Query<PayerOptionsQuery>,
) -> impl IntoResponse {
    let friends = match get_friends(&state.db, current_user.id).await {
        Ok(friends) => friends,
        Err(error) => {
            return (
                StatusCode::INTERNAL_SERVER_ERROR,
                format!("Error loading friends: {error}"),
            )
                .into_response();
        }
    };

    let friend = match friends
        .into_iter()
        .find(|friend| friend.id == query.split_with)
    {
        Some(friend) => friend,
        None => {
            return (StatusCode::BAD_REQUEST, "Selected user is not your friend.").into_response();
        }
    };

    let template = PayerSelectTemplate {
        current_user_id: current_user.id,
        friend,
    };

    match template.render() {
        Ok(html) => Html(html).into_response(),
        Err(error) => {
            (StatusCode::INTERNAL_SERVER_ERROR, format!("Error: {error}")).into_response()
        }
    }
}

pub async fn close_expense_modal(_current_user: CurrentUser) -> Html<&'static str> {
    Html("")
}

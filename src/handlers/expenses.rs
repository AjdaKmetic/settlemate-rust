use askama::Template;
use axum::{
    Form,
    extract::State,
    http::StatusCode,
    response::{Html, IntoResponse},
};
use serde::Deserialize;

use crate::app::state::AppState;
use crate::entities::users;
use crate::handlers::current_user::CurrentUser;
use crate::services::db::expense_service::{NewSplit, create_expense};
use crate::services::db::user_service::get_all_users;

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
    let users = get_all_users(&state.db).await.unwrap_or_default();

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
        Ok(_) => (StatusCode::OK, "Expense added successfully").into_response(),

        Err(error) => (
            StatusCode::INTERNAL_SERVER_ERROR,
            format!("Error adding expense: {error}"),
        )
            .into_response(),
    }
}

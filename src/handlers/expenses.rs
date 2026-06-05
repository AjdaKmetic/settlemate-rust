use askama::Template;
use axum::{
    extract::State,
    response::{Html, IntoResponse},
    Form,
    http::StatusCode,
};
use serde::Deserialize;

use crate::app::{current_user, state::AppState};
use crate::entities::users;
use crate::services::db::user_service::get_all_users;
use crate::services::db::expense_service::{
    create_expense, NewSplit,
};

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

pub async fn new_expense(State(state): State<AppState>) -> impl IntoResponse {
    let users = get_all_users(&state.db).await.unwrap_or_default();
    let current_user_id = users
        .iter()
        .find(|u| u.name == "Ajda")
        .map(|u| u.id)
        .unwrap_or(1);

    let template = NewExpenseTemplate { friends: users, current_user_id };

    Html(template.render().unwrap())
}

pub async fn add_expense(
    State(state): State<AppState>,
    Form(form): Form<NewExpenseForm>,
) -> impl IntoResponse {
    let users = get_all_users(&state.db).await.unwrap_or_default();
    let current_user_id = users
        .iter()
        .find(|u| u.name == "Ajda")
        .map(|u| u.id)
        .unwrap_or(1);
    let other_user_id = users.iter().find(|u| u.id != current_user_id).map(|u| u.id).unwrap_or(form.paid_by);
    
    let half = form.amount / 2.0;

    let splits = vec! [
        NewSplit {
            user_id: current_user_id,
            amount: half,
        },
        NewSplit {
            user_id: form.split_with,
            amount: half,
        },
    ];

    match create_expense(
        &state.db,
        form.description,
        form.amount,
        form.paid_by,
        splits,
    )
    .await {
        Ok(_) => {
            print!("Expense created successfully: paid_by = {}, amount = {}", form.paid_by, form.amount);
            Html(String::new()).into_response()
        }
        Err(e) => {
            println!("Error creating expense: {e}");
            (StatusCode::INTERNAL_SERVER_ERROR, format!("Error {e} !")).into_response()
        }
    }


    
}


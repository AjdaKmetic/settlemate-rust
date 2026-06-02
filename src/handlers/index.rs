use askama::Template;
use axum::{
    extract::State,
    http::StatusCode,
    response::{Html, IntoResponse},
};

use crate::{
    app::state::AppState,
    entities::groups,
    handlers::friends::{FriendView, users_to_friend_views},
    services::db::group_service::get_all_groups,
    services::db::user_service::get_all_users,
    services::domain::balance::Balance,
};

#[derive(Template)]
#[template(path = "index.html")]
pub struct IndexTemplate {
    pub username: String,
    pub balance_state_class: &'static str,
    pub balance_label: &'static str,
    pub balance_secondary: &'static str,
    pub formatted_balance: String,
    pub active_tab: String,
    pub friends: Vec<FriendView>,
    pub groups: Vec<groups::Model>,
}

#[derive(Template)]
#[template(path = "tab_shell.html")]
pub struct TabShellTemplate {
    pub active_tab: String,
    pub friends: Vec<FriendView>,
    pub groups: Vec<groups::Model>,
}

impl IndexTemplate {
    fn new(username: String, balance: f64, groups: Vec<groups::Model>) -> Self {
        let is_positive = balance > 0.005;
        let is_negative = balance < -0.005;

        let (balance_state_class, balance_label, balance_secondary) = if is_positive {
            (
                "balance-positive",
                "Overall, you are owed",
                "Across all shared expenses",
            )
        } else if is_negative {
            (
                "balance-negative",
                "Overall, you owe",
                "Across all shared expenses",
            )
        } else {
            (
                "balance-neutral",
                "Overall, you are settled up",
                "No outstanding shared expenses",
            )
        };

        let formatted_balance = if is_negative {
            format!("-€{:.2}", balance.abs())
        } else {
            format!("€{:.2}", balance.abs())
        };

        Self {
            username,
            balance_state_class,
            balance_label,
            balance_secondary,
            formatted_balance,
            active_tab: "groups".to_string(),
            friends: Vec::new(),
            groups,
        }
    }
}

fn current_user_balance(state: &AppState) -> (String, f64) {
    let data = state.data.lock().unwrap();
    let Some(current_user_id) = data.current_user_id else {
        return ("Ajda".to_string(), 120.50);
    };

    let username = data
        .users
        .iter()
        .find(|user| user.id == current_user_id)
        .map(|user| user.name().to_string())
        .unwrap_or_else(|| "Ajda".to_string());

    let balances = Balance::balances_with_payments(&data.expenses, &data.payments);
    let balance = balances.get(&current_user_id).copied().unwrap_or(0.0);

    (username, balance)
}

pub async fn index(State(state): State<AppState>) -> impl IntoResponse {
    let (username, balance) = current_user_balance(&state);
    match get_all_groups(&state.db).await {
        Ok(groups) => {
            let template = IndexTemplate::new(username, balance, groups);
            Html(template.render().unwrap()).into_response()
        }
        Err(e) => (StatusCode::INTERNAL_SERVER_ERROR, format!("Error: {e}")).into_response(),
    }
}

fn render_tab(
    active_tab: &str,
    friends: Vec<FriendView>,
    groups: Vec<groups::Model>,
) -> Html<String> {
    let template = TabShellTemplate {
        active_tab: active_tab.to_string(),
        friends,
        groups,
    };

    Html(template.render().unwrap())
}

pub async fn tabs_friends(State(state): State<AppState>) -> impl IntoResponse {
    match get_all_users(&state.db).await {
        Ok(users) => {
            render_tab("friends", users_to_friend_views(users), Vec::new()).into_response()
        }
        Err(e) => (StatusCode::INTERNAL_SERVER_ERROR, format!("Error: {e}")).into_response(),
    }
}

pub async fn tabs_groups(State(state): State<AppState>) -> impl IntoResponse {
    match get_all_groups(&state.db).await {
        Ok(groups) => render_tab("groups", Vec::new(), groups).into_response(),
        Err(e) => (StatusCode::INTERNAL_SERVER_ERROR, format!("Error: {e}")).into_response(),
    }
}

pub async fn tabs_activity(State(_state): State<AppState>) -> impl IntoResponse {
    render_tab("activity", Vec::new(), Vec::new())
}

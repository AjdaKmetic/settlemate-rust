use askama::Template;
use axum::{
    extract::State,
    response::{Html, IntoResponse},
};

use crate::{app::state::AppState, services::domain::balance::Balance};

#[derive(Template)]
#[template(path = "index.html")]
pub struct IndexTemplate {
    pub username: String,
    pub balance_state_class: &'static str,
    pub balance_label: &'static str,
    pub balance_secondary: &'static str,
    pub formatted_balance: String,
    pub active_tab: String,
}

#[derive(Template)]
#[template(path = "tab_shell.html")]
pub struct TabShellTemplate {
    pub active_tab: String,
}

impl IndexTemplate {
    fn new(username: String, balance: f64) -> Self {
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
    let template = IndexTemplate::new(username, balance);
    Html(template.render().unwrap())
}

fn render_tab(active_tab: &str) -> Html<String> {
    let template = TabShellTemplate {
        active_tab: active_tab.to_string(),
    };

    Html(template.render().unwrap())
}

pub async fn tabs_friends(State(_state): State<AppState>) -> impl IntoResponse {
    render_tab("friends")
}

pub async fn tabs_groups(State(_state): State<AppState>) -> impl IntoResponse {
    render_tab("groups")
}

pub async fn tabs_activity(State(_state): State<AppState>) -> impl IntoResponse {
    render_tab("activity")
}

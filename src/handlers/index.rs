use askama::Template;
use axum::{
    extract::State,
    http::StatusCode,
    response::{Html, IntoResponse},
};

use crate::{
    app::state::AppState,
    entities::groups,
    handlers::current_user::CurrentUser,
    handlers::friends::{FriendView, users_to_friend_views},
    services::db::expense_service::get_balance,
    services::db::friendship_service::get_friends,
    services::db::group_service::get_all_groups,
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
    fn new(username: String, balance_cents: i64, friends: Vec<FriendView>) -> Self {
        let is_positive = balance_cents > 0;
        let is_negative = balance_cents < 0;

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

        let absolute = balance_cents.unsigned_abs();
        let euros = absolute / 100;
        let cents = absolute % 100;
        let sign = if balance_cents < 0 { "-" } else { "" };

        let formatted_balance = format!("{sign}€{euros}.{cents:02}");

        Self {
            username,
            balance_state_class,
            balance_label,
            balance_secondary,
            formatted_balance,
            active_tab: "friends".to_string(),
            friends,
            groups: Vec::new(),
        }
    }
}

pub async fn index(current_user: CurrentUser, State(state): State<AppState>) -> impl IntoResponse {
    let balance = match get_balance(&state.db, current_user.id).await {
        Ok(balance) => balance,

        Err(error) => {
            return (
                StatusCode::INTERNAL_SERVER_ERROR,
                format!("Error while loading balance: {error}"),
            )
                .into_response();
        }
    };

    let users = match get_friends(&state.db, current_user.id).await {
        Ok(users) => users,

        Err(error) => {
            return (
                StatusCode::INTERNAL_SERVER_ERROR,
                format!("Error loading friends: {error}"),
            )
                .into_response();
        }
    };

    let friends = match users_to_friend_views(&state.db, current_user.id, users).await {
        Ok(friends) => friends,

        Err(error) => {
            return (
                StatusCode::INTERNAL_SERVER_ERROR,
                format!("Error loading balances: {error}"),
            )
                .into_response();
        }
    };

    let template = IndexTemplate::new(current_user.name, balance, friends);

    Html(template.render().unwrap()).into_response()
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

pub async fn tabs_friends(
    current_user: CurrentUser,
    State(state): State<AppState>,
) -> impl IntoResponse {
    match get_friends(&state.db, current_user.id).await {
        Ok(users) => match users_to_friend_views(&state.db, current_user.id, users).await {
            Ok(friends) => render_tab("friends", friends, Vec::new()).into_response(),
            Err(error) => (
                StatusCode::INTERNAL_SERVER_ERROR,
                format!("Error loading balances: {error}"),
            )
                .into_response(),
        },

        Err(error) => {
            (StatusCode::INTERNAL_SERVER_ERROR, format!("Error: {error}")).into_response()
        }
    }
}

pub async fn tabs_groups(
    _current_user: CurrentUser,
    State(state): State<AppState>,
) -> impl IntoResponse {
    match get_all_groups(&state.db).await {
        Ok(groups) => render_tab("groups", Vec::new(), groups).into_response(),
        Err(e) => (StatusCode::INTERNAL_SERVER_ERROR, format!("Error: {e}")).into_response(),
    }
}

pub async fn tabs_activity(
    _current_user: CurrentUser,
    State(_state): State<AppState>,
) -> impl IntoResponse {
    render_tab("activity", Vec::new(), Vec::new())
}

use askama::Template;
use axum::{
    extract::{Form, State},
    http::StatusCode,
    response::{Html, IntoResponse},
};
use serde::Deserialize;

use crate::app::state::AppState;
use crate::entities::users;
use crate::services::db::user_service::{create_user, get_all_users};

pub struct FriendView {
    pub initials: String,
    pub name: String,
    pub email: String,
}

#[derive(Template)]
#[template(path = "friends.html")]
struct FriendsTemplate {
    friends: Vec<FriendView>,
}

#[derive(Template)]
#[template(path = "friend_form.html")]
struct FriendFormTemplate;

fn get_initials(name: &str) -> String {
    let mut words = name.split_whitespace();
    let first = words.next().unwrap_or_default();

    let initials = if let Some(second) = words.next() {
        first
            .chars()
            .next()
            .into_iter()
            .chain(second.chars().next())
            .collect::<String>()
    } else {
        first.chars().take(2).collect::<String>()
    };

    initials.to_uppercase()
}

pub fn user_to_friend_view(user: users::Model) -> FriendView {
    FriendView {
        initials: get_initials(&user.name),
        name: user.name,
        email: user.email,
    }
}

pub fn users_to_friend_views(users: Vec<users::Model>) -> Vec<FriendView> {
    users.into_iter().map(user_to_friend_view).collect()
}

fn render_friends_panel(users: Vec<users::Model>) -> Result<String, askama::Error> {
    FriendsTemplate {
        friends: users_to_friend_views(users),
    }
    .render()
}

#[derive(Deserialize)]
pub struct NewFriendForm {
    name: String,
    email: String,
}

pub async fn list_friends(State(state): State<AppState>) -> impl IntoResponse {
    // TODO: izriši lepšo HTML napako (npr. ločen `error.html` template).
    match get_all_users(&state.db).await {
        Ok(users) => match render_friends_panel(users) {
            Ok(html) => Html(html).into_response(),
            Err(e) => (StatusCode::INTERNAL_SERVER_ERROR, format!("Error: {e}")).into_response(),
        },
        Err(e) => (StatusCode::INTERNAL_SERVER_ERROR, format!("Error: {e}")).into_response(),
    }
}

pub async fn friend_form() -> impl IntoResponse {
    match FriendFormTemplate.render() {
        Ok(html) => Html(html).into_response(),
        Err(e) => (StatusCode::INTERNAL_SERVER_ERROR, format!("Error: {e}")).into_response(),
    }
}

pub async fn add_friend(
    State(state): State<AppState>,
    Form(form): Form<NewFriendForm>,
) -> impl IntoResponse {
    match create_user(&state.db, &form.name, &form.email, "password").await {
        Ok(_) => match get_all_users(&state.db).await {
            Ok(users) => match render_friends_panel(users) {
                Ok(html) => Html(html).into_response(),
                Err(e) => {
                    (StatusCode::INTERNAL_SERVER_ERROR, format!("Error: {e}")).into_response()
                }
            },
            Err(e) => (StatusCode::INTERNAL_SERVER_ERROR, format!("Error: {e}")).into_response(),
        },
        Err(error) => (
            StatusCode::INTERNAL_SERVER_ERROR,
            format!("Error while adding friends: {error}"),
        )
            .into_response(),
    }
}

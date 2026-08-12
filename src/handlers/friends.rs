use askama::Template;
use axum::{
    extract::{Form, Path, State},
    http::{HeaderName, HeaderValue, StatusCode},
    response::{Html, IntoResponse, Response},
};
use serde::Deserialize;

use sea_orm::DatabaseConnection;

use crate::app::state::AppState;
use crate::entities::users;
use crate::handlers::current_user::CurrentUser;
use crate::services::db::expense_service::get_balance_with_friend;
use crate::services::db::friendship_service::{add_friendship, delete_friendship, get_friends};
use crate::services::db::user_service::find_user_by_email;

pub struct FriendView {
    pub id: i32,
    pub initials: String,
    pub name: String,
    pub email: String,
    pub balance_state_class: &'static str,
    pub formatted_balance: String,
    pub balance_note: &'static str,
    pub has_balance: bool,
}

#[derive(Template)]
#[template(path = "friends.html")]
struct FriendsTemplate {
    friends: Vec<FriendView>,
}

#[derive(Template)]
#[template(path = "friend_form.html")]
struct FriendFormTemplate {
    has_error: bool,
    error_message: String,
}

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

fn user_to_friend_view(user: users::Model, balance_cents: i64) -> FriendView {
    let absolute = balance_cents.unsigned_abs();
    let euros = absolute / 100;
    let cents = absolute % 100;

    let formatted_balance = format!("€{euros}.{cents:02}");

    let (balance_state_class, balance_note) = if balance_cents > 0 {
        ("is-positive", "owes you")
    } else if balance_cents < 0 {
        ("is-negative", "you owe")
    } else {
        ("is-neutral", "settled up")
    };

    FriendView {
        id: user.id,
        initials: get_initials(&user.name),
        name: user.name,
        email: user.email,
        balance_state_class,
        formatted_balance,
        balance_note,
        has_balance: balance_cents != 0,
    }
}

pub async fn users_to_friend_views(
    db: &DatabaseConnection,
    current_user_id: i32,
    users: Vec<users::Model>,
) -> Result<Vec<FriendView>, sea_orm::DbErr> {
    let mut friends = Vec::with_capacity(users.len());

    for user in users {
        let balance_cents = get_balance_with_friend(db, current_user_id, user.id).await?;

        friends.push(user_to_friend_view(user, balance_cents));
    }

    Ok(friends)
}

async fn render_friends_panel(
    db: &DatabaseConnection,
    current_user_id: i32,
    users: Vec<users::Model>,
) -> Result<String, String> {
    let friends = users_to_friend_views(db, current_user_id, users)
        .await
        .map_err(|error| error.to_string())?;

    FriendsTemplate { friends }
        .render()
        .map_err(|error| error.to_string())
}
fn render_friend_form(has_error: bool, error_message: &str) -> Response {
    match (FriendFormTemplate {
        has_error,
        error_message: error_message.to_string(),
    })
    .render()
    {
        Ok(html) => Html(html).into_response(),
        Err(error) => (
            StatusCode::INTERNAL_SERVER_ERROR,
            format!("Error rendering friend form: {error}"),
        )
            .into_response(),
    }
}

fn render_friend_form_error(error_message: &str) -> Response {
    let mut response = render_friend_form(true, error_message);

    response.headers_mut().insert(
        HeaderName::from_static("hx-retarget"),
        HeaderValue::from_static("#friend-form-slot"),
    );

    response.headers_mut().insert(
        HeaderName::from_static("hx-reswap"),
        HeaderValue::from_static("innerHTML"),
    );

    response
}

#[derive(Deserialize)]
pub struct NewFriendForm {
    email: String,
}

pub async fn list_friends(
    current_user: CurrentUser,
    State(state): State<AppState>,
) -> impl IntoResponse {
    // TODO: izriši lepšo HTML napako (npr. ločen `error.html` template).
    match get_friends(&state.db, current_user.id).await {
        Ok(users) => match render_friends_panel(&state.db, current_user.id, users).await {
            Ok(html) => Html(html).into_response(),
            Err(e) => (StatusCode::INTERNAL_SERVER_ERROR, format!("Error: {e}")).into_response(),
        },
        Err(e) => (StatusCode::INTERNAL_SERVER_ERROR, format!("Error: {e}")).into_response(),
    }
}

pub async fn friend_form(_current_user: CurrentUser) -> Response {
    render_friend_form(false, "")
}

pub async fn add_friend(
    current_user: CurrentUser,
    State(state): State<AppState>,
    Form(form): Form<NewFriendForm>,
) -> Response {
    let friend = match find_user_by_email(&state.db, &form.email).await {
        Ok(Some(user)) => user,

        Ok(None) => {
            return render_friend_form_error("No registered user has that email.");
        }

        Err(error) => {
            return (
                StatusCode::INTERNAL_SERVER_ERROR,
                format!("Error finding user: {error}"),
            )
                .into_response();
        }
    };

    if friend.id == current_user.id {
        return render_friend_form_error("You cannot add yourself as a friend.");
    }

    match add_friendship(&state.db, current_user.id, friend.id).await {
        Ok(true) => match get_friends(&state.db, current_user.id).await {
            Ok(users) => match render_friends_panel(&state.db, current_user.id, users).await {
                Ok(html) => Html(html).into_response(),

                Err(error) => {
                    (StatusCode::INTERNAL_SERVER_ERROR, format!("Error: {error}")).into_response()
                }
            },

            Err(error) => (
                StatusCode::INTERNAL_SERVER_ERROR,
                format!("Error loading friends: {error}"),
            )
                .into_response(),
        },

        Ok(false) => render_friend_form_error("This user is already your friend."),

        Err(error) => (
            StatusCode::INTERNAL_SERVER_ERROR,
            format!("Error adding friend: {error}"),
        )
            .into_response(),
    }
}

pub async fn remove_friend(
    State(state): State<AppState>,
    current_user: CurrentUser,
    Path(friend_id): Path<i32>,
) -> impl IntoResponse {
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

    if balance != 0 {
        return (
            StatusCode::CONFLICT,
            "Settle up before removing this friend.",
        )
            .into_response();
    }
    match delete_friendship(&state.db, current_user.id, friend_id).await {
        Ok(_) => match get_friends(&state.db, current_user.id).await {
            Ok(friends) => match render_friends_panel(&state.db, current_user.id, friends).await {
                Ok(html) => Html(html).into_response(),
                Err(error) => {
                    (StatusCode::INTERNAL_SERVER_ERROR, format!("Error: {error}")).into_response()
                }
            },
            Err(error) => {
                (StatusCode::INTERNAL_SERVER_ERROR, format!("Error: {error}")).into_response()
            }
        },
        Err(error) => (
            StatusCode::INTERNAL_SERVER_ERROR,
            format!("Error removing friend: {error}"),
        )
            .into_response(),
    }
}

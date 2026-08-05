use askama::Template;
use axum::{
    Form,
    extract::State,
    http::StatusCode,
    response::{Html, IntoResponse, Response},
};
use serde::Deserialize;

use crate::{
    app::state::AppState,
    services::db::user_service::{create_user, find_user_by_email},
};

#[derive(Template)]
#[template(path = "Register.html")]
struct RegisterTemplate {
    has_error: bool,
    error_message: String,
    created: bool,
}

#[derive(Deserialize)]
pub struct RegisterForm {
    name: String,
    email: String,
    password: String,
    password_confirmation: String,
}

fn render_register(status: StatusCode, error_message: &str, created: bool) -> Response {
    let template = RegisterTemplate {
        has_error: !error_message.is_empty(),
        error_message: error_message.to_string(),
        created,
    };

    match template.render() {
        Ok(html) => (status, Html(html)).into_response(),
        Err(error) => (
            StatusCode::INTERNAL_SERVER_ERROR,
            format!("Template error: {error}"),
        )
            .into_response(),
    }
}

pub async fn register_form() -> Response {
    render_register(StatusCode::OK, "", false)
}

pub async fn register_user(
    State(state): State<AppState>,
    Form(form): Form<RegisterForm>,
) -> Response {
    let name = form.name.trim();
    let email = form.email.trim().to_lowercase();

    if name.is_empty() {
        return render_register(StatusCode::BAD_REQUEST, "Ime je obvezno.", false);
    }

    if !email.contains('@') {
        return render_register(
            StatusCode::BAD_REQUEST,
            "Vnesi veljaven e-poštni naslov.",
            false,
        );
    }

    if form.password.chars().count() < 8 {
        return render_register(
            StatusCode::BAD_REQUEST,
            "Geslo mora vsebovati najmanj 8 znakov.",
            false,
        );
    }

    if form.password != form.password_confirmation {
        return render_register(StatusCode::BAD_REQUEST, "Gesli se ne ujemata.", false);
    }

    match find_user_by_email(&state.db, &email).await {
        Ok(Some(_)) => {
            return render_register(
                StatusCode::CONFLICT,
                "Uporabnik s tem e-poštnim naslovom že obstaja.",
                false,
            );
        }
        Ok(None) => {}
        Err(error) => {
            return (
                StatusCode::INTERNAL_SERVER_ERROR,
                format!("Database error: {error}"),
            )
                .into_response();
        }
    }

    match create_user(&state.db, name, &email, &form.password).await {
        Ok(_) => render_register(StatusCode::CREATED, "", true),
        Err(error) => (
            StatusCode::INTERNAL_SERVER_ERROR,
            format!("Registration error: {error}"),
        )
            .into_response(),
    }
}

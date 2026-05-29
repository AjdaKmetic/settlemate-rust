// Primer: branje in pisanje uporabnikov v bazo (SeaORM).
//
// Ta modul je namenjen študentom kot vzorec za:
//   - dodajanje Axum poti, ki uporabljajo skupno stanje (`AppState`),
//   - branje iz baze (`get_all_users`) in pisanje v bazo (`create_user`),
//   - prikaz HTML preko Askama predloge,
//   - osnovni HTMX vzorec: oddaja obrazca POST in dodajanje fragmenta v seznam.

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
use crate::services::user_service::{create_user, get_all_users};

#[derive(Template)]
#[template(path = "users.html")]
struct UsersTemplate {
    users: Vec<users::Model>,
}

#[derive(Template)]
#[template(path = "user_item.html")]
struct UserItemTemplate {
    name: String,
    email: String,
}

#[derive(Deserialize)]
pub struct NewUserForm {
    name: String,
    email: String,
    password: String,
}

// GET /users
pub async fn list_users(State(state): State<AppState>) -> impl IntoResponse {
    // TODO: izriši lepšo HTML napako (npr. ločen `error.html` template).
    match get_all_users(&state.db).await {
        Ok(users) => Html(UsersTemplate { users }.render().unwrap()).into_response(),
        Err(e) => (StatusCode::INTERNAL_SERVER_ERROR, format!("Napaka: {e}")).into_response(),
    }
}

// POST /users — ustvari novega uporabnika iz obrazca in vrne nov <li>,
// ki ga HTMX doda v seznam (hx-swap="beforeend").
pub async fn create_user_handler(
    State(state): State<AppState>,
    Form(form): Form<NewUserForm>,
) -> impl IntoResponse {
    match create_user(&state.db, &form.name, &form.email, &form.password).await {
        Ok(model) => Html(
            UserItemTemplate {
                name: model.name,
                email: model.email,
            }
            .render()
            .unwrap(),
        )
        .into_response(),
        Err(e) => {
            // TODO: izriši lepšo HTML napako (npr. ločen `error.html` template).
            (StatusCode::INTERNAL_SERVER_ERROR, format!("Napaka: {e}")).into_response()
        }
    }
}

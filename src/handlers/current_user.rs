use axum::{
    extract::FromRequestParts,
    http::{StatusCode, request::Parts},
    response::{IntoResponse, Redirect, Response},
};
use axum_extra::extract::cookie::CookieJar;

use crate::{app::state::AppState, services::db::session_service::find_user_by_session_token};

#[derive(Clone, Debug)]
pub struct CurrentUser {
    pub id: i32,
    pub name: String,
    pub email: String,
}

impl FromRequestParts<AppState> for CurrentUser {
    type Rejection = Response;

    async fn from_request_parts(
        parts: &mut Parts,
        state: &AppState,
    ) -> Result<Self, Self::Rejection> {
        let jar = CookieJar::from_headers(&parts.headers);

        let token = match jar.get("settlemate_session") {
            Some(cookie) => cookie.value().to_string(),
            None => {
                return Err(Redirect::to("/login").into_response());
            }
        };

        match find_user_by_session_token(&state.db, &token).await {
            Ok(Some(user)) => Ok(Self {
                id: user.id,
                name: user.name,
                email: user.email,
            }),

            Ok(None) => Err(Redirect::to("/login").into_response()),

            Err(error) => {
                eprintln!("Napaka pri preverjanju seje: {error}");

                Err((
                    StatusCode::INTERNAL_SERVER_ERROR,
                    "Napaka pri preverjanju prijave.",
                )
                    .into_response())
            }
        }
    }
}

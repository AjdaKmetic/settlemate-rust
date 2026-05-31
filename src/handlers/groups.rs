// povezava med URL-jem, service funkcijo in HTML tamplatom

// GET /groups

use askama::Template;
use axum::{
    extract::State,
    http::StatusCode,
    response::{Html, IntoResponse},
};

use crate::app::state::AppState;
use crate::entities::groups;
use crate::services::group_service::get_all_groups;

#[derive(Template)]
#[template(path = "groups.html")]
struct GroupsTemplate {
    groups: Vec<groups::Model>,
}

pub async fn list_groups(State(state): State<AppState>) -> impl IntoResponse {
    match get_all_groups(&state.db).await {
        Ok(groups) => {
            Html(GroupsTemplate { groups }.render().unwrap()).into_response()
        }
        Err(error) => {
            (
                StatusCode::INTERNAL_SERVER_ERROR,
                format!("Napaka pri branju skupin: {error}"),
            )
                .into_response()
        }
    }
}
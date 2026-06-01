use askama::Template;
use axum::{
    extract::State,
    http::StatusCode,
    response::{Html, IntoResponse},
};

use crate::app::state::AppState;
use crate::entities::groups;
use crate::services::db::group_service::get_all_groups;

#[derive(Template)]
#[template(path = "groups.html")]
struct GroupsTemplate {
    groups: Vec<groups::Model>,
}

pub async fn list_groups(State(state): State<AppState>) -> impl IntoResponse {
    match get_all_groups(&state.db).await {
        Ok(groups) => {
            let template = GroupsTemplate { groups };

            match template.render() {
                Ok(html) => Html(html).into_response(),
                Err(error) => (
                    StatusCode::INTERNAL_SERVER_ERROR,
                    format!("Napaka pri renderiranju strani: {error}"),
                )
                    .into_response(),
            }
        }
        Err(error) => (
            StatusCode::INTERNAL_SERVER_ERROR,
            format!("Napaka pri branju skupin: {error}"),
        )
            .into_response(),
    }
}

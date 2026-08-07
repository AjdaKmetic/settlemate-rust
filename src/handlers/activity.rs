use crate::handlers::current_user::CurrentUser;
use askama::Template;
use axum::response::{Html, IntoResponse};

#[derive(Template)]
#[template(path = "activity.html")]
pub struct ActivityTemplate;

pub async fn activity(_current_user: CurrentUser) -> impl IntoResponse {
    Html(ActivityTemplate.render().unwrap())
}

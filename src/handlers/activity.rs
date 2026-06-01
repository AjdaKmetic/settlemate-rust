use askama::Template;
use axum::response::{Html, IntoResponse};

#[derive(Template)]
#[template(path = "activity.html")]
pub struct ActivityTemplate;

pub async fn activity() -> impl IntoResponse {
    Html(ActivityTemplate.render().unwrap())
}

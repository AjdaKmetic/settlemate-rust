use askama::Template;
use axum::response::{Html, IntoResponse};

#[derive(Template)]
#[template(path = "index.html")]
pub struct IndexTemplate;

pub async fn index() -> impl IntoResponse {
    Html(IndexTemplate.render().unwrap())
}
use askama::Template;
use axum::response::{Html, IntoResponse};

#[derive(Template)]
#[template(path = "index.html")]
pub struct IndexTemplate {
    pub username: String,
}

pub async fn index() -> impl IntoResponse {
    let template = IndexTemplate {
        username: "Ajda".to_string(),
    };
    Html(template.render().unwrap())
}
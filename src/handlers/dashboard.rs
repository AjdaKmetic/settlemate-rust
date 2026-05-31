use askama::Template;
use axum::response::{Html, IntoResponse};

#[derive(Template)]
#[template(path = "dashboard.html")]
pub struct DashboardTemplate;

pub async fn dashboard() -> impl IntoResponse {
    Html(DashboardTemplate.render().unwrap())
}

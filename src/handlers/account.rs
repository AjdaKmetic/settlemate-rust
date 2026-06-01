use axum::response::Html;

pub async fn account() -> Html<&'static str> {
    Html("<h1>Račun</h1>")
}

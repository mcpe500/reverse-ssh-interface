use axum::response::Html;

pub async fn index() -> Html<&'static str> {
    Html("<h1>Reverse SSH Interface</h1><p>Frontend not yet built. Use API endpoints.</p>")
}

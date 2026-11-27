use axum::response::Html;

const PAGE: &str = include_str!("../../templates/index.html");

pub fn index() -> Html<&'static str> {
    Html(PAGE)
}

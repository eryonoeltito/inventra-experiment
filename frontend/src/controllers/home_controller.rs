use axum::response::Html;

use crate::views::home_view;

pub async fn index() -> Html<&'static str> {
    home_view::index()
}

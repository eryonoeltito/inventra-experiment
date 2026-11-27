use axum::{http::header, response::IntoResponse};

const STYLESHEET: &str = include_str!("../../static/css/app.css");

pub fn sajikan_stylesheet() -> impl IntoResponse {
    (
        [(header::CONTENT_TYPE, "text/css; charset=utf-8")],
        STYLESHEET,
    )
}

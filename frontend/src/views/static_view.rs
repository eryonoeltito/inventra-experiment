use axum::{http::header, response::IntoResponse};

const STYLESHEET: &str = include_str!("../../static/css/app.css");
const GAMBAR_HOMEPAGE: &[u8] = include_bytes!("../../static/image/gambar_halaman_awal.jpeg");

pub fn sajikan_stylesheet() -> impl IntoResponse {
    (
        [(header::CONTENT_TYPE, "text/css; charset=utf-8")],
        STYLESHEET,
    )
}

pub fn sajikan_gambar_halaman_awal() -> impl IntoResponse {
    ([(header::CONTENT_TYPE, "image/jpeg")], GAMBAR_HOMEPAGE)
}

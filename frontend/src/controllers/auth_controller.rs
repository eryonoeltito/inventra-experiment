use axum::{http::StatusCode, response::IntoResponse};

use crate::views::auth_view;

fn respons_halaman(
    hasil: Result<axum::response::Html<String>, askama::Error>,
) -> axum::response::Response {
    match hasil {
        Ok(halaman) => halaman.into_response(),
        Err(kesalahan) => {
            eprintln!("Gagal merender halaman autentikasi: {kesalahan}");
            (
                StatusCode::INTERNAL_SERVER_ERROR,
                "Halaman tidak dapat ditampilkan saat ini.",
            )
                .into_response()
        }
    }
}

pub async fn tampilkan_login() -> impl IntoResponse {
    respons_halaman(auth_view::buat_halaman_login())
}

pub async fn tampilkan_register() -> impl IntoResponse {
    respons_halaman(auth_view::buat_halaman_register())
}

pub async fn tampilkan_pending_approval() -> impl IntoResponse {
    respons_halaman(auth_view::buat_halaman_pending_approval())
}

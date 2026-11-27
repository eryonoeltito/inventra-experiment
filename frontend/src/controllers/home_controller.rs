use axum::{http::StatusCode, response::IntoResponse};

use crate::views::home_view;

pub async fn index() -> impl IntoResponse {
    match home_view::index() {
        Ok(halaman) => halaman.into_response(),
        Err(kesalahan) => {
            eprintln!("Gagal merender halaman awal: {kesalahan}");
            (
                StatusCode::INTERNAL_SERVER_ERROR,
                "Halaman tidak dapat ditampilkan saat ini.",
            )
                .into_response()
        }
    }
}

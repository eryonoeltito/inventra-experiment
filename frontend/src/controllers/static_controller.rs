use axum::response::IntoResponse;

use crate::views::static_view;

pub async fn kirim_stylesheet() -> impl IntoResponse {
    static_view::sajikan_stylesheet()
}

pub async fn kirim_gambar_halaman_awal() -> impl IntoResponse {
    static_view::sajikan_gambar_halaman_awal()
}

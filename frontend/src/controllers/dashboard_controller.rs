use axum::{http::StatusCode, response::IntoResponse};

use crate::{models::dashboard::DashboardData, views::dashboard_view};

pub async fn tampilkan_dashboard() -> impl IntoResponse {
    match dashboard_view::buat_halaman(DashboardData::data_contoh()) {
        Ok(halaman) => halaman.into_response(),
        Err(kesalahan) => {
            eprintln!("Gagal merender halaman dashboard: {kesalahan}");
            (
                StatusCode::INTERNAL_SERVER_ERROR,
                "Halaman tidak dapat ditampilkan saat ini.",
            )
                .into_response()
        }
    }
}

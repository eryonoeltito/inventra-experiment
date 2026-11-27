use axum::{Json, extract::State, http::StatusCode, response::Response};
use sqlx::PgPool;

use crate::{models::gudang::Gudang, views::error_view::error_response};

pub async fn ambil_daftar_gudang(
    State(pool): State<PgPool>,
) -> Result<Json<Vec<Gudang>>, Response> {
    let gudang = Gudang::ambil_daftar(&pool).await.map_err(|error| {
        eprintln!("Gagal mengambil daftar gudang: {error}");
        error_response(
            StatusCode::INTERNAL_SERVER_ERROR,
            "Gagal mengambil daftar gudang",
        )
    })?;

    Ok(Json(gudang))
}

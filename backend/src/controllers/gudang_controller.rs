use axum::{
    Json,
    extract::{Path, State, rejection::PathRejection},
    http::StatusCode,
    response::{IntoResponse, Response},
};
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

pub async fn ambil_detail_gudang(
    State(pool): State<PgPool>,
    path: Result<Path<i64>, PathRejection>,
) -> Result<Json<Gudang>, Response> {
    let Path(id) = path.map_err(|rejection| {
        error_response(
            rejection.into_response().status(),
            "Parameter path tidak valid",
        )
    })?;

    let gudang = Gudang::ambil_berdasarkan_id(&pool, id)
        .await
        .map_err(|error| {
            eprintln!("Gagal mengambil detail gudang: {error}");
            error_response(
                StatusCode::INTERNAL_SERVER_ERROR,
                "Gagal mengambil detail gudang",
            )
        })?;

    match gudang {
        Some(gudang) => Ok(Json(gudang)),
        None => Err(error_response(
            StatusCode::NOT_FOUND,
            format!("Gudang dengan ID {id} tidak ditemukan"),
        )),
    }
}

use axum::{
    Json,
    extract::{
        Path, State,
        rejection::{JsonRejection, PathRejection},
    },
    http::StatusCode,
    response::{IntoResponse, Response},
};
use serde::Serialize;
use sqlx::PgPool;

use crate::{
    models::gudang::{Gudang, GudangBaru, GudangUpdate},
    views::error_view::error_response,
};

#[derive(Serialize)]
pub struct PesanGudangResponse {
    message: String,
}

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

pub async fn buat_gudang(
    State(pool): State<PgPool>,
    body: Result<Json<GudangBaru>, JsonRejection>,
) -> Result<(StatusCode, Json<Gudang>), Response> {
    let Json(data) = body.map_err(|rejection| {
        error_response(rejection.into_response().status(), "Body JSON tidak valid")
    })?;
    if let Err(errors) = data.validasi() {
        return Err(error_response(StatusCode::BAD_REQUEST, errors.join("; ")));
    }

    let gudang = Gudang::buat(&pool, data)
        .await
        .map_err(|error| respons_error_database(error, "Gagal menambahkan gudang"))?;

    Ok((StatusCode::CREATED, Json(gudang)))
}

pub async fn ubah_gudang(
    State(pool): State<PgPool>,
    path: Result<Path<i64>, PathRejection>,
    body: Result<Json<GudangBaru>, JsonRejection>,
) -> Result<Json<Gudang>, Response> {
    let Path(id) = path.map_err(|rejection| {
        error_response(
            rejection.into_response().status(),
            "Parameter path tidak valid",
        )
    })?;
    let Json(data) = body.map_err(|rejection| {
        error_response(rejection.into_response().status(), "Body JSON tidak valid")
    })?;
    if let Err(errors) = data.validasi() {
        return Err(error_response(StatusCode::BAD_REQUEST, errors.join("; ")));
    }

    let gudang = Gudang::ubah(&pool, id, data)
        .await
        .map_err(|error| respons_error_database(error, "Gagal mengubah gudang"))?;

    match gudang {
        Some(gudang) => Ok(Json(gudang)),
        None => Err(error_response(
            StatusCode::NOT_FOUND,
            format!("Gudang dengan ID {id} tidak ditemukan"),
        )),
    }
}

pub async fn patch_gudang(
    State(pool): State<PgPool>,
    path: Result<Path<i64>, PathRejection>,
    body: Result<Json<GudangUpdate>, JsonRejection>,
) -> Result<Json<Gudang>, Response> {
    let Path(id) = path.map_err(|rejection| {
        error_response(
            rejection.into_response().status(),
            "Parameter path tidak valid",
        )
    })?;
    let Json(data) = body.map_err(|rejection| {
        error_response(rejection.into_response().status(), "Body JSON tidak valid")
    })?;
    if let Err(errors) = data.validasi() {
        return Err(error_response(StatusCode::BAD_REQUEST, errors.join("; ")));
    }

    let gudang = Gudang::patch(&pool, id, data)
        .await
        .map_err(|error| respons_error_database(error, "Gagal memperbarui gudang"))?;

    match gudang {
        Some(gudang) => Ok(Json(gudang)),
        None => Err(error_response(
            StatusCode::NOT_FOUND,
            format!("Gudang dengan ID {id} tidak ditemukan"),
        )),
    }
}

pub async fn hapus_gudang(
    State(pool): State<PgPool>,
    path: Result<Path<i64>, PathRejection>,
) -> Result<(StatusCode, Json<PesanGudangResponse>), Response> {
    let Path(id) = path.map_err(|rejection| {
        error_response(
            rejection.into_response().status(),
            "Parameter path tidak valid",
        )
    })?;

    let terhapus = Gudang::hapus(&pool, id)
        .await
        .map_err(|error| respons_error_database(error, "Gagal menghapus gudang"))?;

    if terhapus {
        Ok((
            StatusCode::OK,
            Json(PesanGudangResponse {
                message: "Berhasil menghapus gudang".to_string(),
            }),
        ))
    } else {
        Err(error_response(
            StatusCode::NOT_FOUND,
            format!("Gudang dengan ID {id} tidak ditemukan"),
        ))
    }
}

fn respons_error_database(error: sqlx::Error, operasi: &str) -> Response {
    let kode = error
        .as_database_error()
        .and_then(|database_error| database_error.code());
    eprintln!("{operasi}: {error}");

    match kode.as_deref() {
        Some("23505") => error_response(StatusCode::CONFLICT, "Nama gudang sudah digunakan"),
        Some("23503") => error_response(
            StatusCode::CONFLICT,
            "Gudang masih digunakan oleh barang dan tidak dapat dihapus",
        ),
        _ => error_response(
            StatusCode::INTERNAL_SERVER_ERROR,
            "Terjadi kesalahan pada server",
        ),
    }
}

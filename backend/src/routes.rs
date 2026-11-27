use axum::{
    Json, Router,
    extract::Extension,
    http::StatusCode,
    routing::{get, post},
};
use serde_json::json;
use sqlx::PgPool;

use crate::{
    controllers::{auth_controller, barang_controller, gudang_controller, riwayat_stok_controller},
    middleware::auth_middleware::wajib_login,
    views::error_view::error_response,
};

pub fn router(pool: PgPool, jwt_secret: std::sync::Arc<str>) -> Router {
    let auth_routes = Router::new()
        .route("/api/auth/register", post(auth_controller::register))
        .route("/api/auth/login", post(auth_controller::login))
        .route("/api/auth/logout", post(auth_controller::logout));

    let protected_routes = Router::new()
        .route("/api/gudang", get(gudang_controller::ambil_daftar_gudang))
        .route(
            "/api/barang",
            get(barang_controller::list_barang).post(barang_controller::buat_barang),
        )
        .route(
            "/api/barang/{id}",
            axum::routing::put(barang_controller::ubah_barang)
                .delete(barang_controller::hapus_barang)
                .patch(barang_controller::patch_barang)
                .get(barang_controller::detail_barang),
        )
        .route(
            "/api/riwayat-stok",
            get(riwayat_stok_controller::list_riwayat_stok),
        )
        .route(
            "/api/barang/{id}/riwayat-stok",
            get(riwayat_stok_controller::list_riwayat_barang),
        )
        .route_layer(axum::middleware::from_fn(wajib_login));

    Router::new()
        .route("/health", get(health_check))
        .merge(auth_routes)
        .merge(protected_routes)
        .fallback(|| async { error_response(StatusCode::NOT_FOUND, "Route tidak ditemukan") })
        .method_not_allowed_fallback(|| async {
            error_response(StatusCode::METHOD_NOT_ALLOWED, "Method tidak diizinkan")
        })
        .layer(Extension(pool.clone()))
        .with_state(pool)
        .layer(Extension(jwt_secret))
}

async fn health_check() -> Json<serde_json::Value> {
    Json(json!({
        "status": "success",
        "message": "API inventaris berhasil berjalan"
    }))
}

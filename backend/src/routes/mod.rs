pub mod auth_routes;
pub mod barang_routes;
pub mod gudang_routes;
pub mod riwayat_stok_routes;

use axum::{Json, Router, extract::Extension, http::StatusCode, routing::get};
use serde_json::json;
use sqlx::PgPool;

use crate::views::error_view::error_response;

pub fn router(pool: PgPool, jwt_secret: std::sync::Arc<str>) -> Router {
    Router::new()
        .route("/health", get(health_check))
        .merge(auth_routes::routes())
        .merge(barang_routes::routes())
        .merge(gudang_routes::routes())
        .merge(riwayat_stok_routes::routes())
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

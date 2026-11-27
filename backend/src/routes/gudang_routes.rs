use axum::{Router, middleware, routing::get};
use sqlx::PgPool;

use crate::{controllers::gudang_controller, middleware::auth_middleware::wajib_login};

pub fn routes() -> Router<PgPool> {
    Router::new()
        .route(
            "/api/gudang",
            get(gudang_controller::ambil_daftar_gudang).post(gudang_controller::buat_gudang),
        )
        .route(
            "/api/gudang/{id}",
            get(gudang_controller::ambil_detail_gudang)
                .put(gudang_controller::ubah_gudang)
                .patch(gudang_controller::patch_gudang)
                .delete(gudang_controller::hapus_gudang),
        )
        .route_layer(middleware::from_fn(wajib_login))
}

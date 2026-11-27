use axum::{Router, middleware, routing::get};
use sqlx::PgPool;

use crate::{controllers::barang_controller, middleware::auth_middleware::wajib_login};

pub fn routes() -> Router<PgPool> {
    Router::new()
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
        .route_layer(middleware::from_fn(wajib_login))
}

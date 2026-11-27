use axum::{Router, routing::get};

use crate::controllers::{
    auth_controller, dashboard_controller, home_controller, static_controller,
};

pub fn router() -> Router {
    Router::new()
        .route("/", get(home_controller::index))
        .route("/dashboard", get(dashboard_controller::tampilkan_dashboard))
        .route("/login", get(auth_controller::tampilkan_login))
        .route("/register", get(auth_controller::tampilkan_register))
        .route(
            "/pending-approval",
            get(auth_controller::tampilkan_pending_approval),
        )
        .route(
            "/static/css/app.css",
            get(static_controller::kirim_stylesheet),
        )
        .route(
            "/static/image/gambar_halaman_awal.jpeg",
            get(static_controller::kirim_gambar_halaman_awal),
        )
}

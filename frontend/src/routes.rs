use axum::{Router, routing::get};

use crate::controllers::{dashboard_controller, home_controller, static_controller};

pub fn router() -> Router {
    Router::new()
        .route("/", get(home_controller::index))
        .route("/dashboard", get(dashboard_controller::tampilkan_dashboard))
        .route(
            "/static/css/app.css",
            get(static_controller::kirim_stylesheet),
        )
}

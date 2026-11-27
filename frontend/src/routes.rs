use axum::{Router, routing::get};

use crate::controllers::home_controller;

pub fn router() -> Router {
    Router::new().route("/", get(home_controller::index))
}

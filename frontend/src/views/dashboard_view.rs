use askama::Template;
use axum::response::Html;

use crate::models::dashboard::DashboardData;

#[derive(Template)]
#[template(path = "dashboard/index.html")]
struct DashboardTemplate {
    data: DashboardData,
}

pub fn buat_halaman(data: DashboardData) -> Result<Html<String>, askama::Error> {
    DashboardTemplate { data }.render().map(Html)
}

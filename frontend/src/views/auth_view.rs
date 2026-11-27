use askama::Template;
use axum::response::Html;

#[derive(Template)]
#[template(path = "auth/login.html")]
struct LoginTemplate;

#[derive(Template)]
#[template(path = "auth/register.html")]
struct RegisterTemplate;

#[derive(Template)]
#[template(path = "auth/pending_approval.html")]
struct PendingApprovalTemplate;

pub fn buat_halaman_login() -> Result<Html<String>, askama::Error> {
    LoginTemplate.render().map(Html)
}

pub fn buat_halaman_register() -> Result<Html<String>, askama::Error> {
    RegisterTemplate.render().map(Html)
}

pub fn buat_halaman_pending_approval() -> Result<Html<String>, askama::Error> {
    PendingApprovalTemplate.render().map(Html)
}

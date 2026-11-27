use askama::Template;
use axum::response::Html;

#[derive(Template)]
#[template(path = "index.html")]
struct HomeTemplate;

pub fn index() -> Result<Html<String>, askama::Error> {
    HomeTemplate.render().map(Html)
}

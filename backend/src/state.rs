use std::sync::Arc;

use sqlx::{PgPool, postgres::PgPoolOptions};

pub struct AppState {
    pub pool: PgPool,
    pub jwt_secret: Arc<str>,
    pub app_host: String,
    pub app_port: u16,
}

impl AppState {
    pub async fn from_env() -> Self {
        dotenvy::dotenv().ok();

        let database_url =
            std::env::var("DATABASE_URL").expect("DATABASE_URL tidak ditemukan di file .env");
        let pool = PgPoolOptions::new()
            .max_connections(5)
            .connect(&database_url)
            .await
            .expect("Gagal terhubung ke PostgreSQL");
        println!("Berhasil terhubung ke PostgreSQL!");

        let jwt_secret =
            std::env::var("JWT_SECRET").expect("JWT_SECRET tidak ditemukan di file .env");
        if jwt_secret.len() < 32 {
            panic!("JWT_SECRET harus memiliki minimal 32 byte untuk HS256");
        }

        let app_host = std::env::var("APP_HOST").expect("APP_HOST tidak ditemukan di file .env");
        let app_port = std::env::var("APP_PORT")
            .expect("APP_PORT tidak ditemukan di file .env")
            .parse::<u16>()
            .expect("APP_PORT harus berupa angka antara 0 hingga 65535");

        Self {
            pool,
            jwt_secret: Arc::from(jwt_secret),
            app_host,
            app_port,
        }
    }
}

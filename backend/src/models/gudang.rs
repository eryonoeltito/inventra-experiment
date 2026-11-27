use serde::Serialize;
use sqlx::{FromRow, PgPool};

#[derive(Debug, Serialize, FromRow)]
pub struct Gudang {
    pub id: i64,
    pub nama_gudang: String,
}

impl Gudang {
    pub async fn ambil_daftar(pool: &PgPool) -> Result<Vec<Self>, sqlx::Error> {
        sqlx::query_as::<_, Self>(
            "SELECT id, nama_gudang
             FROM public.gudang
             ORDER BY id",
        )
        .fetch_all(pool)
        .await
    }

    pub async fn ambil_berdasarkan_id(pool: &PgPool, id: i64) -> Result<Option<Self>, sqlx::Error> {
        sqlx::query_as::<_, Self>(
            "SELECT id, nama_gudang
             FROM public.gudang
             WHERE id = $1",
        )
        .bind(id)
        .fetch_optional(pool)
        .await
    }
}

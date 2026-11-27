use serde::{Deserialize, Serialize};
use sqlx::{FromRow, PgPool};

#[derive(Debug, Serialize, FromRow)]
pub struct Gudang {
    pub id: i64,
    pub nama_gudang: String,
}

#[derive(Debug, Deserialize)]
pub struct GudangBaru {
    pub nama_gudang: String,
}

#[derive(Debug, Deserialize)]
pub struct GudangUpdate {
    pub nama_gudang: Option<String>,
}

fn validasi_nama_gudang(nama_gudang: Option<&str>) -> Vec<String> {
    let mut errors = Vec::new();

    if let Some(nama_gudang) = nama_gudang {
        if nama_gudang.trim().is_empty() {
            errors.push("Nama gudang tidak boleh kosong".to_string());
        }
        if nama_gudang.chars().count() > 100 {
            errors.push("Nama gudang maksimal 100 karakter".to_string());
        }
    }

    errors
}

impl GudangBaru {
    pub fn validasi(&self) -> Result<(), Vec<String>> {
        let errors = validasi_nama_gudang(Some(&self.nama_gudang));

        if errors.is_empty() {
            Ok(())
        } else {
            Err(errors)
        }
    }
}

impl GudangUpdate {
    pub fn validasi(&self) -> Result<(), Vec<String>> {
        let mut errors = validasi_nama_gudang(self.nama_gudang.as_deref());

        if self.nama_gudang.is_none() {
            errors.push("Nama gudang harus diisi untuk diperbarui".to_string());
        }

        if errors.is_empty() {
            Ok(())
        } else {
            Err(errors)
        }
    }
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

    pub async fn buat(pool: &PgPool, data: GudangBaru) -> Result<Self, sqlx::Error> {
        sqlx::query_as::<_, Self>(
            "INSERT INTO public.gudang (nama_gudang)
             VALUES ($1)
             RETURNING id, nama_gudang",
        )
        .bind(data.nama_gudang)
        .fetch_one(pool)
        .await
    }

    pub async fn ubah(
        pool: &PgPool,
        id: i64,
        data: GudangBaru,
    ) -> Result<Option<Self>, sqlx::Error> {
        sqlx::query_as::<_, Self>(
            "UPDATE public.gudang
             SET nama_gudang = $1
             WHERE id = $2
             RETURNING id, nama_gudang",
        )
        .bind(data.nama_gudang)
        .bind(id)
        .fetch_optional(pool)
        .await
    }

    pub async fn patch(
        pool: &PgPool,
        id: i64,
        data: GudangUpdate,
    ) -> Result<Option<Self>, sqlx::Error> {
        sqlx::query_as::<_, Self>(
            "UPDATE public.gudang
             SET nama_gudang = COALESCE($1, nama_gudang)
             WHERE id = $2
             RETURNING id, nama_gudang",
        )
        .bind(data.nama_gudang)
        .bind(id)
        .fetch_optional(pool)
        .await
    }

    pub async fn hapus(pool: &PgPool, id: i64) -> Result<bool, sqlx::Error> {
        let hasil = sqlx::query("DELETE FROM public.gudang WHERE id = $1 RETURNING id")
            .bind(id)
            .fetch_optional(pool)
            .await?;

        Ok(hasil.is_some())
    }
}

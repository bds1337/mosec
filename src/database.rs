use sqlx::sqlite::{SqlitePool, SqlitePoolOptions};

use crate::model::{Audit};

pub async fn init_pool(database_url: &str) -> Result<SqlitePool, sqlx::Error> {
	SqlitePoolOptions::new()
		.acquire_timeout(std::time::Duration::from_secs(1))
		.connect(database_url)
		.await
}

pub async fn get_all_audits(pool: &SqlitePool) -> Result<Vec<Audit>, &'static str> {
// pub async fn get_all_audits(pool: &SqlitePool) -> Result<(), &'static str> {
	Audit::all(pool).await.map_err(|_| "error retrieving audits")
	// Audit::get_subject_from(pool).await.map_err(|_| "error")
}
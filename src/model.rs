use serde::{Deserialize, Serialize};
use sqlx::{SqlitePool, types::chrono};

#[derive(Debug, Serialize, Deserialize)]
pub struct Audit {
    pub id: String,
    pub subject_from: Option<String>,
    pub subject_to: Option<String>,
    pub time: chrono::NaiveDateTime,
    pub msg: Option<String>,
    pub encrypted: Option<bool>,
}

impl Audit {
	// pub async fn all(connection: &SqlitePool) -> Result<Vec<Audit>, sqlx::Error> {
	pub async fn all(connection: &SqlitePool) -> Result<Vec<Audit>, sqlx::Error> {
		let audits = sqlx::query_as!(Audit, r#"SELECT * FROM audits"#)
			.fetch_all(connection)
			.await?;

		print!("{:?}", audits);

		Ok(audits)
		// Ok(())
	}

	pub async fn get_subject_from(connection: &SqlitePool) -> Result<(), sqlx::Error> {
        let test = sqlx::query!(
  			r#"
            SELECT subject_from FROM audits WHERE id = 1
            "#)
        .fetch_one(connection)
        .await?;

        print!("{:?}", test);

        Ok(())
    }
}
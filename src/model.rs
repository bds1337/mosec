use serde::{Deserialize, Serialize};
use sqlx::{SqlitePool, types::chrono};

// FIXME: Use Transaction instead of Audit
#[derive(Debug, Serialize, Deserialize)]
pub struct Audit {
    pub id: String,
    pub subject_from: Option<String>,
    pub subject_to: Option<String>,
    pub time: chrono::NaiveDateTime,
    pub msg: Option<String>,
    pub encrypted: Option<bool>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Transaction {
    pub id: String,
    pub node_source: Option<String>,
    pub node_source_addr: Option<String>,
    pub node_dest: Option<String>,
    pub node_dest_addr: Option<String>,
    pub protocol: Option<String>,
    pub category: Option<String>,
    pub description: Option<String>,
    pub time: chrono::NaiveDateTime,
    pub status: Option<bool>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Node {
    pub id: String,
    pub name: Option<String>,
    pub addr: Option<String>,
    pub protocol: Option<String>,
    pub description: Option<String>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Category {
    pub id: String,
    pub name: String,
    pub level: String,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct User {
    pub id: String,
    pub name: Option<String>,
}

impl Audit {
	// pub async fn all(connection: &SqlitePool) -> Result<Vec<Audit>, sqlx::Error> {
	pub async fn all(connection: &SqlitePool) -> Result<Vec<Audit>, sqlx::Error> {
		let audits = sqlx::query_as!(Audit, r#"SELECT * FROM audits"#)
			.fetch_all(connection)
			.await?;
		Ok(audits)
	}

	pub async fn get_subject_from(connection: &SqlitePool) -> Result<(), sqlx::Error> {
        let test = sqlx::query!(
  			r#"
            SELECT subject_from FROM audits WHERE id = 1
            "#)
        .fetch_one(connection)
        .await?;
        Ok(())
    }
}

use sqlx::{Pool, Postgres};

use crate::{dto::log::LogEntryDto, model::log::NewLogEntryModel, RetError};

#[derive(Debug, Clone)]
pub struct PgLogRepository {
    pool: Pool<Postgres>,
}

impl PgLogRepository {
    pub fn new(pool: Pool<Postgres>) -> Self {
        Self { pool }
    }

    pub async fn add_log_record(&self, rec: &NewLogEntryModel) -> Result<(), Box<RetError>> {
        sqlx::query!(
            r#"
INSERT INTO log_entry ( timestamp, username, host_id, store_path, activation_type )
VALUES ( $1, $2, $3, $4, $5 )
        "#,
            rec.timestamp,
            rec.username,
            rec.host_id,
            rec.store_path,
            rec.activation_type,
        )
        .execute(&self.pool)
        .await?;

        Ok(())
    }
}

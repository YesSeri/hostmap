use chrono::DateTime;
use sqlx::{Pool, Postgres};

use crate::{
    dto::log::LogEntryDto,
    model::{
        host::ExistingHostModel,
        log::{ExistingLogEntryModel, HostId, LogEntryWithRevision, NewLogEntryModel},
    },
    RetError,
};

#[derive(Debug, Clone)]
pub struct ActivationLogRepository {
    pool: Pool<Postgres>,
}

impl ActivationLogRepository {
    pub fn new(pool: Pool<Postgres>) -> Self {
        Self { pool }
    }

    pub async fn insert_activation_log_record(
        &self,
        rec: &NewLogEntryModel,
    ) -> Result<(), Box<RetError>> {
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

    pub async fn latest_entry_for_host(
        &self,
        HostId(h_id): HostId,
    ) -> Result<Option<ExistingLogEntryModel>, Box<RetError>> {
        let log_entry_with_rev = sqlx::query_as!(
            LogEntryWithRevision,
            r#"
        select log_entry_id, timestamp, username, host_id, store_path, activation_type, (SELECT NULL) as rev_id, (SELECT NULL) as branch
        from log_entry
        where host_id = $1
        order by timestamp desc
        limit 1
        "#,
            h_id
        )
        .fetch_optional(&self.pool)
        .await?;
        let log_entry = log_entry_with_rev.map(|el| el.into());
        Ok(log_entry)
    }

    pub async fn entries_for_host(
        &self,
        HostId(h_id): HostId,
    ) -> Result<Vec<ExistingLogEntryModel>, sqlx::Error> {
        let log_entry_vec = sqlx::query_as!(
            LogEntryWithRevision,
            r#"
        select log_entry_id, timestamp, username, host_id, store_path, activation_type, (SELECT NULL) as rev_id, (SELECT NULL) as branch
        from log_entry
        where host_id = $1
        order by timestamp desc
        "#,
            h_id
        )
        .fetch_all(&self.pool)
        .await?;
        let log_entries = log_entry_vec.into_iter().map(|el| el.into()).collect();
        Ok(log_entries)
    }
    pub async fn host_with_logs_by_name(
        &self,
        name: &str,
    ) -> sqlx::Result<(ExistingHostModel, Vec<LogEntryWithRevision>)> {
        let host = sqlx::query_as!(
            ExistingHostModel,
            r#"select host_id, name, url from host where name = $1"#,
            name
        )
        .fetch_one(&self.pool)
        .await?;

        let log_with_revision = sqlx::query_as!(
            LogEntryWithRevision,
            r#"
        select log_entry_id, timestamp, username, host_id, store_path, activation_type, (SELECT NULL) as rev_id, (SELECT NULL) as branch
        from log_entry
        where host_id = $1
        order by timestamp desc
        "#,
            host.host_id
        )
        .fetch_all(&self.pool)
        .await?;
        let log_models = log_with_revision.into_iter().map(|el| el.into()).collect();

        Ok((host, log_models))
    }
}

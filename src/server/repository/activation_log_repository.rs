use crate::shared::model::{
    host::HostModel,
    log::{CreateLogEntryModel, ExistingLogEntryModel, LogEntryWithRevision},
};
use sqlx::{Pool, Postgres, QueryBuilder};

use crate::server::custom_error::RetError;

#[derive(Debug, Clone)]
pub struct ActivationLogRepository {
    pool: Pool<Postgres>,
}

impl ActivationLogRepository {
    pub fn new(pool: Pool<Postgres>) -> Self {
        Self { pool }
    }
    pub(crate) async fn bulk_insert_log_records(
        &self,
        _recs: &[CreateLogEntryModel],
    ) -> Result<u64, RetError> {
        const CHUNK_SIZE: usize = 1000;
        let mut i = 0;
        for chunk in _recs.chunks(CHUNK_SIZE) {
            let mut query_builder: QueryBuilder<Postgres> = QueryBuilder::new(
                "INSERT INTO log_entry(timestamp, hostname, username, store_path, activation_type) ",
            );
            query_builder.push_values(chunk.iter(), |mut b, rec| {
                b.push_bind(rec.timestamp)
                    .push_bind(&rec.hostname)
                    .push_bind(&rec.username)
                    .push_bind(&rec.store_path)
                    .push_bind(&rec.activation_type);
            });
            // on conflict do nothing to avoid duplicate entries
            query_builder
                .push(" ON CONFLICT (hostname, username, timestamp, store_path, activation_type) DO NOTHING");
            let query = query_builder.build();
            let res = query.execute(&self.pool).await?;
            i += res.rows_affected();
        }
        Ok(i)
    }

    pub async fn latest_entry_for_host(
        &self,
        host: HostModel,
    ) -> Result<Option<ExistingLogEntryModel>, RetError> {
        let log_entry_with_rev = sqlx::query_as!(
            LogEntryWithRevision,
            r#"
        SELECT log_entry_id, hostname, timestamp, username, store_path,
            activation_type, (SELECT NULL) as rev_id, (SELECT NULL) as branch
        FROM log_entry 
        WHERE 1 = 1 
            AND hostname = $1
        ORDER BY timestamp desc LIMIT 1
        "#,
            host.hostname,
        )
        .fetch_optional(&self.pool)
        .await?;
        let log_entry = log_entry_with_rev.map(|el| el.into());
        Ok(log_entry)
    }

    pub async fn get_logs_by_hostname(
        &self,
        hostname: &str,
    ) -> sqlx::Result<Vec<LogEntryWithRevision>> {
        let log_with_revision = sqlx::query_as!(
            LogEntryWithRevision,
            r#"
        select log_entry_id, timestamp, username, hostname,
            store_path, activation_type, (SELECT NULL) as rev_id, (SELECT NULL) as branch
        from log_entry where hostname = $1
        order by timestamp desc
        "#,
            hostname
        )
        .fetch_all(&self.pool)
        .await?;
        let log_models = log_with_revision.into_iter().collect();

        Ok(log_models)
    }
}

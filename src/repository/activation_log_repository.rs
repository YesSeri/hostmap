use sqlx::{Pool, Postgres, QueryBuilder};

use crate::{
    model::log::{ExistingLogEntryModel, HostId, LogEntryWithRevision, NewLogEntryModel},
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
    pub(crate) async fn bulk_insert_log_records(
        &self,
        _recs: &[NewLogEntryModel],
    ) -> Result<(), RetError> {
        const CHUNK_SIZE: usize = 1000;
        for chunk in _recs.chunks(CHUNK_SIZE) {
            let mut query_builder: QueryBuilder<Postgres> = QueryBuilder::new(
                "INSERT INTO log_entry( timestamp, username, host_id, store_path, activation_type ) ",
            );
            query_builder.push_values(chunk.iter(), |mut b, rec| {
                b.push_bind(rec.timestamp)
                    .push_bind(&rec.username)
                    .push_bind(rec.host_id)
                    .push_bind(&rec.store_path)
                    .push_bind(&rec.activation_type);
            });

            // on conflict do nothing to avoid duplicate entries
            query_builder.push(" ON CONFLICT (timestamp, username, store_path, activation_type, host_id) DO NOTHING");
            let query = query_builder.build();
            query.execute(&self.pool).await?;
        }
        Ok(())
    }

    pub async fn latest_entry_for_host(
        &self,
        host_id: HostId,
    ) -> Result<Option<ExistingLogEntryModel>, RetError> {
        let log_entry_with_rev = sqlx::query_as!(
            LogEntryWithRevision,
            r#"
        select log_entry_id, timestamp, username, host_id, store_path, activation_type, (SELECT NULL) as rev_id, (SELECT NULL) as branch
        from log_entry
        where host_id = $1
        order by timestamp desc
        limit 1
        "#,
            host_id
        )
        .fetch_optional(&self.pool)
        .await?;
        let log_entry = log_entry_with_rev.map(|el| el.into());
        Ok(log_entry)
    }

    // pub async fn entries_for_host(
    //     &self,
    //     host_id: HostId,
    // ) -> Result<Vec<ExistingLogEntryModel>, sqlx::Error> {
    //     let log_entry_vec = sqlx::query_as!(
    //         LogEntryWithRevision,
    //         r#"
    //     select log_entry_id, timestamp, username, host_id, store_path, activation_type, (SELECT NULL) as rev_id, (SELECT NULL) as branch
    //     from log_entry
    //     where host_id = $1
    //     order by timestamp desc
    //     "#,
    //         host_id
    //     )
    //     .fetch_all(&self.pool)
    //     .await?;
    //     let log_entries = log_entry_vec.into_iter().map(|el| el.into()).collect();
    //     Ok(log_entries)
    // }

    // pub async fn get_host_by_host_id(
    //     &self,
    //     host_id: i64,
    // ) -> sqlx::Result<Option<ExistingHostModel>> {
    //     let host = sqlx::query_as!(
    //         ExistingHostModel,
    //         r#"select host_id, name, url from host where host_id = $1"#,
    //         host_id
    //     )
    //     .fetch_one(&self.pool)
    //     .await?;
    //     Ok(Some(host))
    // }

    pub async fn get_logs_by_host_id(
        &self,
        host_id: HostId,
    ) -> sqlx::Result<Vec<LogEntryWithRevision>> {
        let log_with_revision = sqlx::query_as!(
            LogEntryWithRevision,
            r#"
        select log_entry_id, timestamp, username, host_id, store_path, activation_type, (SELECT NULL) as rev_id, (SELECT NULL) as branch
        from log_entry
        where host_id = $1
        order by timestamp desc
        "#,
        host_id
        )
        .fetch_all(&self.pool)
        .await?;
        let log_models = log_with_revision.into_iter().collect();

        Ok(log_models)
    }
}

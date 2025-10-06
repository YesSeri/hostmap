use crate::shared::model::{
    activation::{Activation, ActivationWithRevision, NewActivation},
    host::HostModel,
};
use sqlx::{Pool, Postgres, QueryBuilder};

use crate::server::custom_error::RetError;

#[derive(Debug, Clone)]
pub struct ActivationRepository {
    pool: Pool<Postgres>,
}

impl ActivationRepository {
    pub fn new(pool: Pool<Postgres>) -> Self {
        Self { pool }
    }

    pub(crate) async fn insert_many(
        transaction: &mut sqlx::Transaction<'_, sqlx::Postgres>,
        log_models: &[NewActivation],
    ) -> Result<u64, RetError> {
        const CHUNK_SIZE: usize = 1000;
        let mut i = 0;
        for chunk in log_models.chunks(CHUNK_SIZE) {
            let mut query_builder: QueryBuilder<Postgres> = QueryBuilder::new(
                "INSERT INTO activation(activated_at, hostname, username, store_path, activation_type) ",
            );
            query_builder.push_values(chunk.iter(), |mut b, rec| {
                b.push_bind(rec.core.activated_at)
                    .push_bind(&rec.core.hostname)
                    .push_bind(&rec.core.username)
                    .push_bind(&rec.core.store_path)
                    .push_bind(&rec.core.activation_type);
            });
            // on conflict do nothing to avoid duplicate entries
            query_builder
                .push(" ON CONFLICT (hostname, username, activated_at, store_path, activation_type) DO NOTHING");
            let query = query_builder.build();
            let res = query.execute(&mut **transaction).await?;
            i += res.rows_affected();
        }
        Ok(i)
    }

    pub async fn latest_entry_for_host(
        pool: &Pool<Postgres>,
        host: HostModel,
    ) -> Result<Option<Activation>, RetError> {
        let activation_with_revision = sqlx::query_as!(
            ActivationWithRevision,
            r#"
        SELECT activation_id, hostname, activated_at, username, store_path,
            activation_type, (SELECT NULL) as commit_hash, (SELECT NULL) as branch
        FROM activation 
        WHERE 1 = 1 
            AND hostname = $1
        ORDER BY activated_at desc LIMIT 1
        "#,
            host.hostname,
        )
        .fetch_optional(pool)
        .await?;
        let activation = activation_with_revision.map(|el| el.into());
        Ok(activation)
    }

    pub async fn get_logs_by_hostname(
        pool: &Pool<Postgres>,
        hostname: &str,
    ) -> sqlx::Result<Vec<ActivationWithRevision>> {
        let log_with_revision = sqlx::query_as!(
            ActivationWithRevision,
            r#"
        select activation_id, activated_at, username, hostname,
            store_path, activation_type, (SELECT NULL) as commit_hash, (SELECT NULL) as branch
        from activation where hostname = $1
        order by activated_at desc
        "#,
            hostname
        )
        .fetch_all(pool)
        .await?;
        let log_models = log_with_revision.into_iter().collect();

        Ok(log_models)
    }
}

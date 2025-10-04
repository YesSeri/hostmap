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

    async fn bulk_insert_store_paths(&self, _recs: &[&str]) -> Result<u64, RetError> {
        // start transaction
        const CHUNK_SIZE: usize = 1000;
        let mut i = 0;
        for chunk in _recs.chunks(CHUNK_SIZE) {
            let mut query_builder: QueryBuilder<Postgres> =
                QueryBuilder::new("INSERT INTO nix_store_path(store_path) ");
            query_builder.push_values(chunk.iter(), |mut b, rec| {
                b.push_bind(rec.to_string());
            });
            // on conflict do nothing to avoid duplicate entries
            query_builder.push(" ON CONFLICT (store_path) DO NOTHING");
            let query = query_builder.build();
            let res = query.execute(&self.pool).await?;
            i += res.rows_affected();
        }
        Ok(i)
    }
    async fn insert_many(&self, log_models: &[NewActivation]) -> Result<u64, RetError> {
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
            let res = query.execute(&self.pool).await?;
            i += res.rows_affected();
        }
        Ok(i)
    }
    pub(crate) async fn insert_many_activations_with_store_paths(
        &self,
        activation_model: &[NewActivation],
    ) -> Result<u64, RetError> {
        let mut tx = self.pool.begin().await?;

        let store_paths: Vec<&str> = activation_model
            .iter()
            .map(|el| el.core.store_path.as_str())
            .collect();
        self.bulk_insert_store_paths(&store_paths).await?;
        let inserted_count = self.insert_many(activation_model).await?;

        tx.commit().await?;
        Ok(inserted_count)
    }

    pub async fn latest_entry_for_host(
        &self,
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
        .fetch_optional(&self.pool)
        .await?;
        let activation = activation_with_revision.map(|el| el.into());
        Ok(activation)
    }

    pub async fn get_logs_by_hostname(
        &self,
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
        .fetch_all(&self.pool)
        .await?;
        let log_models = log_with_revision.into_iter().collect();

        Ok(log_models)
    }
}

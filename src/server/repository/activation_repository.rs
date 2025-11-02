use crate::shared::model::activation::{ActivationWithRevision, NewActivation};
use sqlx::{Pool, Postgres, QueryBuilder};

use crate::server::custom_error::RetError;

#[derive(Debug, Clone)]
pub struct ActivationRepository;

impl ActivationRepository {
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
    pub async fn get_logs_by_hostname(
        pool: &Pool<Postgres>,
        hostname: &str,
    ) -> sqlx::Result<Vec<ActivationWithRevision>> {
        let rows = sqlx::query_as!(
            ActivationWithRevision,
            r#"
            WITH best_nix_git_link AS (
              SELECT DISTINCT ON (n.store_path)
                     n.store_path,
                     n.commit_hash,
                     n.branch
              FROM nix_git_link n
              ORDER BY n.store_path, (n.branch = 'master') DESC, n.linked_at ASC NULLS LAST
            )
            SELECT a.activation_id, a.activated_at, a.username, a.hostname, a.store_path, a.activation_type, b.commit_hash AS "commit_hash?", b.branch AS "branch?"
            FROM activation a
            LEFT JOIN best_nix_git_link b
              ON b.store_path = a.store_path
            WHERE a.hostname = $1
            ORDER BY a.activated_at DESC
            "#,
            hostname
        )
        .fetch_all(pool)
        .await?;

        Ok(rows)
    }
}

use std::collections::HashMap;

use crate::shared::model::{
    activation::{Activation, ActivationWithRevision},
    host::{HostModel, HostWithLatestLog},
};
use sqlx::{Pool, Postgres, QueryBuilder};

use crate::server::custom_error::RetError;

#[derive(Debug, Clone)]
pub struct HostRepository {}

impl HostRepository {
    pub async fn bulk_insert_hosts(
        pool: &Pool<Postgres>,
        hosts: &[HostModel],
    ) -> Result<u64, sqlx::Error> {
        const CHUNK_SIZE: usize = 500; // rows (hosts) per INSERT
        tracing::info!("Inserting {} hosts", hosts.len());
        let tuple_vec: Vec<(&str, &str, HashMap<String, String>)> = hosts
            .iter()
            .map(|h| (h.hostname.as_str(), h.host_url.as_str(), h.metadata.clone()))
            .collect();
        let mut rows_inserted = 0;

        for chunk in tuple_vec.chunks(CHUNK_SIZE) {
            tracing::info!("Inserting chunk of {} hosts", chunk.len());
            let mut query_builder =
                QueryBuilder::new("INSERT INTO host(hostname, host_url, metadata) ");
            query_builder.push_values(chunk.iter(), |mut b, row| {
                b.push_bind(row.0)
                    .push_bind(row.1)
                    .push_bind(serde_json::to_value(&row.2).unwrap());
            });

            // on conflict do nothing to avoid duplicate entries
            query_builder.push(" ON CONFLICT (hostname) DO UPDATE SET host_url = EXCLUDED.host_url, metadata = EXCLUDED.metadata, updated_at = NOW(), created_at = host.created_at");
            let query = query_builder.build();
            let res = query.execute(pool).await?;
            rows_inserted += res.rows_affected();
        }
        Ok(rows_inserted)
    }

    pub async fn get_host_from_hostname(
        pool: &Pool<Postgres>,
        hostname: String,
    ) -> Result<Option<HostModel>, RetError> {
        dbg!("fetching host from db with name: {:?}", &hostname);
        let result = sqlx::query!(
            r#"
            SELECT hostname, host_url, metadata FROM host
                WHERE hostname = $1
            "#,
            hostname,
        )
        .map(|record| HostModel {
            hostname: record.hostname,
            host_url: record.host_url,
            // json to hashmap
            metadata: serde_json::from_value(record.metadata).unwrap_or(HashMap::from([(
                "error".to_string(),
                "nested json metadata is not allowed".to_string(),
            )])),
        })
        .fetch_optional(pool)
        .await?;
        dbg!(&result);

        Ok(result)
    }

    pub async fn get_all_hosts_with_latest_activation(
        pool: &Pool<Postgres>,
    ) -> Result<Vec<HostWithLatestLog>, RetError> {
        let logs = sqlx::query_as!(
            ActivationWithRevision,
            r#"

WITH latest AS (
SELECT 
DISTINCT ON (ac.hostname)
ac.activation_id, ac.activated_at, ac.username, ac.store_path, ac.activation_type, ac.hostname
FROM activation ac 
     ORDER BY ac.hostname, ac.activated_at DESC 
)
SELECT DISTINCT ON(l.hostname) l.activation_id, l.activated_at, l.username, 
    l.store_path, l.activation_type, l.hostname, ngl.commit_hash AS "commit_hash?", ngl.branch AS "branch?"
    FROM latest l
    LEFT JOIN nix_git_link ngl ON ngl.store_path = l.store_path
    ORDER BY l.hostname, ngl.branch = 'master' desc, ngl.linked_at asc
;
            "#,
        )
        .fetch_all(pool)
        .await?;
        let all_logs: Vec<Activation> = logs.into_iter().map(|el| el.into()).collect();
        let hosts = Self::get_all_hosts(pool).await?;

        let mut result = Vec::new();
        for host in hosts {
            let latest_log = all_logs
                .iter()
                .find(|log| log.core.hostname == host.hostname)
                .cloned();

            let host_with_latest_log = HostWithLatestLog {
                host: host.clone(),
                logs: latest_log,
            };
            result.push(host_with_latest_log);
        }
        Ok(result)
    }
    pub(crate) async fn get_all_hosts(pool: &Pool<Postgres>) -> Result<Vec<HostModel>, RetError> {
        let result = sqlx::query!(
            r#"
            SELECT hostname, host_url, metadata FROM host
            "#
        )
        .map(|record| HostModel {
            hostname: record.hostname,
            host_url: record.host_url,
            // json to hashmap
            metadata: serde_json::from_value(record.metadata).unwrap_or(HashMap::from([(
                "error".to_string(),
                "nested json metadata is not allowed".to_string(),
            )])),
        })
        .fetch_all(pool)
        .await?;
        Ok(result)
    }
}

use std::collections::HashMap;

use crate::shared::model::{
    host::{HostModel, HostWithLatestLog},
    log::{ExistingLogEntryModel, LogEntryWithRevision},
};
use sqlx::{Execute, Pool, Postgres, QueryBuilder};

use crate::server::custom_error::RetError;

#[derive(Debug, Clone)]
pub struct HostRepository {
    pool: Pool<Postgres>,
}

impl HostRepository {
    pub fn new(pool: Pool<Postgres>) -> Self {
        Self { pool }
    }

    pub async fn bulk_insert_hosts(&self, hosts: &[HostModel]) -> Result<u64, sqlx::Error> {
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
            let res = query.execute(&self.pool).await?;
            rows_inserted += res.rows_affected();
        }
        Ok(rows_inserted)
    }

    pub async fn get_host_from_hostname(
        &self,
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
        .fetch_optional(&self.pool)
        .await?;
        dbg!(&result);

        Ok(result)
    }

    // pub async fn latest_entry_for_host(
    //     &self,
    //     host: HostModel,
    // ) -> Result<Option<ExistingLogEntryModel>, RetError> {
    //     let log_entry_with_rev = sqlx::query_as!(
    //         LogEntryWithRevision,
    //         r#"
    //     SELECT log_entry_id, hostname, timestamp, username, store_path,
    //         activation_type, (SELECT NULL) as rev_id, (SELECT NULL) as branch
    //     FROM log_entry
    //     WHERE 1 = 1
    //         AND hostname = $1
    //     ORDER BY timestamp desc LIMIT 1
    //     "#,
    //         host.hostname,
    //     )
    //     .fetch_optional(&self.pool)
    //     .await?;
    //     let log_entry = log_entry_with_rev.map(|el| el.into());
    //     Ok(log_entry)
    // }
    pub async fn get_all_hosts_with_latest_log_entry(
        &self,
    ) -> Result<Vec<HostWithLatestLog>, RetError> {
        let logs = sqlx::query_as!(
            LogEntryWithRevision,
            r#"
            SELECT DISTINCT ON(hostname) log_entry_id, hostname, timestamp, username, store_path, activation_type,
                (SELECT NULL) as rev_id, (SELECT NULL) as branch
            FROM log_entry
            ORDER BY hostname, timestamp desc
            "#,
        ).fetch_all(&self.pool).await?;
        let all_logs: Vec<ExistingLogEntryModel> = logs.into_iter().map(|el| el.into()).collect();
        let hosts = self.get_all_hosts().await?;

        let mut result = Vec::new();
        for host in hosts {
            let latest_log = all_logs
                .iter()
                .find(|log| log.hostname == host.hostname)
                .cloned();

            let host_with_latest_log = HostWithLatestLog {
                host: host.clone(),
                logs: latest_log,
            };
            result.push(host_with_latest_log);
        }

        Ok(result)
    }
    pub async fn get_all_hosts(&self) -> Result<Vec<HostModel>, RetError> {
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
        .fetch_all(&self.pool)
        .await?;
        Ok(result)
    }
}

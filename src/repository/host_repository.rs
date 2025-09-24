use crate::shared::{
    dto::host::{self, CurrentHostDto},
    model::{host::HostModel, log::HostName},
};
use sqlx::{Execute, Pool, Postgres, QueryBuilder};

use crate::RetError;

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
        tracing::info!("Hosts: {hosts:?}");
        let tuple_vec: Vec<(&str, &str, &serde_json::Value)> = hosts
            .iter()
            .map(|h| (h.hostname.as_str(), h.host_url.as_str(), &h.metadata))
            .collect();
        let mut rows_inserted = 0;

        for chunk in tuple_vec.chunks(CHUNK_SIZE) {
            tracing::info!("Inserting chunk of {} hosts", chunk.len());
            tracing::info!("Chunk: {chunk:?}");
            let mut query_builder = QueryBuilder::new("INSERT INTO host(hostname, host_url) ");
            query_builder.push_values(chunk.iter(), |mut b, row| {
                b.push_bind(row.0).push_bind(row.1).push_bind(row.2);
            });

            // on conflict do nothing to avoid duplicate entries
            query_builder.push(" ON CONFLICT (hostname) DO NOTHING ");
            let query = query_builder.build();
            tracing::info!("Executing query: {:?}", query.sql());
            let res = query.execute(&self.pool).await?;
            tracing::info!("Affected rows: {:?}", res.rows_affected());
            rows_inserted += res.rows_affected();
        }
        Ok(rows_inserted)
    }

    pub async fn get_host_from_hostname(
        &self,
        host_name: String,
    ) -> Result<Option<HostModel>, RetError> {
        dbg!("fetching host from db with name: {:?}", &host_name);
        let result = sqlx::query_as!(
            HostModel,
            r#"
            SELECT hostname, host_url, metadata FROM host
                WHERE hostname = $1
            "#,
            host_name,
        )
        .fetch_optional(&self.pool)
        .await?;
        dbg!(&result);

        Ok(result)
    }
    pub async fn get_all_hosts(&self) -> Result<Vec<host::CurrentHostDto>, RetError> {
        let result = sqlx::query_as!(
            HostModel,
            r#"
            SELECT hostname, host_url, metadata FROM host
            "#
        )
        .fetch_all(&self.pool)
        .await?;
        let dto_vec = result.into_iter().map(CurrentHostDto::from).collect();
        Ok(dto_vec)
    }
}

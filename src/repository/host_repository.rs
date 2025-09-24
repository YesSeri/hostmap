use crate::shared::{
    dto::host,
    model::{host::HostModel, host_group::HostGroupModel, log::HostName},
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

    pub async fn bulk_insert_group_hosts(
        &self,
        groups: &[HostGroupModel],
    ) -> Result<u64, sqlx::Error> {
        const CHUNK_SIZE: usize = 1000;
        let mut rows_inserted = 0;
        for chunk in groups.chunks(CHUNK_SIZE) {
            let mut query_builder: QueryBuilder<Postgres> =
                QueryBuilder::new("INSERT INTO host_group(host_group_name) ");
            query_builder.push_values(chunk.iter(), |mut b, rec| {
                b.push_bind(&rec.host_group_name);
            });
            // on conflict do nothing to avoid duplicate entries
            query_builder.push(" ON CONFLICT (host_group_name) DO NOTHING");
            let query = query_builder.build();
            let res = query.execute(&self.pool).await?;
            rows_inserted += res.rows_affected();
        }
        Ok(rows_inserted)
    }

    pub async fn bulk_insert_hosts(
        &self,
        host_groups: &[HostGroupModel],
    ) -> Result<u64, sqlx::Error> {
        const CHUNK_SIZE: usize = 500; // rows (hosts) per INSERT
        tracing::info!("Inserting {} hosts", host_groups.len());
        tracing::info!("Host groups: {host_groups:?}");
        let tuple_vec: Vec<(&str, &str, &str)> = host_groups
            .iter()
            .flat_map(|g| {
                g.host_models.iter().map(move |h| {
                    (
                        g.host_group_name.as_str(),
                        h.host_name.as_str(),
                        h.host_url.as_str(),
                    )
                })
            })
            .collect();
        let mut rows_inserted = 0;
        // let mut rows_staged = 0usize;

        for chunk in tuple_vec.chunks(CHUNK_SIZE) {
            tracing::info!("Inserting chunk of {} hosts", chunk.len());
            tracing::info!("Chunk: {chunk:?}");
            let mut query_builder =
                QueryBuilder::new("INSERT INTO host(host_group_name, host_name, host_url) ");
            query_builder.push_values(chunk.iter(), |mut b, row| {
                b.push_bind(row.0).push_bind(row.1).push_bind(row.2);
            });

            // on conflict do nothing to avoid duplicate entries
            query_builder.push(" ON CONFLICT (host_group_name, host_name) DO NOTHING ");
            let query = query_builder.build();
            tracing::info!("Executing query: {:?}", query.sql());
            let res = query.execute(&self.pool).await?;
            tracing::info!("Affected rows: {:?}", res.rows_affected());
            rows_inserted += res.rows_affected();
        }
        Ok(rows_inserted)
    }

    pub async fn get_host_from_tuple(
        &self,
        host_tuple: (String, String),
    ) -> Result<Option<HostModel>, RetError> {
        dbg!("fetching host from db with tuple: {:?}", &host_tuple);
        let result = sqlx::query_as!(
            HostModel,
            r#"
            SELECT host_group_name, host_name, host_url FROM host
                WHERE 
                host_group_name = $1
                AND
                host_name = $2
            "#,
            host_tuple.0,
            host_tuple.1,
        )
        .fetch_optional(&self.pool)
        .await?;
    dbg!(&result);

        Ok(result)
    }

    pub async fn get_all_host_groups(&self) -> Result<Vec<HostGroupModel>, sqlx::Error> {
        let groups = sqlx::query!(
            r#"
            SELECT host_group_name
            FROM host_group
            ORDER BY host_group_name
            "#
        )
        .fetch_all(&self.pool)
        .await?;

        let mut result = Vec::with_capacity(groups.len());
        for rec in groups {
            let group_with_hosts = self.get_group_host_with_hosts(rec.host_group_name).await?;
            result.push(group_with_hosts);
        }
        Ok(result)
    }
    async fn get_group_host_with_hosts(
        &self,
        host_group_name: String,
    ) -> Result<HostGroupModel, sqlx::Error> {
        let host_models = sqlx::query_as!(
            HostModel,
            r#"
        select host_group_name, host_name, host_url
        from host
        where host_group_name = $1
        order by host_name
        "#,
            host_group_name
        )
        .fetch_all(&self.pool)
        .await?;

        Ok(HostGroupModel {
            host_group_name,
            host_models,
        })
    }
}

use shared::{
    dto::host,
    model::{
        host::{CreateHostModel, HostModel},
        host_group::{CreateHostGroupModel, HostGroupModel},
        log::HostName,
    },
};
use sqlx::{Pool, Postgres, QueryBuilder};

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
        groups: &[CreateHostGroupModel],
    ) -> Result<u64, sqlx::Error> {
        const CHUNK_SIZE: usize = 1000;
        let mut rows_inserted = 0;
        for chunk in groups.chunks(CHUNK_SIZE) {
            let mut query_builder: QueryBuilder<Postgres> =
                QueryBuilder::new("INSERT INTO host_group(name) ");
            query_builder.push_values(chunk.iter(), |mut b, rec| {
                b.push_bind(&rec.group_name);
            });

            // on conflict do nothing to avoid duplicate entries
            query_builder.push(" ON CONFLICT (name) DO NOTHING");
            let query = query_builder.build();
            let res = query.execute(&self.pool).await?;
            rows_inserted += res.rows_affected();
        }
        Ok(rows_inserted)
    }

    pub async fn bulk_insert_hosts(
        &self,
        host_groups: &[CreateHostGroupModel],
    ) -> Result<u64, sqlx::Error> {
        const CHUNK_SIZE: usize = 500; // rows (hosts) per INSERT
        let tuple_vec: Vec<(&str, &str, &str)> = host_groups
            .iter()
            .flat_map(|g| {
                g.hosts
                    .iter()
                    .map(move |h| (g.group_name.as_str(), h.name.as_str(), h.url.as_str()))
            })
            .collect();
        let mut rows_inserted = 0;
        // let mut rows_staged = 0usize;

        for chunk in tuple_vec.chunks(CHUNK_SIZE) {
            let mut query_builder: QueryBuilder<Postgres> =
                QueryBuilder::new("INSERT INTO host(host_group_id, name, url) ");
            query_builder.push_values(chunk.iter(), |mut b, row| {
                b.push("( (SELECT host_group_id FROM host_group WHERE name = ")
                    .push_bind(&row.0)
                    .push(" ), ")
                    .push_bind(&row.1)
                    .push_bind(&row.2)
                    .push(" ) ");
            });

            // on conflict do nothing to avoid duplicate entries
            query_builder.push(" ON CONFLICT (name) DO NOTHING ");
            let query = query_builder.build();
            let res = query.execute(&self.pool).await?;
            rows_inserted += res.rows_affected();
        }
        Ok(rows_inserted)

    }


    pub async fn get_host_from_hostname(
        &self,
        h_name: HostName,
    ) -> Result<Option<HostModel>, sqlx::Error> {
        let result = sqlx::query_as!(
            HostModel,
            r#"
            select host_id, name, url from host
            where name = $1
            "#,
            h_name
        )
        .fetch_optional(&self.pool)
        .await?;

        Ok(result)
    }

    pub async fn get_all_host_groups(&self) -> Result<Vec<HostGroupModel>, sqlx::Error> {
        let groups = sqlx::query!(
            r#"
            select host_group_id, name
            from host_group
            order by host_group_id
            "#
        )
        .fetch_all(&self.pool)
        .await?;

        let mut result = Vec::with_capacity(groups.len());
        for g in groups {
            let group_with_hosts = self.get_group_hosts_with_hosts(g.host_group_id).await?;
            result.push(group_with_hosts);
        }
        Ok(result)
    }
    async fn get_group_hosts_with_hosts(
        &self,
        group_id: i64,
    ) -> Result<HostGroupModel, sqlx::Error> {
        let group = sqlx::query!(
            r#"
        select host_group_id, name
        from host_group
        where host_group_id = $1
        "#,
            group_id
        )
        .fetch_one(&self.pool)
        .await?;

        let hosts = sqlx::query_as!(
            HostModel,
            r#"
        select host_id, name, url
        from host
        where host_group_id = $1
        order by host_id
        "#,
            group_id
        )
        .fetch_all(&self.pool)
        .await?;

        Ok(HostGroupModel {
            host_group_id: group.host_group_id,
            group_name: group.name,
            hosts,
        })
    }
}

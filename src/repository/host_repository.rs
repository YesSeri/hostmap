use sqlx::{Pool, Postgres};

use crate::model::{
    host::HostModel,
    host_group::{CreateHostGroupModel, HostGroupModel},
    log::HostName,
};

#[derive(Debug, Clone)]
pub struct HostRepository {
    pool: Pool<Postgres>,
}

impl HostRepository {
    pub fn new(pool: Pool<Postgres>) -> Self {
        Self { pool }
    }
    pub async fn insert_group_hosts_with_hosts(
        &self,
        group: &CreateHostGroupModel,
    ) -> Result<i64, sqlx::Error> {
        let mut tx = self.pool.begin().await?;
        let host_group_id: i64 = sqlx::query_scalar!(
            r#"
            insert into host_group(name)
            values ($1)
            returning host_group_id
            "#,
            group.group_name
        )
        .fetch_one(&mut *tx)
        .await?;
        for h in &group.hosts {
            sqlx::query!(
                r#"
                insert into host(host_group_id, name, url)
                values ($1, $2, $3)
                "#,
                host_group_id,
                h.name,
                h.url
            )
            .execute(&mut *tx)
            .await?;
        }
        tx.commit().await?;
        Ok(host_group_id)
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

use serde::{Deserialize, Serialize};
use sqlx::{Pool, Postgres};

use crate::dto::host::{HostDto, HostGroupDto};

#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct HostModel {
    pub host_id: Option<i64>,
    pub name: String,
    pub url: String,
}

impl From<HostDto> for HostModel {
    fn from(HostDto { name, url }: HostDto) -> Self {
        Self {
            host_id: None,
            name,
            url,
        }
    }
}

#[derive(Debug, Clone, Serialize)]
pub struct HostGroupModel {
    pub host_group_id: Option<i64>,
    pub group_name: String,
    pub hosts: Vec<HostModel>,
}

impl From<HostGroupDto> for HostGroupModel {
    fn from(
        HostGroupDto {
            group_name,
            host_dtos,
        }: HostGroupDto,
    ) -> Self {
        let hosts = host_dtos.into_iter().map(HostModel::from).collect();
        Self {
            host_group_id: None,
            group_name,
            hosts,
        }
    }
}

#[derive(Debug, Clone)]
pub struct PgHostRepository {
    pool: Pool<Postgres>,
}

impl PgHostRepository {
    pub fn new(pool: Pool<Postgres>) -> Self {
        Self { pool }
    }
    pub async fn insert_group_hosts_with_hosts(
        &self,
        group: &HostGroupModel,
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
            host_group_id: Some(group.host_group_id),
            group_name: group.name,
            hosts,
        })
    }

    pub async fn get_latest_log_entry_for_host(
        &self,
        host_id: i64,
    ) -> Result<Option<crate::model::log::LogEntryModel>, sqlx::Error> {
        let log_entry = sqlx::query_as!(
            crate::model::log::LogEntryModel,
            r#"
        select log_entry_id, timestamp, username, host_id, store_path, activation_type
        from log_entry
        where host_id = $1
        order by timestamp desc
        limit 1
        "#,
            host_id
        )
        .fetch_optional(&self.pool)
        .await?;
        Ok(log_entry)
    }
}

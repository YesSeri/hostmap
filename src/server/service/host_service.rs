use sqlx::{Pool, Postgres};

use crate::{
    server::custom_error::RetError,
    server::repository::host_repository::HostRepository,
    shared::model::host::{HostModel, HostWithLatestLog},
};

#[derive(Debug, Clone)]
pub struct HostService {
    pool: Pool<Postgres>,
}

impl HostService {
    pub fn new(pool: Pool<Postgres>) -> Self {
        Self { pool }
    }

    pub async fn get_all_with_latest_log(&self) -> Result<Vec<HostWithLatestLog>, RetError> {
        let hosts = HostRepository::get_all_hosts_with_latest_activation(&self.pool).await?;
        Ok(hosts)
    }

    pub async fn create_many(&self, hosts: &[HostModel]) -> Result<u64, sqlx::Error> {
        let hosts = HostRepository::bulk_insert_hosts(&self.pool, hosts).await?;
        Ok(hosts)
    }

    pub async fn get_host_from_hostname(
        &self,
        hostname: String,
    ) -> Result<Option<HostModel>, RetError> {
        let host = HostRepository::get_host_from_hostname(&self.pool, hostname).await?;
        Ok(host)
    }
}

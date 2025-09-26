use std::collections::BTreeMap;

use chrono::NaiveDate;

use crate::{
    server::custom_error::RetError,
    server::repository::host_repository::HostRepository,
    shared::model::host::{HostModel, HostWithLatestLog},
};

#[derive(Debug, Clone)]
pub struct HostService {
    repo: HostRepository,
}

impl HostService {
    pub fn new(repo: HostRepository) -> Self {
        Self { repo }
    }

    pub async fn get_all_hosts_with_latest_log_entry(
        &self,
    ) -> Result<Vec<HostWithLatestLog>, RetError> {
        let hosts = self.repo.get_all_hosts_with_latest_log_entry().await?;
        Ok(hosts)
    }
    pub async fn get_all_hosts(&self) -> Result<Vec<HostModel>, RetError> {
        let hosts = self.repo.get_all_hosts().await?;
        Ok(hosts)
    }

    pub async fn bulk_insert_hosts(&self, hosts: &[HostModel]) -> Result<u64, sqlx::Error> {
        let hosts = self.repo.bulk_insert_hosts(hosts).await?;
        Ok(hosts)
    }

    pub async fn get_host_from_hostname(
        &self,
        hostname: String,
    ) -> Result<Option<HostModel>, RetError> {
        let host = self.repo.get_host_from_hostname(hostname).await?;
        Ok(host)
    }
}

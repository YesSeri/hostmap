use std::collections::BTreeMap;

use chrono::NaiveDate;
use sqlx::error::BoxDynError;

use crate::{
    model::log::{ExistingLogEntryModel, HostId, NewLogEntryModel},
    repository::activation_log_repository::ActivationLogRepository,
    RetError,
};

#[derive(Debug, Clone)]
pub struct ActivationLogService {
    repo: ActivationLogRepository,
}

impl ActivationLogService {
    pub fn new(repo: ActivationLogRepository) -> Self {
        Self { repo }
    }
    pub(crate) async fn latest_entry_for_host(
        &self,
        host_id: HostId,
    ) -> Result<Option<ExistingLogEntryModel>, Box<RetError>> {
        self.repo.latest_entry_for_host(host_id).await
    }
    pub(crate) async fn insert_activation_log_record(
        &self,
        rec: &NewLogEntryModel,
    ) -> Result<(), Box<RetError>> {
        self.repo.insert_activation_log_record(rec).await
    }
    pub async fn activation_logs_by_date_for_host_name(
        &self,
        host_id: HostId,
    ) -> Result<BTreeMap<NaiveDate, Vec<ExistingLogEntryModel>>, Box<RetError>> {
        println!("getting acti logs by date");
        let log_list = self.repo.entries_for_host(host_id).await?;
        println!("getting acti logs by date2");
        dbg!(&log_list);
        let mut log_map: BTreeMap<NaiveDate, Vec<ExistingLogEntryModel>> = BTreeMap::new();

        for item in log_list {
            let date = item.timestamp.date_naive();
            log_map.entry(date).or_default().push(item);
        }
        dbg!(&log_map);

        Ok(log_map)
    }
}

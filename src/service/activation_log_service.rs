use std::collections::BTreeMap;

use chrono::NaiveDate;
use sqlx::error::BoxDynError;

use crate::{
    model::{
        host::ExistingHostModel,
        log::{ExistingLogEntryModel, HostId, LogEntryWithRevision, NewLogEntryModel},
    },
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

    pub async fn host_with_logs_by_name(
        &self,
        host: &ExistingHostModel,
    ) -> Result<BTreeMap<NaiveDate, Vec<LogEntryWithRevision>>, Box<RetError>> {
        let (host, logs) = self.repo.host_with_logs_by_name(&host.name).await?;
        let mut map: BTreeMap<NaiveDate, Vec<LogEntryWithRevision>> = BTreeMap::new();
        for log in logs {
            let date = log.timestamp.date_naive();
            map.entry(date).or_default().push(log.clone());
        }
        Ok(map)
    }
}

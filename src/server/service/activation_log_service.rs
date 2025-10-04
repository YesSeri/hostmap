use std::collections::BTreeMap;

use chrono::NaiveDate;

use crate::{
    server::{
        custom_error::RetError, repository::activation_log_repository::ActivationLogRepository,
    },
    shared::{
        dto::host::CurrentHostDto,
        model::{
            host::HostModel,
            log::{CreateLogEntryModel, ExistingLogEntryModel, LogEntryWithRevision},
        },
    },
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
        host_dto: CurrentHostDto,
    ) -> Result<Option<ExistingLogEntryModel>, RetError> {
        let model: HostModel = host_dto.into();
        self.repo.latest_entry_for_host(model).await
    }

    pub async fn host_with_logs_by_hostname(
        &self,
        hostname: &str,
    ) -> Result<BTreeMap<NaiveDate, Vec<LogEntryWithRevision>>, RetError> {
        let logs = self.repo.get_logs_by_hostname(hostname).await?;
        let mut map: BTreeMap<NaiveDate, Vec<LogEntryWithRevision>> = BTreeMap::new();
        for log in logs {
            let date = log.timestamp.date_naive();
            map.entry(date).or_default().push(log.clone());
        }
        Ok(map)
    }

    pub(crate) async fn bulk_insert_log_records(
        &self,
        log_entry_models: &[CreateLogEntryModel],
    ) -> Result<u64, RetError> {
        self.repo
            .bulk_insert_log_records_with_store_paths(log_entry_models)
            .await
    }
}

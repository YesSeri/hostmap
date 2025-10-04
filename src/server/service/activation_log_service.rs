use std::collections::BTreeMap;

use chrono::NaiveDate;

use crate::{
    server::{custom_error::RetError, repository::activation_repository::ActivationRepository},
    shared::{
        dto::host::CurrentHostDto,
        model::{
            activation::{Activation, ActivationWithRevision, NewActivation},
            host::HostModel,
        },
    },
};

#[derive(Debug, Clone)]
pub struct ActivationLogService {
    repo: ActivationRepository,
}

impl ActivationLogService {
    pub fn new(repo: ActivationRepository) -> Self {
        Self { repo }
    }
    pub(crate) async fn latest_entry_for_host(
        &self,
        host_dto: CurrentHostDto,
    ) -> Result<Option<Activation>, RetError> {
        let model: HostModel = host_dto.into();
        self.repo.latest_entry_for_host(model).await
    }

    pub async fn host_with_logs_by_hostname(
        &self,
        hostname: &str,
    ) -> Result<BTreeMap<NaiveDate, Vec<ActivationWithRevision>>, RetError> {
        let logs = self.repo.get_logs_by_hostname(hostname).await?;
        let mut map: BTreeMap<NaiveDate, Vec<ActivationWithRevision>> = BTreeMap::new();
        for log in logs {
            let date = log.activated_at.date_naive();
            map.entry(date).or_default().push(log.clone());
        }
        Ok(map)
    }

    pub(crate) async fn bulk_insert_log_records(
        &self,
        new_activations: &[NewActivation],
    ) -> Result<u64, RetError> {
        self.repo
            .insert_many_activations_with_store_paths(new_activations)
            .await
    }
}

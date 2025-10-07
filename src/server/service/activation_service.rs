use std::collections::BTreeMap;

use chrono::NaiveDate;
use sqlx::{Pool, Postgres};

use crate::{
    server::{
        custom_error::RetError,
        repository::{
            activation_repository::ActivationRepository, store_path_repository::StorePathRepository,
        },
    },
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
    pool: Pool<Postgres>,
}

impl ActivationLogService {
    pub fn new(pool: Pool<Postgres>) -> Self {
        Self { pool }
    }

    pub async fn host_with_logs_by_hostname(
        &self,
        hostname: &str,
    ) -> Result<BTreeMap<NaiveDate, Vec<ActivationWithRevision>>, RetError> {
        let logs = ActivationRepository::get_logs_by_hostname(&self.pool, hostname).await?;
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
        let mut tx = self.pool.begin().await?;

        let store_paths: Vec<&str> = new_activations
            .iter()
            .map(|el| el.core.store_path.as_str())
            .collect();
        StorePathRepository::bulk_insert_store_paths(&mut tx, &store_paths).await?;

        let i = ActivationRepository::insert_many(&mut tx, new_activations).await?;
        tx.commit().await?;
        Ok(i)
    }
}

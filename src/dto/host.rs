use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};

use crate::{
    dto::revision::StorePathDto,
    model::{host::HostModel, log::ExistingLogEntryModel},
};

#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct HostDto {
    pub name: String,
    pub url: String,
}
impl From<HostModel> for HostDto {
    fn from(HostModel { name, url, .. }: HostModel) -> Self {
        Self { name, url }
    }
}

#[derive(Debug, Clone, Deserialize, Serialize, PartialEq)]
pub(crate) struct CurrentHostDto {
    pub(crate) host_name: String,
    pub(crate) host_id: i64,
    pub(crate) url: String,
    pub(crate) store_path: StorePathDto,
    pub(crate) activation_type: String,
    pub(crate) timestamp: DateTime<Utc>,
    pub(crate) rev_id: Option<String>,
    pub(crate) branch: Option<String>,
}

impl From<(HostModel, ExistingLogEntryModel)> for CurrentHostDto {
    fn from(
        (
            HostModel {
                name: host_name,
                url,
                host_id,
            },
            ExistingLogEntryModel {
                store_path,
                activation_type,
                timestamp,
                revision,
                ..
            },
        ): (HostModel, ExistingLogEntryModel),
    ) -> Self {
        let (rev_id, branch) = match revision {
            Some(r) => (Some(r.rev_id), Some(r.branch)),
            None => (None, None),
        };

        Self {
            host_name,
            url,
            host_id,
            store_path: StorePathDto::from(store_path),
            activation_type,
            timestamp,
            rev_id,
            branch,
        }
    }
}

use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};

use crate::{
    dto::{
        log::{LogEntryDto, LogHistoryDto},
        revision::StorePathDto,
    },
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

#[derive(Debug, Clone, Deserialize, Serialize)]
pub(crate) struct CurrentHostDto {
    pub(crate) host_name: String,
    pub(crate) host_id: i64,
    pub(crate) url: String,
    pub(crate) log_entry: Option<LogHistoryDto>,
}

impl From<(HostModel, Option<ExistingLogEntryModel>)> for CurrentHostDto {
    fn from(
        (
            HostModel {
                name: host_name,
                url,
                host_id,
            },
            log_entry,
        ): (HostModel, Option<ExistingLogEntryModel>),
    ) -> Self {
        let log_entry = log_entry.map(LogHistoryDto::from);

        Self {
            host_name,
            url,
            host_id,
            log_entry,
        }
    }
}

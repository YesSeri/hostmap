use serde::{Deserialize, Serialize};

use crate::{
    dto::log::LogHistoryDto,
    model::{host::HostModel, log::ExistingLogEntryModel},
};

#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct CreateHostDto {
    pub name: String,
    pub url: String,
}
impl From<HostModel> for CreateHostDto {
    fn from(HostModel { name, url, .. }: HostModel) -> Self {
        Self { name, url }
    }
}

#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct HostDto {
    pub host_name: String,
    pub host_id: i64,
    pub url: String,
}
impl From<HostModel> for HostDto {
    fn from(HostModel { name, url, host_id }: HostModel) -> Self {
        Self {
            host_name: name,
            url,
            host_id,
        }
    }
}
#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct HostWithLogsDto {
    pub host_name: String,
    pub host_id: i64,
    pub url: String,
    pub log_entry: Vec<LogHistoryDto>,
}

impl From<(HostModel, ExistingLogEntryModel)> for HostWithLogsDto {
    fn from(
        (
            HostModel {
                name: host_name,
                url,
                host_id,
            },
            log_entry,
        ): (HostModel, ExistingLogEntryModel),
    ) -> Self {

        Self {
            host_name,
            url,
            host_id,
            log_entry: log_entry.into(),
        }
    }
}

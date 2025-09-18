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
pub struct HostDto<L> {
    pub host_name: String,
    pub host_id: i64,
    pub url: String,
    pub logs: L,
}

impl From<HostModel> for HostDtoNoLogs {
    fn from(HostModel { name, url, host_id }: HostModel) -> Self {
        Self {
            host_name: name,
            url,
            host_id,
            logs: (),
        }
    }
}
impl From<(HostModel, Option<ExistingLogEntryModel>)> for HostDto<Option<LogHistoryDto>> {
    fn from(
        (HostModel { name, url, host_id }, log_entry): (HostModel, Option<ExistingLogEntryModel>),
    ) -> Self {
        Self {
            host_name: name,
            url,
            host_id,
            logs: log_entry.map(Into::into),
        }
    }
}

impl From<(HostModel, Vec<ExistingLogEntryModel>)> for HostDto<Vec<LogHistoryDto>> {
    fn from(
        (HostModel { name, url, host_id }, entries): (HostModel, Vec<ExistingLogEntryModel>),
    ) -> Self {
        Self {
            host_name: name,
            url,
            host_id,
            logs: entries.into_iter().map(Into::into).collect(),
        }
    }
}

pub type HostDtoNoLogs = HostDto<()>;
pub type CurrentHostDto = HostDto<Option<LogHistoryDto>>;
pub type HostWithLogsDto = HostDto<Vec<LogHistoryDto>>;

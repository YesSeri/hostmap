use serde::{Deserialize, Serialize};

use crate::{
    dto::log::LogHistoryDto,
    model::{host::HostModel, host_group::HostGroupName, log::ExistingLogEntryModel},
};

// #[derive(Debug, Clone, Deserialize, Serialize)]
// pub struct CreateHostDto {
//     pub host_name: String,
//     pub host_url: String,
// }
// impl From<HostModel> for CreateHostDto {
//     fn from(HostModel { host_name: name, host_url: url, .. }: HostModel) -> Self {
//         Self { host_name: name, host_url: url }
//     }
// }

#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct HostDto<L> {
    pub host_name: String,
    pub host_group_name: String,
    pub host_url: String,
    pub logs: L,
}

impl From<HostModel> for HostDtoNoLogs {
    fn from(
        HostModel {
            host_name: name,
            host_url,
            host_group_name,
        }: HostModel,
    ) -> Self {
        Self {
            host_name: name,
            host_url,
            host_group_name,
            logs: (),
        }
    }
}
impl From<(HostModel, Option<ExistingLogEntryModel>)> for HostDto<Option<LogHistoryDto>> {
    fn from(
        (
            HostModel {
                host_name,
                host_url,
                host_group_name,
            },
            log_entry,
        ): (HostModel, Option<ExistingLogEntryModel>),
    ) -> Self {
        Self {
            host_name,
            host_group_name,
            host_url,
            logs: log_entry.map(Into::into),
        }
    }
}

impl From<(HostModel, Vec<ExistingLogEntryModel>)> for HostDto<Vec<LogHistoryDto>> {
    fn from(
        (
            HostModel {
                host_name,
                host_url,
                host_group_name,
            },
            entries,
        ): (HostModel, Vec<ExistingLogEntryModel>),
    ) -> Self {
        Self {
            host_name,
            host_group_name,
            host_url,
            logs: entries.into_iter().map(Into::into).collect(),
        }
    }
}

pub type HostDtoNoLogs = HostDto<()>;
impl HostDtoNoLogs {
    pub fn new(host_name: String, host_group_name: String, host_url: String) -> Self {
        Self {
            host_name,
            host_group_name,
            host_url,
            logs: (),
        }
    }
}
pub type CurrentHostDto = HostDto<Option<LogHistoryDto>>;
pub type HostWithLogsDto = HostDto<Vec<LogHistoryDto>>;

impl From<HostModel> for CurrentHostDto {
    fn from(
        HostModel {
            host_name,
            host_group_name,
            host_url,
        }: HostModel,
    ) -> Self {
        Self {
            host_name,
            host_group_name,
            host_url,
            logs: None,
        }
    }
}

#[derive(Debug, Clone, Deserialize)]
pub struct IncomingHostDto {
    host_name: String,
    host_url: String,
}

impl From<(IncomingHostDto, &HostGroupName)> for CurrentHostDto {
    fn from((inc, group_name): (IncomingHostDto, &HostGroupName)) -> Self {
        Self {
            host_name: inc.host_name,
            host_group_name: group_name.0.clone(),
            host_url: inc.host_url,
            logs: None,
        }
    }
}

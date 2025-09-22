use serde::{Deserialize, Serialize};

use crate::{
    dto::log::LogHistoryDto,
    model::{host::HostModel, log::ExistingLogEntryModel},
};

#[derive(Debug, Clone, Deserialize, Serialize, PartialEq, Eq)]
pub struct HostDto<L> {
    pub host_name: String,
    pub host_group_name: String,
    pub host_url: String,
    pub logs: L,
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

pub type HostWithLogsDto = HostDto<Vec<LogHistoryDto>>;
pub type CurrentHostDto = HostDto<Option<LogHistoryDto>>;

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

// #[derive(Debug, Clone, Deserialize)]
// pub struct IncomingHostDto {
//     host_name: String,
//     host_url: String,
// }

// impl From<(IncomingHostDto, &HostGroupName)> for CurrentHostDto {
//     fn from((inc, group_name): (IncomingHostDto, &HostGroupName)) -> Self {
//         Self {
//             host_name: inc.host_name,
//             host_group_name: group_name.0.clone(),
//             host_url: inc.host_url,
//             logs: None,
//         }
//     }
// }

#[derive(Deserialize, Serialize, Debug)]
pub struct RawHost {
    #[serde(rename="name")]
    pub(crate) host_name: String,
    #[serde(rename="url")]
    pub(crate) host_url: String,
}

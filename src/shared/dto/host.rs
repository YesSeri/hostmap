use std::collections::HashMap;

use serde::{Deserialize, Serialize};

use crate::shared::{
    dto::log::LogHistoryDto,
    model::{host::HostModel, log::ExistingLogEntryModel},
};

#[derive(Debug, Clone, Deserialize, Serialize, PartialEq, Eq)]
pub struct HostDto<L> {
    pub hostname: String,
    pub host_url: String,
    pub metadata: HashMap<String, String>,
    pub logs: L,
}

impl From<(HostModel, Option<ExistingLogEntryModel>)> for HostDto<Option<LogHistoryDto>> {
    fn from(
        (
            HostModel {
                hostname,
                host_url,
                metadata,
            },
            log_entry,
        ): (HostModel, Option<ExistingLogEntryModel>),
    ) -> Self {
        Self {
            hostname,
            host_url,
            logs: log_entry.map(Into::into),
            metadata,
        }
    }
}

impl From<(HostModel, Vec<ExistingLogEntryModel>)> for HostDto<Vec<LogHistoryDto>> {
    fn from(
        (
            HostModel {
                hostname,
                host_url,
                metadata,
            },
            entries,
        ): (HostModel, Vec<ExistingLogEntryModel>),
    ) -> Self {
        Self {
            hostname,
            host_url,
            logs: entries.into_iter().map(Into::into).collect(),
            metadata,
        }
    }
}

pub type HostWithLogsDto = HostDto<Vec<LogHistoryDto>>;
pub type CurrentHostDto = HostDto<Option<LogHistoryDto>>;

impl From<HostModel> for CurrentHostDto {
    fn from(
        HostModel {
            hostname,
            host_url,
            metadata,
        }: HostModel,
    ) -> Self {
        Self {
            hostname,
            host_url,
            logs: None,
            metadata,
        }
    }
}

// #[derive(Debug, Clone, Deserialize)]
// pub struct IncomingHostDto {
//     hostname: String,
//     host_url: String,
// }

// impl From<(IncomingHostDto, &HostGroupName)> for CurrentHostDto {
//     fn from((inc, group_name): (IncomingHostDto, &HostGroupName)) -> Self {
//         Self {
//             hostname: inc.hostname,
//             host_url: inc.host_url,
//             logs: None,
//         }
//     }
// }

#[derive(Deserialize, Serialize, Debug)]
pub struct RawHost {
    #[serde(rename = "name")]
    pub(crate) hostname: String,
    #[serde(rename = "url")]
    pub(crate) host_url: String,
}

use serde::{Deserialize, Serialize};

use crate::shared::{
    dto::host::CurrentHostDto,
    model::log::ExistingLogEntryModel,
};

#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct HostModel {
    pub hostname: String,
    pub host_url: String,
    pub metadata: serde_json::Value,
}

impl From<CurrentHostDto> for HostModel {
    fn from(
        CurrentHostDto {
            hostname,
            host_url,
            metadata,
            logs,
        }: CurrentHostDto,
    ) -> Self {
        Self {
            hostname,
            host_url,
            metadata,
        }
    }
}

pub struct HostWithLatestLog {
    pub host: HostModel,
    pub logs: Option<ExistingLogEntryModel>,
}

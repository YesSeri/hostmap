use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};

use crate::{
    dto::revision::{RevisionDto, StorePathDto},
    model::{host::ExistingHostModel, log::ExistingLogEntryModel},
};

#[derive(Debug, Deserialize, Serialize)]
pub struct LogEntryDto {
    #[serde(deserialize_with = "from_custom_fmt")]
    pub timestamp: DateTime<Utc>,
    pub username: String,
    pub store_path: String,
    pub activation_type: String,
    #[serde(default)]
    pub revision: Option<RevisionDto>,
}
fn from_custom_fmt<'de, D>(deserializer: D) -> Result<DateTime<Utc>, D::Error>
where
    D: serde::Deserializer<'de>,
{
    let s = String::deserialize(deserializer)?;
    let dt =
        DateTime::parse_from_str(&s, "%Y-%m-%d %H:%M:%S%:z").map_err(serde::de::Error::custom)?;
    Ok(dt.with_timezone(&Utc))
}

impl From<ExistingLogEntryModel> for LogEntryDto {
    fn from(
        ExistingLogEntryModel {
            timestamp,
            username,
            store_path,
            activation_type,
            revision,
            ..
        }: ExistingLogEntryModel,
    ) -> Self {
        Self {
            timestamp,
            username,
            store_path,
            activation_type,
            revision: revision.map(|el| el.into()),
        }
    }
}

#[derive(Debug, Deserialize, Serialize, Clone)]
pub struct LogHistoryDto {
    pub(crate) host_name: String,
    pub(crate) host_id: i64,
    pub(crate) activation_type: String,
    pub(crate) timestamp: DateTime<Utc>,
    pub(crate) username: String,
    pub(crate) rev_id: Option<String>,
    pub(crate) store_path: StorePathDto,
    pub(crate) revision: Option<RevisionDto>,
}

impl From<(ExistingHostModel, ExistingLogEntryModel)> for LogHistoryDto {
    fn from((host, log): (ExistingHostModel, ExistingLogEntryModel)) -> Self {
        Self {
            host_name: host.name,
            host_id: host.host_id,
            activation_type: log.activation_type,
            timestamp: log.timestamp,
            username: log.username,
            rev_id: log.revision.as_ref().map(|r| r.rev_id.clone()),
            store_path: log.store_path.into(),
            revision: log.revision.map(|r| r.into()),
        }
    }
}

use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};

use crate::shared::{
    dto::revision::{RevisionDto, StorePathDto},
    model::log::{ExistingLogEntryModel, NewLogEntryModel},
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
    let s: String = Deserialize::deserialize(deserializer)?;
    DateTime::parse_from_rfc3339(&s)
        .map(|dt| dt.with_timezone(&Utc))
        .map_err(serde::de::Error::custom)
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

#[derive(Debug, Deserialize, Serialize, Clone, PartialEq, Eq)]
pub struct LogHistoryDto {
    pub activation_type: String,
    pub timestamp: DateTime<Utc>,
    pub username: String,
    pub store_path: StorePathDto,
    pub revision: Option<RevisionDto>,
}

impl From<ExistingLogEntryModel> for LogHistoryDto {
    fn from(log: ExistingLogEntryModel) -> Self {
        Self {
            activation_type: log.activation_type,
            timestamp: log.timestamp,
            username: log.username,
            store_path: log.store_path.into(),
            revision: log.revision.map(|r| r.into()),
        }
    }
}

impl From<NewLogEntryModel> for LogHistoryDto {
    fn from(
        NewLogEntryModel {
            activation_type,
            timestamp,
            username,
            store_path,
            revision,
            ..
        }: NewLogEntryModel,
    ) -> Self {
        Self {
            activation_type,
            timestamp,
            username,
            store_path: store_path.into(),
            revision: revision.map(|r| r.into()),
        }
    }
}

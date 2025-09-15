use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};

use crate::{dto::revision::RevisionDto, model::log::ExistingLogEntryModel};

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

#[derive(Debug, Deserialize, Serialize)]
pub struct LogHistoryDto {
    #[serde(deserialize_with = "from_custom_fmt")]
    pub timestamp: DateTime<Utc>,
    pub username: String,
    pub store_path: String,
    pub activation_type: String,
    #[serde(default)]
    pub revision: Option<RevisionDto>,
}

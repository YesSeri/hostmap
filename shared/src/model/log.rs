use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};

use crate::{dto::{host::CurrentHostDto, log::LogEntryDto}, model::revision::RevisionModel};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CreateLogEntryModel {
    pub timestamp: DateTime<Utc>,
    pub username: String,
    pub store_path: String,
    pub activation_type: String,
    pub hostname: String,
}
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct LogEntryModel<IdType> {
    pub log_entry_id: IdType,
    pub timestamp: DateTime<Utc>,
    pub username: String,
    pub store_path: String,
    pub activation_type: String,
    pub host_id: i64,
    pub revision: Option<RevisionModel>,
}

pub type ExistingLogEntryModel = LogEntryModel<i64>;
pub type NewLogEntryModel = LogEntryModel<()>;
impl From<(HostId, LogEntryDto)> for NewLogEntryModel {
    fn from(
        (host_id, LogEntryDto {
            timestamp,
            username,
            store_path,
            activation_type,
            revision,
        }): (HostId, LogEntryDto),
    ) -> Self {
        Self {
            log_entry_id: (),
            timestamp,
            username,
            store_path,
            activation_type,
            host_id,
            revision: revision.map(|r| r.into()),
        }
    }
}

pub type HostId = i64;
pub type HostName = String;


#[derive(Debug, Clone)]
pub struct LogEntryWithRevision {
    pub log_entry_id: i64,
    pub timestamp: DateTime<Utc>,
    pub username: String,
    pub store_path: String,
    pub activation_type: String,
    pub host_id: i64,
    pub rev_id: Option<String>,
    pub branch: Option<String>,
}

impl From<LogEntryWithRevision> for ExistingLogEntryModel {
    fn from(entry: LogEntryWithRevision) -> Self {
        let revision = entry.clone().into();
        Self {
            log_entry_id: entry.log_entry_id,
            timestamp: entry.timestamp,
            username: entry.username,
            store_path: entry.store_path,
            activation_type: entry.activation_type,
            host_id: entry.host_id,
            revision,
        }
    }
}

impl From<LogEntryWithRevision> for Option<RevisionModel> {
    fn from(LogEntryWithRevision { rev_id, branch, .. }: LogEntryWithRevision) -> Self {
        match (rev_id, branch) {
            (Some(r), Some(b)) => Some(RevisionModel {
                rev_id: r,
                branch: b,
            }),
            _ => None,
        }
    }
}

impl From<CurrentHostDto> for CreateLogEntryModel {
    fn from(CurrentHostDto { host_id, url, host_name, log_entry }: CurrentHostDto) -> Self {
        Self {
            timestamp: log_entry.timestamp,
            username: log_entry.username,
            store_path: log_entry.store_path.store_path,
            activation_type: log_entry.activation_type,
            hostname: host_name
        }
    }
}
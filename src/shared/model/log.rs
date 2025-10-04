use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};

use crate::shared::{
    dto::{
        host::{CurrentHostDto, HostWithLogsDto},
        log::{LogEntryDto, LogHistoryDto},
    },
    model::{host::HostModel, revision::RevisionModel},
};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CreateLogEntryModel {
    pub timestamp: DateTime<Utc>,
    pub username: String,
    pub store_path: String,
    pub activation_type: String,
    pub hostname: String,
}
impl From<(&HostWithLogsDto, LogHistoryDto)> for CreateLogEntryModel {
    fn from((host_with_logs_dto, log_history_dto): (&HostWithLogsDto, LogHistoryDto)) -> Self {
        Self {
            timestamp: log_history_dto.timestamp,
            username: log_history_dto.username,
            store_path: log_history_dto.store_path.store_path,
            activation_type: log_history_dto.activation_type,
            hostname: host_with_logs_dto.hostname.clone(),
        }
    }
}

pub struct HostName(pub String);

impl HostName {
    pub fn new(v: String) -> Self {
        Self(v)
    }
}
impl From<(CurrentHostDto, LogHistoryDto)> for CreateLogEntryModel {
    fn from((host_dto, log_dto): (CurrentHostDto, LogHistoryDto)) -> Self {
        Self {
            timestamp: log_dto.timestamp,
            username: log_dto.username,
            store_path: log_dto.store_path.store_path,
            activation_type: log_dto.activation_type,
            hostname: host_dto.hostname,
        }
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct LogEntryModel<IdType> {
    pub log_entry_id: IdType,
    pub timestamp: DateTime<Utc>,
    pub username: String,
    pub store_path: String,
    pub activation_type: String,
    pub hostname: String,
    pub revision: Option<RevisionModel>,
}

impl From<LogEntryWithRevision> for LogEntryModel<i64> {
    fn from(entry: LogEntryWithRevision) -> Self {
        let revision = entry.clone().into();
        Self {
            log_entry_id: entry.log_entry_id,
            timestamp: entry.timestamp,
            username: entry.username,
            store_path: entry.store_path,
            activation_type: entry.activation_type,
            revision,
            hostname: entry.hostname,
        }
    }
}

pub type ExistingLogEntryModel = LogEntryModel<i64>;
pub type NewLogEntryModel = LogEntryModel<()>;

impl From<(HostModel, LogEntryDto)> for NewLogEntryModel {
    fn from((host, log): (HostModel, LogEntryDto)) -> Self {
        Self {
            log_entry_id: (),
            timestamp: log.timestamp,
            username: log.username,
            store_path: log.store_path,
            activation_type: log.activation_type,
            revision: log.revision.map(Into::into),
            hostname: host.hostname,
        }
    }
}

#[derive(Debug, Clone)]
pub struct LogEntryWithRevision {
    pub log_entry_id: i64,
    pub timestamp: DateTime<Utc>,
    pub username: String,
    pub store_path: String,
    pub activation_type: String,
    pub hostname: String,
    pub rev_id: Option<String>,
    pub branch: Option<String>,
}

// impl From<LogEntryWithRevision> for ExistingLogEntryModel {
//     fn from(entry: LogEntryWithRevision) -> Self {
//         let revision = entry.clone().into();
//         Self {
//             log_entry_id: entry.log_entry_id,
//             timestamp: entry.timestamp,
//             username: entry.username,
//             store_path: entry.store_path,
//             activation_type: entry.activation_type,
//             revision,
//             hostname: todo!(),
//             hostgroup_name: todo!(),
//         }
//     }
// }

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

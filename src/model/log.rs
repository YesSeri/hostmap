use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use sqlx::prelude::FromRow;

use crate::{dto::log::LogEntryDto, model::revision::RevisionModel};

#[derive(Debug, FromRow, Clone, Serialize, Deserialize)]
pub(crate) struct LogEntryModel<IdType> {
    pub log_entry_id: IdType,
    pub timestamp: DateTime<Utc>,
    pub username: String,
    pub store_path: String,
    pub activation_type: String,
    pub host_id: i64,
    pub revision: Option<RevisionModel>,
}

pub(crate) type ExistingLogEntryModel = LogEntryModel<i64>;
pub(crate) type NewLogEntryModel = LogEntryModel<()>;

#[derive(Debug, Clone, PartialEq, Eq, Hash, Copy)]
pub(crate) struct HostId(pub(crate) i64);
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub(crate) struct HostName(pub(crate) String);
impl From<String> for HostName {
    fn from(s: String) -> Self {
        HostName(s)
    }
}
impl From<HostName> for String {
    fn from(host_name: HostName) -> Self {
        host_name.0
    }
}

impl From<i64> for HostId {
    fn from(num: i64) -> Self {
        HostId(num)
    }
}
impl From<HostId> for i64 {
    fn from(host_id: HostId) -> Self {
        host_id.0
    }
}

impl From<(LogEntryDto, HostId)> for NewLogEntryModel {
    fn from(
        (
            LogEntryDto {
                timestamp,
                username,
                store_path,
                activation_type,
                revision,
            },
            HostId(host_id),
        ): (LogEntryDto, HostId),
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

#[derive(Debug, FromRow, Clone)]
pub(crate) struct LogEntryWithRevision {
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

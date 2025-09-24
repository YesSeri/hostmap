use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};

use crate::shared::dto::revision::RevisionDto;

use super::log::LogEntryWithRevision;

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct RevisionModel {
    pub rev_id: String,
    pub branch: String,
}

impl From<LogEntryWithRevision> for RevisionModel {
    fn from(LogEntryWithRevision { rev_id, branch, .. }: LogEntryWithRevision) -> Self {
        let rev_id = rev_id.unwrap_or("Commit id unknown".to_owned());
        let branch = branch.unwrap_or("Unknown branch".to_owned());
        Self { rev_id, branch }
    }
}

impl From<RevisionDto> for RevisionModel {
    fn from(RevisionDto { rev_id, branch }: RevisionDto) -> Self {
        Self { rev_id, branch }
    }
}

#[derive(Debug, Serialize, Deserialize)]
pub struct StorePathModel {
    pub id: String,
    pub created_at: DateTime<Utc>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct RevisionStorePath {
    pub revision_id: String,
    pub store_path_id: String,
}

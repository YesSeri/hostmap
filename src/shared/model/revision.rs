use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};

use crate::shared::{dto::revision::RevisionDto, model::activation::ActivationWithRevision};

#[derive(Debug, Serialize, Deserialize, Clone, PartialEq, Eq)]
pub struct RevisionModel {
    pub commit_hash: String,
    pub branch: String,
}

impl From<ActivationWithRevision> for RevisionModel {
    fn from(
        ActivationWithRevision {
            branch,
            commit_hash,
            ..
        }: ActivationWithRevision,
    ) -> Self {
        let commit_hash = commit_hash.unwrap_or("N/A".to_owned());
        let branch = branch.unwrap_or("N/A".to_owned());
        Self {
            commit_hash,
            branch,
        }
    }
}

impl From<RevisionDto> for RevisionModel {
    fn from(
        RevisionDto {
            branch,
            commit_hash,
        }: RevisionDto,
    ) -> Self {
        Self {
            branch,
            commit_hash,
        }
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

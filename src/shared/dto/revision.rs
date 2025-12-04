use serde::{Deserialize, Serialize};

use crate::shared::model::revision::RevisionModel;

#[derive(Debug, Deserialize, Serialize, Clone, PartialEq, Eq)]
pub struct RevisionDto {
    pub commit_hash: String,
    pub branch: String,
}
impl From<RevisionModel> for RevisionDto {
    fn from(
        RevisionModel {
            commit_hash,
            branch,
            ..
        }: RevisionModel,
    ) -> Self {
        Self {
            commit_hash,
            branch,
        }
    }
}

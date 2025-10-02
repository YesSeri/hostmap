use serde::{Deserialize, Serialize};

use crate::shared::model::revision::RevisionModel;

#[derive(Debug, Deserialize, Serialize, Clone, PartialEq, Eq)]
pub struct NixGitMappingDto {
    pub rev_id: String,
    pub branch: String,
}
impl From<NixGitMappingModel> for RevisionDto {
    fn from(RevisionModel { rev_id, branch, .. }: RevisionModel) -> Self {
        Self { rev_id, branch }
    }
}

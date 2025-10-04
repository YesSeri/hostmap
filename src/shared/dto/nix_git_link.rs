use serde::{Deserialize, Serialize};

use crate::shared::model::{nix_git_link::NixGitLinkModel, revision::RevisionModel};

#[derive(Debug, Deserialize, Serialize, Clone, PartialEq, Eq)]
pub struct NixGitLinkDto {
    pub nix_store_path: String,
    pub commit_hash: String,
    pub branch: String,
    pub deployed_at: chrono::DateTime<chrono::Utc>,
}
impl From<NixGitLinkModel> for NixGitLinkDto {
    fn from(
        NixGitLinkModel {
            nix_store_path,
            commit_hash,
            branch,
            deployed_at,
        }: NixGitLinkModel,
    ) -> Self {
        Self {
            nix_store_path,
            commit_hash,
            branch,
            deployed_at,
        }
    }
}

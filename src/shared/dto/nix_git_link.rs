use serde::{Deserialize, Serialize};

use crate::shared::{dto::revision::RevisionDto, model::nix_git_link::NixGitLinkModel};

#[derive(Debug, Deserialize, Serialize, Clone, PartialEq, Eq)]
pub struct NixGitLinkDto {
    pub nix_store_path: String,
    // pub commit_hash: String,
    // pub branch: String,
    pub revision: RevisionDto,
    pub deployed_at: chrono::DateTime<chrono::Utc>,
}
impl From<NixGitLinkModel> for NixGitLinkDto {
    fn from(
        NixGitLinkModel {
            nix_store_path,
            revision,
            deployed_at,
        }: NixGitLinkModel,
    ) -> Self {
        Self {
            nix_store_path,
            revision: revision.into(),
            deployed_at,
        }
    }
}

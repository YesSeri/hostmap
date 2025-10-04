use serde::{Deserialize, Serialize};

use crate::shared::{dto::nix_git_link::NixGitLinkDto, model::revision::RevisionModel};

#[derive(Debug, Deserialize, Serialize, Clone, PartialEq, Eq)]
pub struct NixGitLinkModel {
    pub nix_store_path: String,
    // pub commit_hash: String,
    // pub branch: String,
    pub revision: RevisionModel,
    pub deployed_at: chrono::DateTime<chrono::Utc>,
}
impl From<NixGitLinkDto> for NixGitLinkModel {
    fn from(
        NixGitLinkDto {
            nix_store_path,
            revision,
            deployed_at,
        }: NixGitLinkDto,
    ) -> Self {
        Self {
            nix_store_path,
            revision: revision.into(),
            deployed_at,
        }
    }
}

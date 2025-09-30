use serde::{Deserialize, Serialize};

use crate::shared::dto::nix_git_link::NixGitLinkDto;

#[derive(Debug, Deserialize, Serialize, Clone, PartialEq, Eq)]
pub struct NixGitLinkModel {
    pub nix_store_path: String,
    pub commit_hash: String,
    pub branch: String,
    pub deployed_at: chrono::DateTime<chrono::Utc>,
}
impl From<NixGitLinkDto> for NixGitLinkModel {
    fn from(
        NixGitLinkDto {
            nix_store_path,
            commit_hash,
            branch,
            deployed_at,
        }: NixGitLinkDto,
    ) -> Self {
        Self {
            nix_store_path,
            commit_hash,
            branch,
            deployed_at,
        }
    }
}

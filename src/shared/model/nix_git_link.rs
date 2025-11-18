use serde::{Deserialize, Serialize};

use crate::shared::{dto::nix_git_link::NixGitLinkDto, model::revision::RevisionModel};

#[derive(Debug, Deserialize, Serialize, Clone, PartialEq, Eq)]
pub struct NixGitLinkModel {
    pub nix_store_path: String,
    pub revision: RevisionModel,
    pub linked_at: chrono::DateTime<chrono::Utc>,
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
            linked_at: deployed_at,
        }
    }
}

use serde::{Deserialize, Serialize};

use crate::shared::model::revision::RevisionModel;

#[derive(Debug, Deserialize, Serialize, Clone, PartialEq, Eq)]
pub struct NixGitMappingDto {
    pub nix_store_path: String,
    pub rev_id: String,
}
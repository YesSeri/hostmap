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

#[derive(Debug, Clone, Deserialize, Serialize, PartialEq, Eq)]
pub struct StorePathDto {
    pub store_path: String,
    pub abbreviated_path: String,
}

impl From<String> for StorePathDto {
    fn from(store_path: String) -> Self {
        let abbreviated_path = Self::shorten_store_path(&store_path);
        Self {
            store_path,
            abbreviated_path,
        }
    }
}
impl StorePathDto {
    fn shorten_store_path(store_path: &str) -> String {
        if let Some(rest) = store_path.strip_prefix("/nix/store/") {
            if let Some(pos) = rest.find('-') {
                if rest.len() > pos + 1 {
                    rest[..pos].to_string()
                } else {
                    rest.to_string()
                }
            } else {
                rest.to_string()
            }
        } else if store_path == "/run/current-system" {
            "current-system".to_string()
        } else {
            store_path.to_owned()
        }
    }
}

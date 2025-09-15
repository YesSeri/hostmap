use serde::{Deserialize, Serialize};

use crate::{
    model::revision::{RevisionModel, StorePathModel},
    RetError,
};

#[derive(Debug, Deserialize, Serialize)]
pub struct RevisionDto {
    pub rev_id: String,
    pub branch: String,
}
impl From<RevisionModel> for RevisionDto {
    fn from(RevisionModel { rev_id, branch, .. }: RevisionModel) -> Self {
        Self { rev_id, branch }
    }
}

#[derive(Debug, Clone, Deserialize, Serialize, PartialEq)]
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
                if (rest.len() > pos + 1) {
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

#[derive(Debug)]
pub struct RevisionStorePathDto {
    pub revision: RevisionDto,
    pub store_path: StorePathDto,
}

// impl TryFrom<RevisionModel> for RevisionDto {
//     type Error = Box<RetError>;

//     fn try_from(value: RevisionModel) -> Result<Self, Self::Error> {
//         Ok(Self {
//             rev_id: value.rev_id,
//             branch: value.branch,
//         })
//     }
// }

// impl TryFrom<StorePathModel> for StorePathDto {
//     type Error = &'static str;

//     fn try_from(value: StorePathModel) -> Result<Self, Self::Error> {
//         let abbreviated_path = Self::shorten_store_path(&value.id);
//         Ok(Self {
//             store_path: value.id,
//             abbreviated_path
//         })
//     }
// }

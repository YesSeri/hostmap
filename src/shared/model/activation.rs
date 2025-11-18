use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};

use crate::shared::{
    dto::{
        activation::ActivationDto,
        host::{CurrentHostDto, HostWithLogsDto},
    },
    model::{host::HostModel, revision::RevisionModel},
};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ActivationCore {
    pub activated_at: DateTime<Utc>,
    pub username: String,
    pub store_path: String,
    pub activation_type: String,
    pub hostname: String,
    pub revision: Option<RevisionModel>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct NewActivation {
    pub core: ActivationCore,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Activation {
    pub id: i64,
    pub core: ActivationCore,
}

impl<T: HasHostname> From<(&T, ActivationDto)> for NewActivation {
    fn from((host, dto): (&T, ActivationDto)) -> Self {
        Self {
            core: ActivationCore {
                activated_at: dto.activated_at,
                username: dto.username,
                store_path: dto.store_path,
                activation_type: dto.activation_type,
                hostname: host.hostname().to_string(),
                revision: dto.revision.map(|r| RevisionModel {
                    commit_hash: r.commit_hash,
                    branch: r.branch,
                }),
            },
        }
    }
}

pub trait HasHostname {
    fn hostname(&self) -> &str;
}

impl HasHostname for HostModel {
    fn hostname(&self) -> &str {
        &self.hostname
    }
}
impl HasHostname for CurrentHostDto {
    fn hostname(&self) -> &str {
        &self.hostname
    }
}
impl HasHostname for HostWithLogsDto {
    fn hostname(&self) -> &str {
        &self.hostname
    }
}

#[derive(Debug, Clone)]
pub struct ActivationWithRevision {
    pub activation_id: i64,
    pub activated_at: DateTime<Utc>,
    pub username: String,
    pub store_path: String,
    pub activation_type: String,
    pub hostname: String,
    pub commit_hash: Option<String>,
    pub branch: Option<String>,
}

impl From<ActivationWithRevision> for Activation {
    fn from(e: ActivationWithRevision) -> Self {
        Self {
            id: e.activation_id,
            core: ActivationCore {
                activated_at: e.activated_at,
                username: e.username,
                store_path: e.store_path,
                activation_type: e.activation_type,
                hostname: e.hostname,
                revision: match (e.commit_hash, e.branch) {
                    (Some(r), Some(b)) => Some(RevisionModel {
                        commit_hash: r,
                        branch: b,
                    }),
                    _ => None,
                },
            },
        }
    }
}

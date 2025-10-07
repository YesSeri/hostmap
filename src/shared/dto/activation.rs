use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};

use crate::shared::{
    dto::revision::RevisionDto,
    model::activation::{Activation, ActivationCore, NewActivation},
};

#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct ActivationDto {
    pub activated_at: DateTime<Utc>,
    pub username: String,
    pub store_path: String,
    pub activation_type: String,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub revision: Option<RevisionDto>,
}
impl From<ActivationCore> for ActivationDto {
    fn from(core: ActivationCore) -> Self {
        Self {
            activated_at: core.activated_at,
            username: core.username,
            store_path: core.store_path,
            activation_type: core.activation_type,
            revision: core.revision.map(|r| r.into()),
        }
    }
}

impl From<Activation> for ActivationDto {
    fn from(l: Activation) -> Self {
        Self::from(l.core)
    }
}
impl From<NewActivation> for ActivationDto {
    fn from(l: NewActivation) -> Self {
        Self::from(l.core)
    }
}

use serde::{Deserialize, Serialize};
use std::collections::HashMap;

use crate::shared::{dto::host::CurrentHostDto, model::activation::Activation};

#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct HostModel {
    pub hostname: String,
    pub host_url: String,
    pub metadata: HashMap<String, String>,
}

impl From<CurrentHostDto> for HostModel {
    fn from(
        CurrentHostDto {
            hostname,
            host_url,
            metadata,
            ..
        }: CurrentHostDto,
    ) -> Self {
        Self {
            hostname,
            host_url,
            metadata,
        }
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct HostWithLatestLog {
    pub host: HostModel,
    pub logs: Option<Activation>,
}

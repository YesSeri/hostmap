use serde::{Deserialize, Serialize};

use crate::shared::dto::host::CurrentHostDto;

#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct HostModel {
    pub host_name: String,
    pub host_group_name: String,
    pub host_url: String,
}

impl From<CurrentHostDto> for HostModel {
    fn from(
        CurrentHostDto {
            host_name,
            host_group_name,
            host_url,
            ..
        }: CurrentHostDto,
    ) -> Self {
        Self {
            host_name,
            host_group_name,
            host_url,
        }
    }
}

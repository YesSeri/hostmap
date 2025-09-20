use serde::{Deserialize, Serialize};

use crate::{dto::host::CurrentHostDto, model::host_group::HostGroupModel};

#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct HostGroupDto {
    pub host_group_name: String,
    pub hosts: Vec<CurrentHostDto>,
}

#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct CreateHostGroupsDto(pub Vec<HostGroupDto>);

impl From<HostGroupModel> for HostGroupDto {
    fn from(
        HostGroupModel {
            host_group_name,
            host_models,
        }: HostGroupModel,
    ) -> Self {
        Self {
            host_group_name,
            hosts: host_models
                .into_iter()
                .map(|el| CurrentHostDto::from(el))
                .collect(),
        }
    }
}

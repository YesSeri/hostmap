use serde::Serialize;

use crate::{dto::host_group::HostGroupDto, model::host::HostModel};

#[derive(Debug, Clone, Serialize)]
pub struct HostGroupModel {
    pub host_group_name: String,
    pub host_models: Vec<HostModel>,
}

impl From<HostGroupDto> for HostGroupModel {
    fn from(
        HostGroupDto {
            host_group_name: group_name,
            hosts: host_dtos,
        }: HostGroupDto,
    ) -> Self {
        Self {
            host_group_name: group_name,
            host_models: host_dtos.into_iter().map(Into::into).collect(),
        }
    }
}

pub struct HostGroupName(pub String);

impl HostGroupName {
    pub fn new(v: String) -> Self {
        Self(v)
    }
}

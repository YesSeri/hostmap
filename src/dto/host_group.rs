use std::collections::HashMap;

use serde::{Deserialize, Serialize};

use crate::{
    dto::host::{CurrentHostDto, HostDto},
    model::host_group::HostGroupModel,
};

#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct CreateHostGroupDto {
    pub group_name: String,
    pub host_dtos: Vec<HostDto>,
}

#[derive(Debug, Clone, Serialize)]
pub struct CreateHostGroupsDto(pub Vec<CreateHostGroupDto>);

impl<'de> Deserialize<'de> for CreateHostGroupsDto {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        let map = HashMap::<String, Vec<HostDto>>::deserialize(deserializer)?;
        let mut groups = Vec::with_capacity(map.len());
        for (name, host_dtos) in map {
            groups.push(CreateHostGroupDto {
                group_name: name,
                host_dtos,
            });
        }
        Ok(CreateHostGroupsDto(groups))
    }
}

#[derive(Debug, Clone, Deserialize, Serialize, PartialEq)]
pub struct CurrentHostGroupDto {
    pub group_name: String,
    pub host_dtos: Vec<CurrentHostDto>,
}

impl From<(HostGroupModel, CurrentHostDto)> for CurrentHostGroupDto {
    fn from(
        (HostGroupModel { group_name, .. }, host_dto): (HostGroupModel, CurrentHostDto),
    ) -> Self {
        Self {
            group_name,
            host_dtos: vec![host_dto],
        }
    }
}

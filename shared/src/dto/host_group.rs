use std::collections::HashMap;

use serde::{Deserialize, Serialize};

use crate::{
    dto::host::{CreateHostDto, HostDto, HostWithLogDto}, model::host_group::HostGroupModel
};

#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct CreateHostGroupDto {
    pub group_name: String,
    pub host_dtos: Vec<CreateHostDto>,
}

#[derive(Debug, Clone, Serialize)]
pub struct CreateHostGroupsDto(pub Vec<CreateHostGroupDto>);

impl<'de> Deserialize<'de> for CreateHostGroupsDto {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        let map = HashMap::<String, Vec<CreateHostDto>>::deserialize(deserializer)?;
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

impl From<(HostGroupModel, CreateHostDto)> for CreateHostGroupDto {
    fn from(
        (HostGroupModel { group_name, .. }, host_dto): (HostGroupModel, CreateHostDto),
    ) -> Self {
        Self {
            group_name,
            host_dtos: vec![host_dto],
        }
    }
}




#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct HostGroupDto {
    pub group_name: String,
    pub host_group_id: i64,
    pub host_dtos: Vec<HostWithLogDto>,
}

impl From<(HostGroupModel, HostWithLogDto)> for HostGroupDto {
    fn from((HostGroupModel { group_name, host_group_id, .. }, host_dto): (HostGroupModel, HostWithLogDto)) -> Self {
        Self {
            group_name,
            host_group_id,
            host_dtos: vec![host_dto],
        }
    }
}
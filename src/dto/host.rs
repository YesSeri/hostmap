use std::collections::HashMap;

use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};

use crate::{
    model::log::LogEntryModel,
    repository::host_repository::{HostGroupModel, HostModel},
};

#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct HostDto {
    pub name: String,
    pub url: String,
}
impl From<HostModel> for HostDto {
    fn from(HostModel { name, url, .. }: HostModel) -> Self {
        Self { name, url }
    }
}

#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct HostGroupDto {
    pub group_name: String,
    pub host_dtos: Vec<HostDto>,
}

impl From<HostGroupModel> for HostGroupDto {
    fn from(
        HostGroupModel {
            group_name: name,
            hosts,
            ..
        }: HostGroupModel,
    ) -> Self {
        let host_dtos = hosts.into_iter().map(HostDto::from).collect();
        Self {
            group_name: name,
            host_dtos,
        }
    }
}

#[derive(Debug, Clone, Serialize)]
pub struct HostGroupsDto(pub Vec<HostGroupDto>);

impl<'de> Deserialize<'de> for HostGroupsDto {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        let map = HashMap::<String, Vec<HostDto>>::deserialize(deserializer)?;
        let mut groups = Vec::with_capacity(map.len());
        for (name, host_dtos) in map {
            groups.push(HostGroupDto {
                group_name: name,
                host_dtos,
            });
        }
        Ok(HostGroupsDto(groups))
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

#[derive(Debug, Clone, Deserialize, Serialize, PartialEq)]
pub struct CurrentHostDto {
    pub host_name: String,
    pub url: String,
    pub store_path: String,
    pub activation_type: String,
    pub timestamp: DateTime<Utc>,
}

impl From<(HostModel, LogEntryModel)> for CurrentHostDto {
    fn from(
        (
            HostModel {
                name: host_name,
                url,
                ..
            },
            LogEntryModel {
                store_path,
                activation_type,
                timestamp,
                ..
            },
        ): (HostModel, LogEntryModel),
    ) -> Self {
        Self {
            host_name,
            url,
            store_path,
            activation_type,
            timestamp,
        }
    }
}

use std::collections::HashMap;

use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};

use crate::model::{
    host::{ExistingHostGroupModel, ExistingHostModel, HostGroupModel, NewHostModel},
    log::{ExistingLogEntryModel, NewLogEntryModel},
};

#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct HostCreateDto {
    pub name: String,
    pub url: String,
}
impl From<NewHostModel> for HostCreateDto {
    fn from(NewHostModel { name, url, .. }: NewHostModel) -> Self {
        Self { name, url }
    }
}

#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct HostGroupCreateDto {
    pub group_name: String,
    pub host_dtos: Vec<HostCreateDto>,
}

#[derive(Debug, Clone, Serialize)]
pub struct HostGroupsCreateDto(pub Vec<HostGroupCreateDto>);

impl<'de> Deserialize<'de> for HostGroupsCreateDto {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        let map = HashMap::<String, Vec<HostCreateDto>>::deserialize(deserializer)?;
        let mut groups = Vec::with_capacity(map.len());
        for (name, host_dtos) in map {
            groups.push(HostGroupCreateDto {
                group_name: name,
                host_dtos,
            });
        }
        Ok(HostGroupsCreateDto(groups))
    }
}

#[derive(Debug, Clone, Deserialize, Serialize, PartialEq)]
pub struct CurrentHostGroupDto {
    pub group_name: String,
    pub host_dtos: Vec<CurrentHostDto>,
}

impl From<(ExistingHostGroupModel, CurrentHostDto)> for CurrentHostGroupDto {
    fn from(
        (ExistingHostGroupModel { group_name, .. }, host_dto): (
            ExistingHostGroupModel,
            CurrentHostDto,
        ),
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
    pub host_id: i64,
    pub url: String,
    pub store_path: String,
    pub activation_type: String,
    pub timestamp: DateTime<Utc>,
}

impl From<(ExistingHostModel, ExistingLogEntryModel)> for CurrentHostDto {
    fn from(
        (
            ExistingHostModel {
                name: host_name,
                url,
                host_id,
            },
            ExistingLogEntryModel {
                store_path,
                activation_type,
                timestamp,
                ..
            },
        ): (ExistingHostModel, ExistingLogEntryModel),
    ) -> Self {
        Self {
            host_name,
            url,
            host_id,
            store_path,
            activation_type,
            timestamp,
        }
    }
}

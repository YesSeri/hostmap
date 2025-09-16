use std::collections::HashMap;

use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};

use crate::{
    dto::revision::StorePathDto,
    model::{
        host::{ExistingHostGroupModel, ExistingHostModel, HostGroupModel, NewHostModel},
        log::{ExistingLogEntryModel, NewLogEntryModel},
        revision::RevisionModel,
    },
};

#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct HostDto {
    pub name: String,
    pub url: String,
}
impl From<ExistingHostModel> for HostDto {
    fn from(ExistingHostModel { name, url, .. }: ExistingHostModel) -> Self {
        Self { name, url }
    }
}

#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct HostGroupCreateDto {
    pub group_name: String,
    pub host_dtos: Vec<HostDto>,
}

#[derive(Debug, Clone, Serialize)]
pub struct HostGroupsCreateDto(pub Vec<HostGroupCreateDto>);

impl<'de> Deserialize<'de> for HostGroupsCreateDto {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        let map = HashMap::<String, Vec<HostDto>>::deserialize(deserializer)?;
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
pub(crate) struct CurrentHostDto {
    pub(crate) host_name: String,
    pub(crate) host_id: i64,
    pub(crate) url: String,
    pub(crate) store_path: StorePathDto,
    pub(crate) activation_type: String,
    pub(crate) timestamp: DateTime<Utc>,
    pub(crate) rev_id: Option<String>,
    pub(crate) branch: Option<String>,
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
                revision,
                ..
            },
        ): (ExistingHostModel, ExistingLogEntryModel),
    ) -> Self {
        let (rev_id, branch) = match revision {
            Some(r) => (Some(r.rev_id), Some(r.branch)),
            None => (None, None),
        };

        Self {
            host_name,
            url,
            host_id,
            store_path: StorePathDto::from(store_path),
            activation_type,
            timestamp,
            rev_id,
            branch,
        }
    }
}

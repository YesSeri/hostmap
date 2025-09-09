use serde::{Deserialize, Serialize};
use sqlx::{Pool, Postgres};

use crate::dto::host::{HostDto, HostGroupDto};

#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct HostModel {
    pub host_id: Option<i64>,
    pub name: String,
    pub url: String,
}

impl From<HostDto> for HostModel {
    fn from(HostDto { name, url }: HostDto) -> Self {
        Self {
            host_id: None,
            name,
            url,
        }
    }
}

#[derive(Debug, Clone, Serialize)]
pub struct HostGroupModel {
    pub host_group_id: Option<i64>,
    pub name: String,
    pub hosts: Vec<HostModel>,
}

impl From<HostGroupDto> for HostGroupModel {
    fn from(
        HostGroupDto {
            group_name: name,
            host_dtos,
        }: HostGroupDto,
    ) -> Self {
        let hosts = host_dtos.into_iter().map(HostModel::from).collect();
        Self {
            host_group_id: None,
            name,
            hosts,
        }
    }
}

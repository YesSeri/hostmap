use serde::{Deserialize, Serialize};
use sqlx::{Pool, Postgres};

use crate::dto::host::{HostCreateDto, HostGroupCreateDto};

#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct HostModel<IdType> {
    pub host_id: IdType,
    pub name: String,
    pub url: String,
}
pub(crate) type ExistingHostModel = HostModel<i64>;
pub(crate) type NewHostModel = HostModel<()>;

impl From<HostCreateDto> for NewHostModel {
    fn from(HostCreateDto { name, url }: HostCreateDto) -> Self {
        Self {
            host_id: (),
            name,
            url,
        }
    }
}

#[derive(Debug, Clone, Serialize)]
pub struct HostGroupModel<IdType> {
    pub host_group_id: IdType,
    pub group_name: String,
    pub hosts: Vec<HostModel<IdType>>,
}

pub(crate) type ExistingHostGroupModel = HostGroupModel<i64>;
pub(crate) type NewHostGroupModel = HostGroupModel<()>;

impl From<HostGroupCreateDto> for NewHostGroupModel {
    fn from(
        HostGroupCreateDto {
            group_name,
            host_dtos,
        }: HostGroupCreateDto,
    ) -> Self {
        let hosts = host_dtos
            .into_iter()
            .map(NewHostModel::from)
            .collect::<Vec<NewHostModel>>();
        Self {
            host_group_id: (),
            group_name,
            hosts,
        }
    }
}

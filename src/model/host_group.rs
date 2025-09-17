use serde::Serialize;

use crate::{
    dto::host_group::CreateHostGroupDto,
    model::host::{CreateHostModel, GenericHostModel},
};

#[derive(Debug, Clone, Serialize)]
pub struct GenericHostGroupModel<IdType> {
    pub host_group_id: IdType,
    pub group_name: String,
    pub hosts: Vec<GenericHostModel<IdType>>,
}

pub(crate) type HostGroupModel = GenericHostGroupModel<i64>;
pub(crate) type CreateHostGroupModel = GenericHostGroupModel<()>;

impl From<CreateHostGroupDto> for CreateHostGroupModel {
    fn from(
        CreateHostGroupDto {
            group_name,
            host_dtos,
        }: CreateHostGroupDto,
    ) -> Self {
        let hosts = host_dtos
            .into_iter()
            .map(CreateHostModel::from)
            .collect::<Vec<CreateHostModel>>();
        Self {
            host_group_id: (),
            group_name,
            hosts,
        }
    }
}

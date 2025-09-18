use serde::Serialize;

use crate::{
    dto::host_group::CreateHostGroupDto,
    model::host::GenericHostModel,
};

#[derive(Debug, Clone, Serialize)]
pub struct GenericHostGroupModel<IdType> {
    pub host_group_id: IdType,
    pub group_name: String,
    pub hosts: Vec<GenericHostModel<IdType>>,
}

pub type HostGroupModel = GenericHostGroupModel<i64>;
pub type CreateHostGroupModel = GenericHostGroupModel<()>;

impl From<CreateHostGroupDto> for CreateHostGroupModel {
    fn from(
        CreateHostGroupDto {
            group_name,
            host_dtos,
        }: CreateHostGroupDto,
    ) -> Self {
        Self {
            host_group_id: (),
            group_name,
            hosts: host_dtos.into_iter().map(Into::into).collect(),
        }
    }
}

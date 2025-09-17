use serde::{Deserialize, Serialize};

use crate::dto::host::HostDto;

#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct GenericHostModel<IdType> {
    pub host_id: IdType,
    pub name: String,
    pub url: String,
}
pub(crate) type HostModel = GenericHostModel<i64>;
pub(crate) type CreateHostModel = GenericHostModel<()>;

impl From<HostDto> for CreateHostModel {
    fn from(HostDto { name, url }: HostDto) -> Self {
        Self {
            host_id: (),
            name,
            url,
        }
    }
}

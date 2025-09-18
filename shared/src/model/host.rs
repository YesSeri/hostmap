use serde::{Deserialize, Serialize};

use crate::dto::host::CreateHostDto;

#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct GenericHostModel<IdType> {
    pub host_id: IdType,
    pub name: String,
    pub url: String,
}
pub type HostModel = GenericHostModel<i64>;
pub type CreateHostModel = GenericHostModel<()>;

impl From<CreateHostDto> for CreateHostModel {
    fn from(CreateHostDto { name, url }: CreateHostDto) -> Self {
        Self {
            host_id: (),
            name,
            url,
        }
    }
}

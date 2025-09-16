use serde::{Deserialize, Serialize};
use sqlx::{Pool, Postgres};

use crate::{
    dto::host::{HostDto, HostGroupCreateDto},
    model::{
        host::{ExistingHostGroupModel, ExistingHostModel, NewHostGroupModel},
        log::ExistingLogEntryModel,
    },
};

#[derive(Debug, Clone)]
pub struct RevisionRepository {
    pool: Pool<Postgres>,
}

impl RevisionRepository {
    pub fn new(pool: Pool<Postgres>) -> Self {
        Self { pool }
    }
}

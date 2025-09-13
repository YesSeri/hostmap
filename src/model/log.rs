use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use sqlx::prelude::FromRow;

use crate::dto::log::LogEntryDto;

#[derive(Debug, FromRow)]
pub(crate) struct LogEntryModel<IdType> {
    pub log_entry_id: IdType,
    pub timestamp: DateTime<Utc>,
    pub username: String,
    pub store_path: String,
    pub activation_type: String,
    pub host_id: i64,
}

pub(crate) type ExistingLogEntryModel = LogEntryModel<i64>;
pub(crate) type NewLogEntryModel = LogEntryModel<()>;

impl ExistingLogEntryModel {
    pub fn new(
        log_entry_id: i64,
        timestamp: DateTime<Utc>,
        username: String,
        store_path: String,
        activation_type: String,
        host_id: i64,
    ) -> Self {
        Self {
            log_entry_id,
            timestamp,
            username,
            store_path,
            activation_type,
            host_id,
        }
    }
}

impl NewLogEntryModel {
    pub fn new(
        timestamp: DateTime<Utc>,
        username: String,
        store_path: String,
        activation_type: String,
        host_id: i64,
    ) -> Self {
        Self {
            log_entry_id: (),
            timestamp,
            username,
            store_path,
            activation_type,
            host_id,
        }
    }
}

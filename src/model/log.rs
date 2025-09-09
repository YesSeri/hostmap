use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use sqlx::prelude::FromRow;

use crate::dto::log::LogEntryDto;

#[derive(Debug, FromRow)]
pub struct LogEntryModel {
    pub log_entry_id: i64,
    pub timestamp: DateTime<Utc>,
    pub username: String,
    pub store_path: String,
    pub activation_type: String,
    pub host_id: i64,
}

impl LogEntryModel {
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

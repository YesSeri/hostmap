use chrono::{DateTime, Utc};
use serde::Deserialize;

#[derive(Debug, Deserialize)]
pub struct LogRecord {
    #[serde(deserialize_with = "from_custom_fmt")]
    pub timestamp: DateTime<Utc>,
    pub username: String,
    pub store_path: String,
    pub activation_type: String,
}
fn from_custom_fmt<'de, D>(deserializer: D) -> Result<DateTime<Utc>, D::Error>
where
    D: serde::Deserializer<'de>,
{
    let s = String::deserialize(deserializer)?;
    let dt =
        DateTime::parse_from_str(&s, "%Y-%m-%d %H:%M:%S%:z").map_err(serde::de::Error::custom)?;
    Ok(dt.with_timezone(&Utc))
}

// enum ActivationType {
//     Boot,
//     Switch,
//     Test,
//     Rollback,
//     Build,
//     DryActivate,
//     BuildVm,
// }

// impl From<String> for ActivationType {
//     fn from(value: &str) -> Self {
//         use ActivationType::*;
//         match value {
//             "boot"=> Boot,
//             "switch"=>Switch
//             "test"=>Test
//             "rollback"=>Rollback
//             "build"=>Build
//             "dry-activate"=>DryActivate
//             "build-vm"=>BuildVm
//
//         }
//     }
// }

//	('boot'),
//	('switch'),
//	('test'),
//	('rollback'),
//	('build'),
//	('dry-activate'),
//	('build-vm');

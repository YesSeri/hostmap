use std::env;

use chrono::{DateTime, FixedOffset};
use serde::Deserialize;

#[derive(Debug, Deserialize)]
struct LogRecord {
    #[serde(deserialize_with = "from_custom_fmt")]
    timestamp: DateTime<FixedOffset>,
    username: String,
    store_path: String,
    activation_type: String,
}
fn from_custom_fmt<'de, D>(deserializer: D) -> Result<DateTime<FixedOffset>, D::Error>
where
    D: serde::Deserializer<'de>,
{
    let s = String::deserialize(deserializer)?;
    DateTime::parse_from_str(&s, "%Y-%m-%d %H:%M:%S%:z").map_err(serde::de::Error::custom)
}

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error + 'static>> {
    dotenvy::dotenv()?;
    let db_url = env::var("DATABASE_URL")?;
    dbg!(db_url);
    let body = reqwest::get("http://hosts-p01.pzz.dk/activationlog.csv")
        .await?
        .text()
        .await?;
    let mut rdr = csv::ReaderBuilder::new()
        .delimiter(b';')
        .has_headers(false)
        .from_reader(body.as_bytes());
    for line in rdr.deserialize() {
        let rec: LogRecord = line?;
        println!(
            "{};{};{};{}",
            rec.timestamp, rec.username, rec.store_path, rec.activation_type
        );
    }
    Ok(())
}

mod model;
use model::log_record::LogRecord;
use std::env;

use sqlx::{postgres::PgPoolOptions, PgPool};
use tokio::task::JoinSet;

type RetError = dyn std::error::Error + Send + Sync + 'static;

#[tokio::main]
async fn main() -> Result<(), Box<RetError>> {
    dotenvy::dotenv()?;
    let db_url = env::var("DATABASE_URL")?;
    dbg!(&db_url);
    let pool = PgPoolOptions::new()
        .max_connections(8)
        .connect(&db_url)
        .await
        .expect("failed to connect to DATABASE_URL");

    let urls = [
        "http://hosts-p01.pzz.dk/activationlog.csv",
        "http://hosts-p02.pzz.dk/activationlog.csv",
        "http://hosts-p03.pzz.dk/activationlog.csv",
    ];
    let mut set = JoinSet::new();
    for url in urls {
        let pool_cloned = pool.clone();
        set.spawn(async move {
            let recs = fetch_activationlog(url).await?;
            for rec in recs {
                match add_log_record(rec, &pool_cloned).await {
                    Ok(id) => println!("inserted a record with id {id}"),
                    Err(err) => println!("could not insert due to {err}"),
                }
            }
            Ok(())
        });
    }
    while let Some(res) = set.join_next().await {
        let res: Result<(), Box<RetError>> = res?;
        res?;
    }
    Ok(())
}

async fn add_log_record(rec: LogRecord, pool: &PgPool) -> Result<i32, Box<RetError>> {
    let rec = sqlx::query!(
        r#"
INSERT INTO log_entry ( timestamp, username, store_path, activation_type )
VALUES ( $1, $2, $3, $4 )
RETURNING log_entry_id
        "#,
        rec.timestamp,
        rec.username,
        rec.store_path,
        rec.activation_type,
    )
    .fetch_one(pool)
    .await?;
    println!("inserted: {rec:?}");

    Ok(rec.log_entry_id)
}

async fn fetch_activationlog(url: &str) -> Result<Vec<LogRecord>, Box<RetError>> {
    let body = reqwest::get(url).await?.text().await?;

    let mut rdr = csv::ReaderBuilder::new()
        .delimiter(b';')
        .has_headers(false)
        .from_reader(body.as_bytes());

    let mut log_records = Vec::new();
    for line in rdr.deserialize() {
        let rec: LogRecord = line.unwrap();
        log_records.push(rec);
    }
    Ok(log_records)
}

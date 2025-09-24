use crate::shared::{
    dto::{
        host::{CurrentHostDto, HostWithLogsDto},
        log::{LogEntryDto, LogHistoryDto},
    },
    model::{host::HostModel, log::NewLogEntryModel},
};
use reqwest::{Response, Url};
use serde_json::de;

fn create_client() -> Result<reqwest::Client, reqwest::Error> {
    let builder = reqwest::Client::builder().connect_timeout(std::time::Duration::from_secs(10));
    builder.build()
}

async fn fetch_activationlog(url: &Url) -> Result<Vec<LogEntryDto>, reqwest::Error> {
    let url = url.as_str();
    let client = create_client()?;
    let res = client.get(url).send().await?;
    let body = res.text().await?;

    let mut rdr = csv::ReaderBuilder::new()
        .delimiter(b';')
        .has_headers(false)
        .from_reader(body.as_bytes());

    let mut log_records = Vec::new();
    for line in rdr.deserialize() {
        let line = match line {
            Ok(line) => line,
            Err(err) => {
                tracing::warn!("could not parse line in csv from url: {url} because of {err}");
                continue;
            }
        };
        log_records.push(line);
    }
    Ok(log_records)
}

pub(crate) async fn insert_host_groups(
    host_group_dtos: &[CurrentHostDto],
) -> Result<(), reqwest::Error> {
    let client = create_client()?;
    client
        .post("http://localhost:3000/api/host_group/bulk")
        .json(&host_group_dtos)
        .send()
        .await?
        .error_for_status()?;
    Ok(())
}

pub async fn scrape_hosts(hosts: &[CurrentHostDto], timeout: u64) -> Result<(), reqwest::Error> {
    tracing::info!("running scraper from start");
    for host in hosts.iter() {
        tokio::time::sleep(std::time::Duration::from_secs(timeout)).await;
        let log_entry_models = scrape_host(host).await?;
        let dtos: Vec<LogHistoryDto> = log_entry_models
            .iter()
            .map(|model| LogHistoryDto::from(model.clone()))
            .collect();

        let host_with_logs_dto = HostWithLogsDto {
            hostname: host.hostname.clone(),
            host_url: host.host_url.clone(),
            metadata: host.metadata.clone(),
            logs: dtos,
        };
        let client = create_client().map_err(log_error)?;
        let res = client
            .post("http://localhost:3000/api/log_entry/bulk")
            .json(&host_with_logs_dto)
            .send()
            .await
            .map_err(log_error)?;
        tracing::info!("scraped host: {:?}", host);
    }
    Ok(())
}

fn log_error(err: reqwest::Error) -> reqwest::Error {
    tracing::error!("Error occurred: {}", err);
    err
}
async fn scrape_host(host: &CurrentHostDto) -> Result<Vec<NewLogEntryModel>, reqwest::Error> {
    let url_text = format!(
        "http://{}/activationlog.csv",
        host.host_url.trim_end_matches('/')
    )
    .to_owned();
    let url = Url::parse(&url_text).expect("could not parse url");
    let recs = fetch_activationlog(&url).await.map_err(log_error)?;
    tracing::debug!("records fetched from url {}: {:?}", url, recs);
    let host_model: HostModel = host.clone().into();
    let log_entry_models = recs
        .into_iter()
        .map(|dto| NewLogEntryModel::from((host_model.clone(), dto)))
        .collect::<Vec<NewLogEntryModel>>();

    Ok(log_entry_models)
}

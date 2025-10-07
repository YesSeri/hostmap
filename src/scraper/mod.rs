use std::{error, path::PathBuf};

use crate::{
    server::endpoint,
    shared::{
        dto::{
            activation::ActivationDto,
            host::{CurrentHostDto, HostWithLogsDto},
        },
        model::activation::NewActivation,
    },
};
use reqwest::{Client, Url, header};

pub async fn run(
    hosts_file: PathBuf,
    scrape_interval: u64,
    url: &str,
    api_key: String,
) -> Result<(), Box<dyn error::Error + Send + Sync + 'static>> {
    tracing::info!(
        "Starting scraper with file: {:?} and interval: {}",
        hosts_file,
        scrape_interval
    );
    let create_host_dtos = parse_hosts(&hosts_file);
    let mut headers = header::HeaderMap::new();
    headers.insert(
        header::AUTHORIZATION,
        header::HeaderValue::from_str("AUTHORIZATION_HEADER").expect("needs authrization header"),
    );
    let client = reqwest::Client::builder()
        .connect_timeout(std::time::Duration::from_secs(10))
        .default_headers(headers)
        .build()?;
    insert_hosts(&create_host_dtos, &client, url).await?;
    loop {
        tracing::info!("running background scraper");
        let create_host_dtos = create_host_dtos.clone();
        scrape_hosts(&create_host_dtos, scrape_interval, &client, url)
            .await
            .unwrap_or_else(|err| {
                tracing::info!("scraping failed due to {err:?}");
            });
    }
}

fn read_hosts_from_file(path: &PathBuf) -> String {
    std::fs::read_to_string(path).expect("could not read target list file")
}

fn parse_hosts(host_file: &PathBuf) -> Vec<CurrentHostDto> {
    let content = read_hosts_from_file(host_file);
    let host_dtos: Vec<CurrentHostDto> =
        serde_json::from_str(&content).expect("could not parse target list file as json. the metadata field must be a key-value pair. nested json is not supported");

    host_dtos
}

async fn fetch_activationlog(
    url: &Url,
    client: &Client,
) -> Result<Vec<ActivationDto>, reqwest::Error> {
    let url = url.as_str();
    let res = client.get(url).send().await?;
    let body = res.text().await?;

    let mut rdr = csv::ReaderBuilder::new()
        .delimiter(b';')
        .has_headers(false)
        .from_reader(body.as_bytes());

    let mut log_records = Vec::new();
    for line in rdr.deserialize() {
        let line: ActivationDto = match line {
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

pub(crate) async fn insert_hosts(
    host_dtos: &[CurrentHostDto],
    client: &Client,
    url: &str,
) -> Result<(), reqwest::Error> {
    let url = format!("{}{}", url, endpoint::hosts_bulk());
    client
        .post(url)
        .json(&host_dtos)
        .send()
        .await?
        .error_for_status()?;
    Ok(())
}

pub async fn scrape_hosts(
    hosts: &[CurrentHostDto],
    timeout: u64,
    client: &Client,
    url: &str,
) -> Result<(), reqwest::Error> {
    tracing::debug!("running scraper from start");
    for host in hosts.iter() {
        if let Err(e) = async {
            let new_activations = scrape_host(host, client).await?;
            let dtos: Vec<ActivationDto> = new_activations
                .into_iter()
                .map(ActivationDto::from)
                .collect();

            let body = HostWithLogsDto {
                hostname: host.hostname.clone(),
                host_url: host.host_url.clone(),
                logs: dtos,
                metadata: host.metadata.clone(),
            };

            let url = format!("{}{}", url, endpoint::activations_bulk());
            let res = client.post(url).json(&body).send().await?;
            res.error_for_status_ref()?;
            let res_text = res.text().await?;

            tracing::debug!(response_text=%res_text, request_host=%host.hostname);
            Ok::<(), reqwest::Error>(())
        }
        .await
        {
            tracing::warn!("scrape/post failed: {} when scraping host: {:?}", e, host);
        }
        tokio::time::sleep(std::time::Duration::from_secs(timeout)).await;
    }
    Ok(())
}

async fn scrape_host(
    host: &CurrentHostDto,
    client: &Client,
) -> Result<Vec<NewActivation>, reqwest::Error> {
    let url_text = format!(
        "http://{}/activationlog.csv",
        host.host_url.trim_end_matches('/')
    )
    .to_owned();
    let url = Url::parse(&url_text).expect("could not parse url");
    let recs = fetch_activationlog(&url, client).await?;
    // let host_model: HostModel = host.clone().into();
    let activation_models = recs
        .into_iter()
        .map(|dto| NewActivation::from((host, dto)))
        .collect::<Vec<NewActivation>>();

    Ok(activation_models)
}

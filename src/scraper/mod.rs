use futures::future::join_all;
use std::{error, path::PathBuf, time::Duration};

use crate::{
    cli::ScraperArgs,
    read_api_key,
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

pub async fn scrape_hosts_batched(
    hosts: &[CurrentHostDto],
    client: &Client,
    scraper_args: &ScraperArgs,
) -> Result<(), reqwest::Error> {
    let interval = Duration::from_secs(scraper_args.scrape_interval);
    let mut ticker = tokio::time::interval(interval);
    ticker.set_missed_tick_behavior(tokio::time::MissedTickBehavior::Burst);
    ticker.tick().await;

    tracing::debug!(
        "running scraper from start of list with {} hosts",
        hosts.len()
    );
    for (batch_idx, batch) in hosts.chunks(scraper_args.concurrent_requests).enumerate() {
        ticker.tick().await;

        let futs = batch.iter().map(|host| {
            let client = client.clone();
            let base_url = scraper_args.url.to_string();
            let hostname = host.hostname.clone();
            async move {
                let res = async {
                    let new_activations =
                        scrape_host(host, &client, scraper_args.activation_logger_port).await?;
                    let res_text =
                        insert_activations(host, new_activations, &client, &base_url).await?;
                    tracing::debug!(response_text=%res_text, request_host=%host.hostname);
                    Ok::<(), reqwest::Error>(())
                }
                .await;
                (hostname, res)
            }
        });

        let results = join_all(futs).await;
        let mut ok = 0;
        let mut fail = 0;

        for (host_id, res) in results {
            match res {
                Ok(_) => ok += 1,
                Err(e) => {
                    fail += 1;
                    tracing::debug!(host = %host_id, error = %e, "scrape attempt failed, skipping host");
                }
            }
        }
        // if fail > 0 {
        tracing::info!(
            batch_idx = batch_idx,
            ok = ok,
            fail = fail,
            "completed scraping batch"
        );
        /* } else {
            tracing::debug!(
                batch_idx = batch_idx,
                ok = ok,
                fail = fail,
                "completed scraping batch"
            );
        }
        */
    }

    Ok(())
}

pub async fn run(
    scraper_args: ScraperArgs,
) -> Result<(), Box<dyn error::Error + Send + Sync + 'static>> {
    // let ScraperArgs{ hosts_file, scrape_interval, concurrent_requests, activation_logger_port, api_key_file, url } = scraper_args;
    let api_key = read_api_key(&scraper_args.api_key_file);
    tracing::info!(
        "Starting scraper with file: {:?} and interval: {} and concurrent requests {}",
        scraper_args.hosts_file,
        scraper_args.scrape_interval,
        scraper_args.concurrent_requests,
    );
    let create_host_dtos = parse_hosts(&scraper_args.hosts_file);
    let mut headers = header::HeaderMap::new();
    headers.insert(
        header::AUTHORIZATION,
        header::HeaderValue::from_str(&format!("Api-Key {api_key}"))?,
    );
    let client = reqwest::Client::builder()
        .connect_timeout(std::time::Duration::from_secs(10))
        .default_headers(headers)
        .build()?;
    insert_hosts(&create_host_dtos, &client, &scraper_args.url).await?;
    let total_hosts = create_host_dtos.len();
    let batches =
        (total_hosts + scraper_args.concurrent_requests - 1) / scraper_args.concurrent_requests;

    tracing::info!(
        total_hosts = total_hosts,
        concurrent_requests = scraper_args.concurrent_requests,
        batches = batches,
        "configured scraper batching"
    );

    loop {
        scrape_hosts_batched(&create_host_dtos, &client, &scraper_args).await?;
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
    let mut parse_errors = 0;
    for (i, line) in rdr.deserialize().enumerate() {
        let line: ActivationDto = match line {
            Ok(line) => line,
            Err(err) => {
                parse_errors += 1;
                if parse_errors <= 5 {
                    tracing::debug!(
                        url = %url,
                        line_idx = i,
                        error = %err,
                        "failed to parse csv line; skipping"
                    );
                }
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

async fn insert_activations(
    host: &CurrentHostDto,
    activation_models: Vec<NewActivation>,
    client: &Client,
    url: &str,
) -> Result<String, reqwest::Error> {
    let activation_dtos: Vec<ActivationDto> = activation_models
        .into_iter()
        .map(ActivationDto::from)
        .collect();
    let body = HostWithLogsDto {
        hostname: host.hostname.clone(),
        host_url: host.host_url.clone(),
        logs: activation_dtos,
        metadata: host.metadata.clone(),
    };

    let url = format!("{}{}", url, endpoint::activations_bulk());
    let res = client.post(url).json(&body).send().await?;
    res.error_for_status_ref()?;
    let res_text = res.text().await?;
    Ok(res_text)
}

async fn scrape_host(
    host: &CurrentHostDto,
    client: &Client,
    activation_logger_port: usize,
) -> Result<Vec<NewActivation>, reqwest::Error> {
    let url_text = format!(
        "http://{}:{}/hostmap/hostmap-activation-logs.csv",
        host.host_url.trim_end_matches('/'),
        activation_logger_port,
    )
    .to_owned();

    tracing::debug!("scraping url: {}", url_text);
    let url = Url::parse(&url_text).expect("could not parse url");
    let recs = fetch_activationlog(&url, client).await?;
    // let host_model: HostModel = host.clone().into();
    let activation_models = recs
        .into_iter()
        .map(|dto| NewActivation::from((host, dto)))
        .collect::<Vec<NewActivation>>();

    Ok(activation_models)
}

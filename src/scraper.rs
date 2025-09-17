use reqwest::Url;

use crate::{
    dto::log::LogEntryDto,
    model::{host::ExistingHostModel, log::NewLogEntryModel},
    AppState, RetError,
};

async fn fetch_activationlog(url: &Url) -> Result<Vec<LogEntryDto>, Box<RetError>> {
    let url = url.as_str();
    let body = reqwest::get(url).await?.text().await?;
    let mut rdr = csv::ReaderBuilder::new()
        .delimiter(b';')
        .has_headers(false)
        .from_reader(body.as_bytes());

    let mut log_records = Vec::new();
    for line in rdr.deserialize() {
        let rec: LogEntryDto = line.unwrap();
        log_records.push(rec);
    }
    Ok(log_records)
}

pub async fn run_scraper(app_state: AppState) -> Result<(), Box<RetError>> {
    let host_groups = app_state.host_repo.get_all_host_groups().await?;
    for group in host_groups.into_iter() {
        for ExistingHostModel { host_id, url, .. } in group.hosts {
            tokio::time::sleep(std::time::Duration::from_secs(5)).await;
            let url_text =
                format!("http://{}/activationlog.csv", url.trim_end_matches('/')).to_owned();
            let url = Url::parse(&url_text).expect("could not parse url");
            let recs = fetch_activationlog(&url).await?;
            let log_entry_models = recs
                .into_iter()
                .map(|dto| NewLogEntryModel::from((dto, host_id)))
                .collect::<Vec<NewLogEntryModel>>();

            let res = app_state
                .activation_log_service
                .bulk_insert_log_records(&log_entry_models)
                .await;
            if let Err(e) = res {
                tracing::error!("error inserting log records for host_id {}: {}", host_id, e);
            }
        }
    }
    Ok(())
}

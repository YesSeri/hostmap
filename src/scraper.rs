use reqwest::Url;

use crate::{
    dto::log::LogEntryDto,
    model::{host::ExistingHostModel, log::NewLogEntryModel},
    AppState, RetError,
};

async fn fetch_activationlog(url: &Url, host_id: i64) -> Result<Vec<LogEntryDto>, Box<RetError>> {
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
        // println!("Processing group: {} with {} hosts", group.name, group.hosts.len());
        for ExistingHostModel { host_id, name, url } in group.hosts {
            let url_text =
                format!("http://{}/activationlog.csv", url.trim_end_matches('/')).to_owned();
            let url = Url::parse(&url_text).expect("could not parse url");
            let recs = fetch_activationlog(&url, host_id).await?;
            let log_entry_models = recs
                .into_iter()
                .map(
                    |LogEntryDto {
                         timestamp,
                         username,
                         store_path,
                         activation_type,
                     }| {
                        NewLogEntryModel::new(
                            timestamp,
                            username,
                            store_path,
                            activation_type,
                            host_id,
                        )
                    },
                )
                .collect::<Vec<NewLogEntryModel>>();
            let mut insertions = 0;
            for m in log_entry_models {
                match app_state.log_repo.add_log_record(&m).await {
                    Ok(id) => insertions += 1,
                    _ => (),
                }
            }
            // println!("inserted {} records for host {}", insertions, host.name)
        }
    }
    Ok(())
}

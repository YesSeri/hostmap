use reqwest::Url;
use shared::{dto::{host_group::{CreateHostGroupDto, CreateHostGroupsDto}, log::LogEntryDto}, model::{host::HostModel, host_group::{CreateHostGroupModel, HostGroupModel}, log::NewLogEntryModel}};

// use crate::{
//     AppState, RetError,
// };

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
        let rec: LogEntryDto = line.unwrap();
        log_records.push(rec);
    }
    Ok(log_records)
}

pub(crate) async fn insert_host_groups(host_group_dtos: CreateHostGroupsDto) -> Result<Vec<CreateHostGroupDto, reqwest::Error> {
    // for group in host_group_dtos.0.iter() {
    //     tracing::info!("inserting host group {} with hosts {}", group.group_name, group.host_dtos.len());
    //     let client = create_client()?;
    //     let res = client
    //         .post("http://localhost:3000/api/host_group/bulk")
    //         .json(&group)
    //         .send()
    //         .await?;

    // }
    Ok(())
}


pub async fn run_scraper(host_groups: Vec<HostGroupModel>) -> Result<(), reqwest::Error> {
    for group in host_groups.into_iter() {
        for HostModel { host_id, url, .. } in group.hosts {
            tokio::time::sleep(std::time::Duration::from_secs(5)).await;
            let url_text =
                format!("http://{}/activationlog.csv", url.trim_end_matches('/')).to_owned();
            let url = Url::parse(&url_text).expect("could not parse url");
            let recs = fetch_activationlog(&url).await?;
            let log_entry_models = recs
                .into_iter()
                .map(|dto| NewLogEntryModel::from((host_id, dto)))
                .collect::<Vec<NewLogEntryModel>>();

            //post request
            let client = create_client()?;
            let res = client
                .post("http://localhost:3000/api/log_entry/bulk")
                .json(&log_entry_models)
                .send()
                .await?;

            // let res = app_state
            //     .activation_log_service
            //     .bulk_insert_log_records(&log_entry_models)
            //     .await;
            // if let Err(e) = res {
            //     tracing::error!("error inserting log records for host_id {}: {}", host_id, e);
            // }
        }
    }
    Ok(())
}

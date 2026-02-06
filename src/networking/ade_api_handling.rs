use crate::{
    caching::cal_caching::{get_resource_from_cache_file, save_resource_to_cache_file},
    cli_params::*,
    utils::rooms::EnseirbRoom,
};
use chrono::{prelude::*, Duration};
use futures::{stream, StreamExt};
use reqwest::Url;
use std::sync::Arc;

const BATCH_SIZE: usize = 5;

pub async fn get_free_rooms_calendar_list(
    resource_ids: Arc<Vec<EnseirbRoom>>,
) -> Result<Vec<String>, Box<dyn std::error::Error>> {
    tracing::info!("Downloading...");
    let return_vec: Vec<String> = fetch_icals_from_urls(resource_ids.clone()).await?;
    tracing::info!("Download done!");

    Ok(return_vec)
}

fn get_time_interval() -> (String, String) {
    let timespan = Args::parse().free_rooms_timespan;
    let today = format!("{}", Local::now());
    let tomorrow = format!("{}", Local::now() + Duration::days(timespan as i64));
    let first_date = today.split(" ").collect::<Vec<&str>>()[0].to_string();
    let last_date = tomorrow.split(" ").collect::<Vec<&str>>()[0].to_string();

    (first_date, last_date)
}

async fn fetch_icals_from_urls(
    resources: Arc<Vec<EnseirbRoom>>,
) -> Result<Vec<String>, Box<dyn std::error::Error>> {
    let client = reqwest::Client::new();
    let resource_processing = |resource: &EnseirbRoom| {
        let client = client.clone();
        let res = resource.clone();
        tokio::spawn(async move {
            let id = match res.id() {
                Some(_id) => _id,
                None => 0,
            };
            if let Some(data) = get_resource_from_cache_file(id) {
                return Some(data);
            }

            let (first_date, last_date) = get_time_interval();
            let url: String = match res.url(first_date, last_date) {
                Some(_url) => _url,
                None => "".to_string(),
            };

            let the_url = match Url::parse(url.as_str()) {
                Ok(_url) => _url,
                Err(_) => {
                    return None;
                }
            };
            let resp = match client.get(the_url).send().await {
                Ok(_resp) => _resp,
                Err(_) => {
                    return None;
                }
            };
            let ical: String = match resp.text().await {
                Ok(_cal) => _cal,
                Err(_) => return None,
            };
            let _ = save_resource_to_cache_file(id, ical.clone());

            Some(ical)
        })
    };

    let thing = stream::iter(resources.iter())
        .map(resource_processing)
        .buffer_unordered(BATCH_SIZE);

    let fuck: Vec<Result<Option<String>, _>> = thing.collect().await;

    let retval: Vec<String> = fuck
        .iter()
        .filter_map(|x| match x {
            Ok(Some(_thing)) => Some(_thing.clone()),
            _ => None,
        })
        .collect();

    Ok(retval)
}

pub async fn get_zik_rooms() -> Result<Vec<EnseirbRoom>, reqwest::Error> {
    let resource_ids: Vec<EnseirbRoom> = vec![
        EnseirbRoom::TD01,
        EnseirbRoom::TD02,
        EnseirbRoom::TD03,
        EnseirbRoom::TD04,
        EnseirbRoom::TD05,
        EnseirbRoom::TD06,
        EnseirbRoom::TD07,
        EnseirbRoom::TD08,
        EnseirbRoom::TD09,
        EnseirbRoom::TD10,
        EnseirbRoom::TD11,
        EnseirbRoom::TD12,
        EnseirbRoom::TD13,
        EnseirbRoom::TD14,
        EnseirbRoom::TD15,
        EnseirbRoom::TD17,
        EnseirbRoom::TD20,
        EnseirbRoom::TD21,
        EnseirbRoom::TD22,
        EnseirbRoom::TD23,
        EnseirbRoom::TD24,
        EnseirbRoom::TD25,
        EnseirbRoom::TD26,
        EnseirbRoom::TD27,
        EnseirbRoom::TD28,
    ];

    Ok(resource_ids)
}

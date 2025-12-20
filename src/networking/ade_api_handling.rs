use crate::caching::cal_caching::{get_resource_from_cache_file, save_resource_to_cache_file};
use chrono::{prelude::*, Duration};
use futures::stream::{self, StreamExt};

// NOTE:     // room no to ADE id:
// TD01: 3224
// TD02: 3223
// TD03: 3222
// TD04: 3260
// TD05: 3259
// TD06: 3258
// TD07: 3254
// TD08: 3253
// TD09: 3252
// TD10: 3251
// TD11: 3250
// TD12: 3249
// TD13: 3248
// TD14: 3247
// TD15: 3280
// TD16:
// TD17: 3230
// TD18:
// TD19:
// TD20: 3296
// TD21: 3329
// TD22: 3330
// TD23: 3331
// TD24: 3327
// TD25: 3314
// TD26: 3315
// TD27: 3316
// TD28: 3318

pub async fn get_free_rooms_calendar_list() -> Result<Vec<String>, reqwest::Error> {
    let resource_ids: Vec<u16> = vec![
        3224, 3223, 3222, 3260, 3259, 3258, 3254, 3253, 3252, 3251, 3250, 3249, 3248, 3247, 3280,
        3230, 3296, 3329, 3330, 3331, 3327, 3314, 3315, 3316, 3318,
    ];

    let mut return_vec: Vec<String> = Vec::new();

    let mut stream = stream::iter(resource_ids);

    while let Some(cal) = stream.next().await {
        let ret_val = fetch_ical_from_url(cal).await?;
        return_vec.push(ret_val);
    }

    Ok(return_vec)
}

fn get_time_interval() -> (String, String) {
    let today = format!("{}", Local::now());
    let tomorrow = format!("{}", Local::now() + Duration::days(3));
    let first_date = today.split(" ").collect::<Vec<&str>>()[0].to_string();
    let last_date = tomorrow.split(" ").collect::<Vec<&str>>()[0].to_string();

    (first_date, last_date)
}

async fn fetch_ical_from_url(resource: u16) -> Result<String, reqwest::Error> {
    if let Some(data) = get_resource_from_cache_file(resource) {
        return Ok(data);
    }

    let (first_date, last_date) = get_time_interval();
    let url = format!("https://adeapp.bordeaux-inp.fr/jsp/custom/modules/plannings/anonymous_cal.jsp?resources={resource}&projectId=1&calType=ical&firstDate={first_date}&lastDate={last_date}&displayConfigId=71");
    let response = reqwest::get(url).await?;
    let ical = response.text().await?;

    let _ = save_resource_to_cache_file(resource, ical.clone()); // Pas de gestion d'erreur pour
    // l'instant

    Ok(ical)
}

pub async fn get_zik_rooms() -> Result<Vec<String>, reqwest::Error> {
    let resource_ids: Vec<u16> = vec![
        3224, 3223, 3222, 3260, 3259, 3258, 3254, 3253, 3252, 3251, 3250, 3249, 3248, 3247, 3280,
        3230, 3296, 3329, 3330, 3331, 3327, 3314, 3315, 3316, 3318,
    ];

    let mut return_vec: Vec<String> = Vec::new();

    let mut stream = stream::iter(resource_ids);

    while let Some(cal) = stream.next().await {
        let ret_val = fetch_ical_from_url(cal).await?;
        return_vec.push(ret_val);
    }

    Ok(return_vec)
}

mod tests {
    use crate::networking::ade_api_handling::*;

    #[test]
    fn time_interval_test() {
        let today = format!("{}", Local::now());
        let tomorrow = format!("{}", Local::now() + Duration::days(3));
        let first_date = today.split(" ").collect::<Vec<&str>>()[0].to_string();
        let last_date = tomorrow.split(" ").collect::<Vec<&str>>()[0].to_string();

        assert!(get_time_interval() == (first_date, last_date));
    }

    #[tokio::test]
    async fn get_ical_from_url_test1() {
        let res = fetch_ical_from_url(0xffff).await;
        if let Err(_result1) = res {
            panic!();
        } else if let Ok(res1) = res {
            let expected = "BEGIN:VCALENDAR\r\nMETHOD:REQUEST\r\nPRODID:-//ADE/version 6.0\r\nVERSION:2.0\r\nCALSCALE:GREGORIAN\r\nEND:VCALENDAR\r\n";
            if expected != res1 {
                panic!();
            }
        };
    }

    #[tokio::test]
    async fn get_ical_from_url_test2() {
        if let Err(_result2) = fetch_ical_from_url(3224).await {
            panic!();
        };
    }
}

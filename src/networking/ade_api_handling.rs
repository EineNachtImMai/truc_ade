use chrono::{prelude::*, Duration};
use futures::stream::{self, StreamExt};

pub async fn get_calendar_list() -> Vec<String> {
    let resource_ids: Vec<u16> = vec![
        3224, 3223, 3222, 3260, 3259, 3258, 3254, 3253, 3252, 3251, 3250, 3249, 3248, 3247, 3280,
        3230, 3296, 3329, 3330, 3331, 3327, 3314, 3315, 3316, 3318,
    ];

    let mut return_vec: Vec<String> = Vec::new();

    let mut stream = stream::iter(resource_ids);

    while let Some(cal) = stream.next().await {
        let ret_val = fetch_ical_from_url(cal).await;
        return_vec.push(ret_val);
    }

    return_vec

    // room no to ADE id:
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
}

fn get_time_interval() -> (String, String) {
    let today = format!("{}", Local::now());
    let tomorrow = format!("{}", Local::now() + Duration::days(3));
    let first_date = today.split(" ").collect::<Vec<&str>>()[0].to_string();
    let last_date = tomorrow.split(" ").collect::<Vec<&str>>()[0].to_string();

    (first_date, last_date)
}

async fn fetch_ical_from_url(resource: u16) -> String {
    let (first_date, last_date) = get_time_interval();
    let url = format!("https://adeapp.bordeaux-inp.fr/jsp/custom/modules/plannings/anonymous_cal.jsp?resources={resource}&projectId=1&calType=ical&firstDate={first_date}&lastDate={last_date}&displayConfigId=71");
    // let response = reqwest::blocking::get(url).unwrap();
    let response = reqwest::get(url).await.unwrap();
    let ical = response.text().await.unwrap();

    ical
}

use chrono::{Duration, prelude::*};
use icalendar::Calendar;
use std::sync::Arc;
use std::{collections::HashMap, error::Error, fs, io::BufReader};

use crate::utils::rooms::EnseirbRoom;

// try to save the data to the file
// if successful, update the resource last updated time
// otherwise, return an error
pub fn save_resource_to_cache_file(res_id: u16, data: String) -> Result<(), Box<dyn Error>> {
    let file_name = format!("cache/{res_id}.ics");
    fs::write(file_name, data)?;

    // keep this after the write, better to think it's outdated when it's not than the contrary
    update_resource_last_update_time(res_id)?;
    Ok(())
}

// only returns the resource if it was updated less than 60 minutes ago
pub fn get_resource_from_cache_file(res_id: u16) -> Option<String> {
    let current_time: DateTime<Utc> = Utc::now();
    let update_time: DateTime<Utc> = match get_resource_last_update_time(res_id) {
        Ok(_update_time) => _update_time,
        Err(_) => {
            tracing::warn!("Error: failed to get last update time of resource.");
            DateTime::from_timestamp(0, 0)?
        }
    }; // default to Jan 1 1970, which SHOULD be longer ago than whatever max time we set

    let ret_val;

    if current_time - update_time <= Duration::minutes(60) {
        let file_name = format!("cache/{res_id}.ics");
        match fs::read_to_string(file_name) {
            Ok(_str) => ret_val = Some(_str),
            Err(_) => ret_val = None,
        }
    } else {
        ret_val = None;
    }

    ret_val
}

pub fn init_resource_last_update_time() -> Result<(), Box<dyn Error>> {
    match fs::create_dir("cache") {
        Ok(_) => {}
        Err(e) => match e.kind() {
            // Fail on any error except already exists, bc the dir probably already does exist
            std::io::ErrorKind::AlreadyExists => {
                tracing::info!("Cache directory already exists.")
            }
            _ => {
                Err(e)?;
            }
        },
    };
    let mut hm = HashMap::new();
    let res_ids: [u16; 25] = [
        // FIX: oh no, magic numbers
        3224, 3223, 3222, 3260, 3259, 3258, 3254, 3253, 3252, 3251, 3250, 3249, 3248, 3247, 3280,
        3230, 3296, 3329, 3330, 3331, 3327, 3314, 3315, 3316, 3318,
    ];
    for res_id in res_ids {
        hm.insert(res_id, 0i64);
    }

    let file = fs::File::create("cache/update_times.json")?;

    Ok(serde_json::to_writer(file, &hm)?)
}

fn get_resource_last_update_time(res_id: u16) -> Result<DateTime<Utc>, Box<dyn Error>> {
    let file = fs::File::open("cache/update_times.json")?;
    let reader = BufReader::new(file);

    let data: HashMap<u16, i64> = serde_json::from_reader(reader)?;

    let date = data.get(&res_id).unwrap_or(&0).to_owned();

    match DateTime::from_timestamp(date, 0) {
        Some(_ret_val) => Ok(_ret_val),
        None => Err("Failed to transform timestamp to date".to_string())?,
    }
}

fn update_resource_last_update_time(res_id: u16) -> Result<(), Box<dyn Error>> {
    let file = fs::File::open("cache/update_times.json")?;
    let reader = BufReader::new(file);

    let mut data: HashMap<u16, i64> = serde_json::from_reader(reader)?;

    data.insert(res_id, Utc::now().timestamp());

    let file = fs::File::create("cache/update_times.json")?;

    Ok(serde_json::to_writer(file, &data)?)
}

// returns cached calendar if it exists and is recent enough, otherwise returns none
pub fn get_cached_free_rooms_cal(cal_list: Arc<Vec<EnseirbRoom>>) -> Option<Calendar> {
    let current_time: DateTime<Utc> = Utc::now();
    let update_time: DateTime<Utc> = match get_cal_last_update_time(cal_list.clone()) {
        Ok(_update_time) => _update_time,
        Err(_) => {
            tracing::warn!("Error: failed to get last update time of resource.");
            DateTime::from_timestamp(0, 0)?
        }
    }; // default to Jan 1 1970, which SHOULD be longer ago than whatever max time we set

    let ret_val;

    if current_time - update_time <= Duration::minutes(60) {
        let file_name = format!("cache/{}.ics", room_list_to_filename(cal_list));
        match fs::read_to_string(file_name) {
            Ok(_str) => ret_val = _str,
            Err(_) => return None,
        }
    } else {
        return None;
    }

    ret_val.parse::<Calendar>().ok()
}

pub fn cache_free_rooms_cal(
    cal_list: Arc<Vec<EnseirbRoom>>,
    value: &Calendar,
) -> Result<(), Box<dyn Error>> {
    let file_name = format!("cache/{}.ics", room_list_to_filename(cal_list.clone()));
    let data = format!("{}", value);
    fs::write(file_name, data)?;

    update_cal_last_update_time(cal_list)?;
    Ok(())
}

pub fn init_cal_last_update_time() -> Result<(), Box<dyn Error>> {
    match fs::create_dir("cache") {
        Ok(_) => {}
        Err(e) => match e.kind() {
            // Fail on any error except already exists, bc the dir probably already does exist
            std::io::ErrorKind::AlreadyExists => {
                tracing::info!("Cache directory already exists.")
            }
            _ => {
                Err(e)?;
            }
        },
    };
    let hm: HashMap<String, i64> = HashMap::new();

    let file = fs::File::create("cache/cal_update_times.json")?;

    Ok(serde_json::to_writer(file, &hm)?)
}

fn update_cal_last_update_time(res_id: Arc<Vec<EnseirbRoom>>) -> Result<(), Box<dyn Error>> {
    let file = fs::File::open("cache/cal_update_times.json")?;
    let reader = BufReader::new(file);

    let mut data: HashMap<String, i64> = serde_json::from_reader(reader)?;

    data.insert(room_list_to_filename(res_id), Utc::now().timestamp());

    let file = fs::File::create("cache/cal_update_times.json")?;

    Ok(serde_json::to_writer(file, &data)?)
}

fn room_list_to_filename(res_id: Arc<Vec<EnseirbRoom>>) -> String {
    res_id
        .iter()
        .map(|x| x.short_name())
        .collect::<Vec<String>>()
        .join("")
}

fn get_cal_last_update_time(
    res_id: Arc<Vec<EnseirbRoom>>,
) -> Result<DateTime<Utc>, Box<dyn Error>> {
    let file = fs::File::open("cache/cal_update_times.json")?;
    let reader = BufReader::new(file);

    let data: HashMap<String, i64> = serde_json::from_reader(reader)?;

    let date = data
        .get(room_list_to_filename(res_id).as_str())
        .unwrap_or(&0)
        .to_owned();

    match DateTime::from_timestamp(date, 0) {
        Some(_ret_val) => Ok(_ret_val),
        None => Err("Failed to transform timestamp to date".to_string())?,
    }
}

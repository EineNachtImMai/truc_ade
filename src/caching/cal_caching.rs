use chrono::{prelude::*, Duration};
use std::{collections::HashMap, error::Error, fs, io::BufReader};

// try to save the data to the file
// if successful, update the resource last updated time
// otherwise, return an error
pub fn save_resource_to_cache_file(res_id: u16, data: String) -> Result<(), Box<dyn Error>> {
    let file_name = format!("cache/{res_id}.ics");
    fs::write(file_name, data)?;

    update_resource_last_update_time(res_id)?;
    Ok(())
}

// only returns the resource if it was updated less than ??? minutes ago
pub fn get_resource_from_cache_file(res_id: u16) -> Option<String> {
    let current_time: DateTime<Utc> = Utc::now();
    let update_time: DateTime<Utc> = match get_resource_last_update_time(res_id) {
        Ok(_update_time) => _update_time,
        Err(_) => DateTime::from_timestamp(0, 0)?,
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
            std::io::ErrorKind::AlreadyExists => {}
            _ => {
                return Err(e)?;
            }
        },
    };
    let mut hm = HashMap::new();
    let res_ids: [u16; 25] = [
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

    let date = match data.get(&res_id) {
        Some(_date) => _date,
        None => &0,
    }
    .to_owned();

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

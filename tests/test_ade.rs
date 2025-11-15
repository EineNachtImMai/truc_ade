use chrono::{prelude::*, Duration};

use crate::networking::ade_api_handling::{get_time_interval};

#[test]
fn time_interval_test() {
    let today = format!("{}", Local::now());
    let tomorrow = format!("{}", Local::now() + Duration::days(3));
    let first_date = today.split(" ").collect::<Vec<&str>>()[0].to_string();
    let last_date = tomorrow.split(" ").collect::<Vec<&str>>()[0].to_string();

    assert!(get_time_interval() == (first_date, last_date));
}

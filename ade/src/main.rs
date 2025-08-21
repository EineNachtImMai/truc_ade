use std::fs::read_to_string;

use chrono::prelude::*;
use icalendar::{Calendar, CalendarComponent, Component};

fn main() {
    let contents = read_to_string("../ade_request_test_ICAL.ics").unwrap();

    let parsed_calendar: Calendar = contents.parse().unwrap();

    for component in &parsed_calendar.components {
        if let CalendarComponent::Event(event) = component {
            if let DateTime(Utc(time)) = event.get_end().unwrap() {
            println!(
                "Start: {:?}, End: {:?}, Now: {:?}",
                event.get_start().unwrap(),
                event.get_end().unwrap(),
                Local::now()
            )
            }
        }
    }
}

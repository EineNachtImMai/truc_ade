use chrono::{prelude::*, Duration};
use icalendar::{
    Calendar, CalendarComponent, CalendarDateTime, Component, DatePerhapsTime, Event, EventLike,
    Property,
};
use itertools::Itertools;

use std::sync::Arc;

fn parse_cal_to_cut_times(cal: Calendar) -> Vec<DateTime<Utc>> {
    let mut cut_times: Vec<DateTime<Utc>> = Vec::new();

    for component in &cal.components {
        if let CalendarComponent::Event(event) = component {
            if let DatePerhapsTime::DateTime(CalendarDateTime::Utc(time)) =
                event.get_start().unwrap()
            {
                cut_times.push(time);
            };

            if let DatePerhapsTime::DateTime(CalendarDateTime::Utc(time)) = event.get_end().unwrap()
            {
                cut_times.push(time);
            }
        }
    }

    cut_times
}

fn get_cut_times(calendar_list: Arc<Vec<String>>) -> Vec<DateTime<Utc>> {
    let mut cut_times: Vec<DateTime<Utc>> = Vec::new();

    for calendar_file in calendar_list.iter() {
        let cal: Calendar = calendar_file.parse().unwrap();
        cut_times.extend(parse_cal_to_cut_times(cal));
    }

    // sort the times and remove duplicate times
    cut_times.sort();
    cut_times.dedup();

    // remove chunks under 20 minutes
    let first_elem = cut_times[0];

    let pair_cut_times: Vec<(DateTime<Utc>, DateTime<Utc>)> =
        cut_times.clone().into_iter().tuple_windows().collect();

    for (start, end) in pair_cut_times {
        if end - start <= Duration::minutes(20) {
            if start == first_elem {
                cut_times.retain(|value| value != &end);
            } else {
                cut_times.retain(|value| value != &start);
            }
        }
    }

    cut_times
}

fn get_free_rooms(
    start_time: &DateTime<Utc>,
    end_time: &DateTime<Utc>,
    calendar_list: Arc<Vec<String>>,
) -> String {
    let mut free_rooms: Vec<&str> = vec![
        "EA-S106/S107 (TD06)",
        "EA-S108/S109 (TD07)",
        "EA-S008/S009 (TD17)",
        "EA-S101/S102 (TD04)",
        "EA-S104/S105 (TD05)",
        "EA-S110/S111 (TD08)",
        "EA-S112/S113 (TD09)",
        "EA-S114 (TD10)",
        "EA-S115/S116 (TD11)",
        "EA-S117/S118 (TD12)",
        "EA-S119/S120 (TD13)",
        "EA-S121/S122 (TD14)",
        "EA-S225 (TD15)",
        "EB-P010/P011 (TD20)",
        "EB-P117 (TD21)",
        "EB-P118/P119 (TD22)",
        "EB-P121 (TD23)",
        "EB-P123 (TD24)",
        "EB-P145 (TD25)",
        "EB-P147 (TD26)",
        "EB-P148/P150 (TD27)",
        "EB-P153/P156 (TD28)",
    ];

    for calendar_file in calendar_list.iter() {
        let cal: Calendar = calendar_file.parse().unwrap();

        for component in &cal.components {
            let start: DateTime<Utc>;
            let end: DateTime<Utc>;
            let event: &Event;

            if let CalendarComponent::Event(evt) = component {
                event = evt;
            } else {
                continue;
            }

            if let DatePerhapsTime::DateTime(CalendarDateTime::Utc(event_start_time)) =
                event.get_start().unwrap()
            {
                start = event_start_time;
            } else {
                continue;
            }

            if let DatePerhapsTime::DateTime(CalendarDateTime::Utc(event_end_time)) =
                event.get_end().unwrap()
            {
                end = event_end_time;
            } else {
                continue;
            }

            if (&start <= start_time && start_time < &end)
                || (&start < end_time && end_time <= &end)
            {
                free_rooms.retain(|value| *value != event.get_location().unwrap());
            }
        }
    }

    free_rooms.join(", ")
}

fn init_ade_cal() -> Calendar {
    let mut cal: Calendar = Calendar::empty();

    cal.append_property(Property::new("METHOD", "REQUEST"));
    cal.append_property(Property::new("PRODID", "-//ADE/version 6.0"));
    cal.append_property(Property::new("VERSION", "2.0"));
    cal.append_property(Property::new("CALSCALE", "GREGORIAN"));

    cal
}

pub fn get_free_roooms_calendar(calendar_list: Arc<Vec<String>>) -> Calendar {
    let mut cal = init_ade_cal();

    let cut_times: Vec<(DateTime<Utc>, DateTime<Utc>)> = get_cut_times(calendar_list.clone())
        .into_iter()
        .tuple_windows()
        .collect();

    for (start_time, end_time) in cut_times.iter() {
        let free_rooms = get_free_rooms(start_time, end_time, calendar_list.clone());
        let start = DatePerhapsTime::DateTime(CalendarDateTime::Utc(start_time.clone()));
        let end = DatePerhapsTime::DateTime(CalendarDateTime::Utc(end_time.clone()));
        cal.push(
            Event::new()
                .description("Salles Libres:")
                .location(&free_rooms)
                .starts(start)
                .ends(end)
                .summary("Salles Libres")
                .last_modified(Local::now().into())
                .created(DateTime::from_timestamp_nanos(0))
                .sequence(2141946518),
        );
    }

    cal.done()
}

pub fn get_zik_calendar(room_list: Arc<Vec<String>>) -> Calendar {
    let mut cal = init_ade_cal();

    cal.done()
}

use chrono::prelude::*;
use icalendar::{
    Calendar, CalendarComponent, CalendarDateTime, Component, DatePerhapsTime, Event, EventLike,
    Property,
};
use itertools::Itertools;

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

fn get_cut_times(calendar_list: Vec<String>) -> Vec<DateTime<Utc>> {
    let mut cut_times: Vec<DateTime<Utc>> = Vec::new();

    for calendar_file in calendar_list.iter() {
        let cal: Calendar = calendar_file.parse().unwrap();
        cut_times.extend(parse_cal_to_cut_times(cal));
    }

    // sort the times and remove duplicate times
    cut_times.sort();
    cut_times.dedup();

    cut_times
}

fn get_free_rooms(start_time: &DateTime<Utc>, calendar_list: Vec<String>) -> String {
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
            if let CalendarComponent::Event(event) = component {
                if let DatePerhapsTime::DateTime(CalendarDateTime::Utc(event_start_time)) =
                    event.get_start().unwrap()
                {
                    if let DatePerhapsTime::DateTime(CalendarDateTime::Utc(event_end_time)) =
                        event.get_end().unwrap()
                    {
                        if &event_start_time <= start_time && start_time < &event_end_time {
                            free_rooms.retain(|value| *value != event.get_location().unwrap());
                        }
                    }
                }
            }
        }
    }

    free_rooms.join(", ")
}

pub fn get_calendar(calendar_list: Vec<String>) -> Calendar {
    let mut cal: Calendar = Calendar::empty();

    cal.append_property(Property::new("METHOD", "REQUEST"));
    cal.append_property(Property::new("PRODID", "-//ADE/version 6.0"));
    cal.append_property(Property::new("VERSION", "2.0"));
    cal.append_property(Property::new("CALSCALE", "GREGORIAN"));

    let cut_times: Vec<(DateTime<Utc>, DateTime<Utc>)> = get_cut_times(calendar_list.clone())
        .into_iter()
        .tuple_windows()
        .collect();

    for (start_time, end_time) in cut_times.iter() {
        let free_rooms = get_free_rooms(start_time, calendar_list.clone());
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

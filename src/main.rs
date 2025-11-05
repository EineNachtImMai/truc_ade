use std::{
    io::{prelude::*, BufReader, Write},
    net::{TcpListener, TcpStream},
};

use itertools::Itertools;

use chrono::{prelude::*, Duration};
use icalendar::{
    Calendar, CalendarComponent, CalendarDateTime, Component, DatePerhapsTime, Event, EventLike,
    Property,
};

// NOTE: The ADE cal goes from 6h to 21h

fn main() {
    serve(get_calendar_list());
}

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

fn get_calendar_list() -> Vec<String> {
    let resource_ids: Vec<u16> = vec![
        3224, 3223, 3222, 3260, 3259, 3258, 3254, 3253, 3252, 3251, 3250, 3249, 3248, 3247, 3280,
        3230, 3296, 3329, 3330, 3331, 3327, 3314, 3315, 3316, 3318,
    ];

    resource_ids
        .iter()
        .map(|resource_id| fetch_ical_from_url(*resource_id))
        .collect()

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

fn fetch_ical_from_url(resource: u16) -> String {
    let (first_date, last_date) = get_time_interval();
    let url = format!("https://adeapp.bordeaux-inp.fr/jsp/custom/modules/plannings/anonymous_cal.jsp?resources={resource}&projectId=1&calType=ical&firstDate={first_date}&lastDate={last_date}&displayConfigId=71");
    let response = reqwest::blocking::get(url).unwrap();
    let ical = response.text().unwrap();

    ical
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

// TODO: make this function
fn get_calendar(calendar_list: Vec<String>) -> Calendar {
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

fn serve(calendar_list: Vec<String>) {
    let listener = TcpListener::bind("0.0.0.0:7878").unwrap();

    for stream in listener.incoming() {
        // NOTE: debug purposes, remove in prod
        println!["Got a connection!"];
        let stream = stream.unwrap();

        handle_connection(stream, calendar_list.clone());
    }
}

fn handle_connection(mut stream: TcpStream, calendar_list: Vec<String>) {
    let buf_reader = BufReader::new(&stream);
    let _http_request: Vec<_> = buf_reader
        .lines()
        .map(|result| result.unwrap())
        .take_while(|line| !line.is_empty())
        .collect();

    println!["{:?}", _http_request];

    let content = format!["{}", get_calendar(calendar_list)];
    let length = content.len();

    let response = format!["HTTP/1.1 200 OK\r\nServer: nginx\r\nConnection: keep-alive\r\nSet-Cookie: JSESSIONID=B993644BFDE8DC479AD092529DAB0BC6; Path=/jsp; Secure; HttpOnly\r\nStrict-Transport-Security: max-age=31536000;includeSubDomains\r\nX-Frame-Options: SAMEORIGIN\r\nX-Content-Type-Options: nosniff\r\nCache-Control: no-cache\r\nPragma: no-cache\r\nExpires: 0\r\nContent-Type: text/calendar;charset=UTF-8\r\nContent-Length: {length}\r\nContent-Disposition: inline; filename=ADECal.ics\r\n\r\n{content}"];

    stream.write_all(response.as_bytes()).unwrap();
}

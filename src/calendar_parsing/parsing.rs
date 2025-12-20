use chrono::{prelude::*, Duration};
use icalendar::{
    Calendar, CalendarComponent, CalendarDateTime, Component, DatePerhapsTime, Event, EventLike,
    Property,
};

use itertools::Itertools;

use std::sync::Arc;

pub enum WindowPosition {
    Open,
    Closed,
}

pub enum AllowedActivities {
    QuietPlaying(WindowPosition),
    LoudPlaying(WindowPosition),
    LoudPlayingAndBattery(WindowPosition),
}

impl AllowedActivities {
    fn enum_index(&self) -> u8 {
        match *self {
            AllowedActivities::QuietPlaying(WindowPosition::Closed) => 1,
            AllowedActivities::QuietPlaying(WindowPosition::Open) => 2,
            AllowedActivities::LoudPlaying(WindowPosition::Closed) => 3,
            AllowedActivities::LoudPlaying(WindowPosition::Open) => 4,
            AllowedActivities::LoudPlayingAndBattery(WindowPosition::Closed) => 5,
            AllowedActivities::LoudPlayingAndBattery(WindowPosition::Open) => 6,
        }
    }
}
impl Ord for AllowedActivities {
    fn cmp(&self, other: &Self) -> std::cmp::Ordering {
        self.enum_index().cmp(&other.enum_index())
    }
}
impl PartialOrd for AllowedActivities {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        Some(self.cmp(&other))
    }
}
impl PartialEq for AllowedActivities {
    fn eq(&self, other: &Self) -> bool {
        self.enum_index() == other.enum_index()
    }
}
impl Eq for AllowedActivities {}

// ------------------------------------------------------------------------------------------------
// FUNCTIONS
// ------------------------------------------------------------------------------------------------

fn parse_cal_to_cut_times(cal: Calendar) -> Vec<DateTime<Utc>> {
    let mut cut_times: Vec<DateTime<Utc>> = Vec::new();

    for component in &cal.components {
        if let CalendarComponent::Event(event) = component {
            if let DatePerhapsTime::DateTime(CalendarDateTime::Utc(time)) = match event.get_start()
            {
                Some(start_) => start_,
                None => continue, // we simply skip the iteration if we're unable to parse
            } {
                cut_times.push(time);
            };

            if let DatePerhapsTime::DateTime(CalendarDateTime::Utc(time)) = match event.get_end() {
                Some(end_) => end_,
                None => continue, // we simply skip the iteration if we're unable to parse
            } {
                cut_times.push(time);
            }
        }
    }

    cut_times
}

fn get_cut_times(calendar_list: Arc<Vec<String>>) -> Vec<DateTime<Utc>> {
    let mut cut_times: Vec<DateTime<Utc>> = Vec::new();

    for calendar_file in calendar_list.iter() {
        let cal: Calendar = match calendar_file.parse() {
            Ok(cal_) => cal_,
            Err(_) => continue, // NOTE: log?
        };
        cut_times.extend(parse_cal_to_cut_times(cal));
    }

    // sort the times and remove duplicate times
    cut_times.sort();
    cut_times.dedup();

    // sanity check before we take the first element
    if cut_times.len() <= 0 {
        return cut_times;
    }

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
        let cal: Calendar = match calendar_file.parse() {
            Ok(cal_) => cal_,
            Err(_) => continue, // NOTE: log?
        };

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
                match event.get_start() {
                    Some(start_) => start_,
                    None => continue, // we simply skip the iteration if we're unable to parse
                }
            {
                start = event_start_time;
            } else {
                continue;
            }

            if let DatePerhapsTime::DateTime(CalendarDateTime::Utc(event_end_time)) =
                match event.get_end() {
                    Some(end_) => end_,
                    None => continue, // we simply skip the iteration if we're unable to parse
                }
            {
                end = event_end_time;
            } else {
                continue;
            }

            let loc = match event.get_location() {
                Some(loc_) => loc_,
                None => continue,
            };

            if (&start <= start_time && start_time < &end)
                || (&start < end_time && end_time <= &end)
            {
                free_rooms.retain(|value| *value != loc);
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

fn get_allowed_level(
    start_time: &DateTime<Utc>,
    end_time: &DateTime<Utc>,
    calendar_list: Arc<Vec<String>>,
) -> AllowedActivities {
    let mut allowed_level: AllowedActivities =
        AllowedActivities::LoudPlayingAndBattery(WindowPosition::Open);

    for calendar_file in calendar_list.iter() {
        let cal: Calendar = match calendar_file.parse() {
            Ok(cal_) => cal_,
            Err(_) => continue,
        };

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
                match event.get_start() {
                    Some(start_) => start_,
                    None => continue, // we simply skip the iteration if we're unable to parse
                }
            {
                start = event_start_time;
            } else {
                continue;
            }

            if let DatePerhapsTime::DateTime(CalendarDateTime::Utc(event_end_time)) =
                match event.get_end() {
                    Some(end_) => end_,
                    None => continue, // we simply skip the iteration if we're unable to parse
                }
            {
                end = event_end_time;
            } else {
                continue;
            }

            if (&start <= start_time && start_time < &end)
                || (&start < end_time && end_time <= &end)
            {
                if let Some(location) = event.get_location() {
                    match location {
                        "EA-S106/S107 (TD06)" | "EA-S108/S109 (TD07)" | "EA-S008/S009 (TD17)" => {
                            let current_allowed_level =
                                AllowedActivities::QuietPlaying(WindowPosition::Closed);
                            if current_allowed_level < allowed_level {
                                allowed_level = current_allowed_level;
                            }
                        }

                        "EA-S101/S102 (TD04)" | "EA-S104/S105 (TD05)" | "EA-S110/S111 (TD08)" => {
                            let current_allowed_level =
                                AllowedActivities::QuietPlaying(WindowPosition::Open);
                            if current_allowed_level < allowed_level {
                                allowed_level = current_allowed_level;
                            }
                        }

                        "EA-S112/S113 (TD09)" | "EA-S114 (TD10)" | "EA-S115/S116 (TD11)" => {
                            let current_allowed_level =
                                AllowedActivities::LoudPlaying(WindowPosition::Closed);
                            if current_allowed_level < allowed_level {
                                allowed_level = current_allowed_level;
                            }
                        }

                        "EA-S117/S118 (TD12)" | "EA-S119/S120 (TD13)" | "EA-S121/S122 (TD14)" => {
                            let current_allowed_level =
                                AllowedActivities::LoudPlaying(WindowPosition::Open);
                            if current_allowed_level < allowed_level {
                                allowed_level = current_allowed_level;
                            }
                        }

                        "EB-P118/P119 (TD22)"
                        | "EB-P121 (TD23)"
                        | "EB-P123 (TD24)"
                        | "EB-P145 (TD25)"
                        | "EB-P147 (TD26)"
                        | "EB-P148/P150 (TD27)"
                        | "EB-P153/P156 (TD28)"
                        | "EA-S225 (TD15)"
                        | "EB-P010/P011 (TD20)"
                        | "EB-P117 (TD21)" => {
                            let current_allowed_level =
                                AllowedActivities::LoudPlayingAndBattery(WindowPosition::Closed);
                            if current_allowed_level < allowed_level {
                                allowed_level = current_allowed_level;
                            }
                        }

                        _ => {}
                    }
                } else {
                    continue;
                }
            }
        }
    }

    allowed_level
}

pub fn get_zik_calendar(room_list: Arc<Vec<String>>) -> Calendar {
    let mut cal = init_ade_cal();

    let cut_times: Vec<(DateTime<Utc>, DateTime<Utc>)> = get_cut_times(room_list.clone())
        .into_iter()
        .tuple_windows()
        .collect();

    for (start_time, end_time) in cut_times.iter() {
        let allowed_activities = get_allowed_level(start_time, end_time, room_list.clone());
        let start = DatePerhapsTime::DateTime(CalendarDateTime::Utc(start_time.clone()));
        let end = DatePerhapsTime::DateTime(CalendarDateTime::Utc(end_time.clone()));

        let activity = match allowed_activities {
            AllowedActivities::QuietPlaying(WindowPosition::Closed) => {
                "Faible volume, fenêtre fermée."
            }
            AllowedActivities::QuietPlaying(WindowPosition::Open) => {
                "Faible volume, fenêtre ouverte."
            }
            AllowedActivities::LoudPlaying(WindowPosition::Closed) => {
                "Volume élevé, fenêtre fermée."
            }
            AllowedActivities::LoudPlaying(WindowPosition::Open) => {
                "Volume élevé, fenêtre ouverte."
            }
            AllowedActivities::LoudPlayingAndBattery(WindowPosition::Closed) => {
                "Volume élevé et batterie, fenêtre fermée."
            }
            AllowedActivities::LoudPlayingAndBattery(WindowPosition::Open) => {
                "Volume élevé et batterie, fenêtre ouverte."
            }
        };

        cal.push(
            Event::new()
                .description(activity)
                .location("Le Zik, Le Zik, Le Zik")
                .starts(start)
                .ends(end)
                .summary("Volume max")
                .last_modified(Local::now().into())
                .created(DateTime::from_timestamp_nanos(0))
                .sequence(2141946518),
        );
    }

    cal.done()
}

mod tests {
    use icalendar::Calendar;

    use crate::calendar_parsing::parsing::parse_cal_to_cut_times;

    #[test]
    fn test_parsing() {
        let cal = Calendar::new();
        let _ret = parse_cal_to_cut_times(cal);

        assert!(_ret == Vec::<chrono::DateTime<chrono::Utc>>::new());
    }
}

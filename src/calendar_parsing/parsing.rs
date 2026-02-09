use chrono::{Duration, prelude::*};
use icalendar::{
    Calendar, CalendarComponent, CalendarDateTime, Component, DatePerhapsTime, Event, EventLike,
    Property,
};

use itertools::Itertools;

use crate::{
    caching::cal_caching::{
        cache_free_rooms_cal, get_cached_free_rooms_cal, get_resource_from_cache_file,
    },
    networking::ade_api_handling::get_free_rooms_calendar_list,
};

use crate::utils::noise_levels::{AllowedActivities, WindowPosition};
use crate::utils::rooms::EnseirbRoom;

const MAX_CALS_TOGETHER: usize = 3;

use std::sync::Arc;

fn parse_cal_to_cut_times(cal: Calendar) -> Vec<DateTime<Utc>> {
    let mut cut_times: Vec<DateTime<Utc>> = Vec::new();

    for component in &cal.components {
        if let CalendarComponent::Event(event) = component {
            if let DatePerhapsTime::DateTime(CalendarDateTime::Utc(time)) = match event.get_start()
            {
                Some(start_) => start_,
                None => {
                    tracing::warn!("Failed to parse event start time");
                    continue;
                }
            } {
                cut_times.push(time);
            };

            if let DatePerhapsTime::DateTime(CalendarDateTime::Utc(time)) = match event.get_end() {
                Some(end_) => end_,
                None => {
                    tracing::warn!("Failed to parse event end time");
                    continue;
                }
            } {
                cut_times.push(time);
            }
        }
    }

    cut_times
}

async fn get_cut_times(calendar_list: Arc<Vec<EnseirbRoom>>) -> Vec<DateTime<Utc>> {
    let mut cut_times: Vec<DateTime<Utc>> = Vec::new();

    let cal_list = match get_free_rooms_calendar_list(calendar_list).await {
        Ok(_list) => _list,
        Err(_) => {
            tracing::error!("Failed to get calendar cut times");
            return cut_times;
        }
    };

    for calendar_file in cal_list.iter() {
        let cal: Calendar = match calendar_file.parse() {
            Ok(cal_) => cal_,
            Err(_) => {
                tracing::warn!("Failed to parse calendar file, skipping...");
                continue;
            }
        };
        cut_times.extend(parse_cal_to_cut_times(cal));
    }

    // sort the times and remove duplicate times
    cut_times.sort();
    cut_times.dedup();

    // sanity check before we take the first element
    if cut_times.is_empty() {
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
    calendar_list: Arc<Vec<EnseirbRoom>>,
) -> String {
    let mut free_room_names: Vec<String> = calendar_list.iter().filter_map(|x| x.name()).collect();

    for calendar_file in calendar_list
        .iter()
        .filter_map(|x| x.id())
        .filter_map(get_resource_from_cache_file)
    {
        let cal: Calendar = match calendar_file.parse() {
            Ok(cal_) => cal_,
            Err(_) => {
                tracing::warn!("Failed to parse calendar file. Skipping...");
                continue;
            }
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
                    None => {
                        tracing::warn!("Failed to parse event start time. Skipping...");
                        continue;
                    } // we simply skip the iteration if we're unable to parse
                }
            {
                start = event_start_time;
            } else {
                continue;
            }

            if let DatePerhapsTime::DateTime(CalendarDateTime::Utc(event_end_time)) =
                match event.get_end() {
                    Some(end_) => end_,
                    None => {
                        tracing::warn!("Failed to parse event end time. Skipping...");
                        continue;
                    } // we simply skip the iteration if we're unable to parse
                }
            {
                end = event_end_time;
            } else {
                continue;
            }

            let loc = match event.get_location() {
                Some(loc_) => loc_,
                None => {
                    tracing::warn!("Failed to parse event location. Skipping...");
                    continue;
                } // we simply skip the iteration if we're unable to parse
            };

            if (&start <= start_time && start_time < &end)
                || (&start < end_time && end_time <= &end)
            {
                free_room_names.retain(|value| *value != loc);
            }
        }
    }

    free_room_names.join(", ")
}

fn init_ade_cal() -> Calendar {
    let mut cal: Calendar = Calendar::empty();

    cal.append_property(Property::new("METHOD", "REQUEST"));
    cal.append_property(Property::new("PRODID", "-//ADE/version 6.0"));
    cal.append_property(Property::new("VERSION", "2.0"));
    cal.append_property(Property::new("CALSCALE", "GREGORIAN"));

    cal
}

fn show_cals_together(calendar_list: Arc<Vec<EnseirbRoom>>) -> Calendar {
    let mut outcal = init_ade_cal();

    for calendar_file in calendar_list
        .iter()
        .filter_map(|x| x.id())
        .filter_map(get_resource_from_cache_file)
    {
        let mut appended: Calendar = match calendar_file.parse() {
            Ok(_cal) => _cal,
            Err(_) => {
                tracing::error!(
                    "Failed to parse calendar file, defaulting to empty calendar instead for this iteration..."
                );
                Calendar::new()
            }
        };
        outcal.append(&mut appended);
    }

    outcal
}

pub async fn get_free_rooms_calendar(calendar_list: Arc<Vec<EnseirbRoom>>) -> Calendar {
    match get_cached_free_rooms_cal(calendar_list.clone()) {
        Some(cal) => return cal,
        None => {
            tracing::info!("Cache miss, downloading and parsing free rooms...")
        }
    }

    let tmp: Vec<DateTime<Utc>> = get_cut_times(calendar_list.clone()).await;

    // NOTE: HAS to be after tmp's creation so we're sure to get a cache hit
    if calendar_list.len() <= MAX_CALS_TOGETHER {
        return show_cals_together(calendar_list);
    }

    let mut cal = init_ade_cal();

    let cut_times: Vec<(DateTime<Utc>, DateTime<Utc>)> = tmp.into_iter().tuple_windows().collect();

    // dbg!(&cut_times);

    for (start_time, end_time) in cut_times.iter() {
        let free_rooms = get_free_rooms(start_time, end_time, calendar_list.clone());
        let start = DatePerhapsTime::DateTime(CalendarDateTime::Utc(*start_time));
        let end = DatePerhapsTime::DateTime(CalendarDateTime::Utc(*end_time));
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

    let cal_final = cal.done();

    if let Err(e) = cache_free_rooms_cal(calendar_list, &cal_final) {
        tracing::warn!("Failed to cache computed calendar: {e}.");
    };

    cal_final
}

fn get_allowed_level(
    start_time: &DateTime<Utc>,
    end_time: &DateTime<Utc>,
    calendar_list: Arc<Vec<EnseirbRoom>>,
) -> AllowedActivities {
    let mut allowed_level: AllowedActivities =
        AllowedActivities::LoudPlayingAndBattery(WindowPosition::Open);

    for calendar_file in calendar_list.iter().filter_map(|x| x.name()) {
        let cal: Calendar = match calendar_file.parse() {
            Ok(cal_) => cal_,
            Err(_) => {
                tracing::warn!("Failed to parse calendar file. Skipping iteration...");
                continue;
            }
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
                    None => {
                        tracing::warn!("Failed to parse event start time. Skipping...");
                        continue;
                    } // we simply skip the iteration if we're unable to parse
                }
            {
                start = event_start_time;
            } else {
                continue;
            }

            if let DatePerhapsTime::DateTime(CalendarDateTime::Utc(event_end_time)) =
                match event.get_end() {
                    Some(end_) => end_,
                    None => {
                        tracing::warn!("Failed to parse event end time. Skipping...");
                        continue;
                    } // we simply skip the iteration if we're unable to parse
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

                        _ => {
                            tracing::warn!("Unknown room encoutered. Ignoring...")
                        }
                    }
                } else {
                    continue;
                }
            }
        }
    }

    allowed_level
}

pub async fn get_zik_calendar() -> Calendar {
    let room_list: Arc<Vec<EnseirbRoom>> = Arc::from(vec![
        EnseirbRoom::TD01,
        EnseirbRoom::TD02,
        EnseirbRoom::TD03,
        EnseirbRoom::TD04,
        EnseirbRoom::TD05,
        EnseirbRoom::TD06,
        EnseirbRoom::TD07,
        EnseirbRoom::TD08,
        EnseirbRoom::TD09,
        EnseirbRoom::TD10,
        EnseirbRoom::TD11,
        EnseirbRoom::TD12,
        EnseirbRoom::TD13,
        EnseirbRoom::TD14,
        EnseirbRoom::TD15,
        EnseirbRoom::TD17,
        EnseirbRoom::TD20,
        EnseirbRoom::TD21,
        EnseirbRoom::TD22,
        EnseirbRoom::TD23,
        EnseirbRoom::TD24,
        EnseirbRoom::TD25,
        EnseirbRoom::TD26,
        EnseirbRoom::TD27,
        EnseirbRoom::TD28,
    ]);
    let mut cal = init_ade_cal();

    let cut_times: Vec<(DateTime<Utc>, DateTime<Utc>)> = get_cut_times(room_list.clone())
        .await
        .into_iter()
        .tuple_windows()
        .collect();

    for (start_time, end_time) in cut_times.iter() {
        let allowed_activities = get_allowed_level(start_time, end_time, room_list.clone());
        let start = DatePerhapsTime::DateTime(CalendarDateTime::Utc(*start_time));
        let end = DatePerhapsTime::DateTime(CalendarDateTime::Utc(*end_time));

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

# Process for the program

## Objective

The objective of this project is to create a helper app which queries the schedule for the different rooms in the building,
picks out the free ones at any given time, and either displays it in the cli (easier, do it as first) or re-exports it as
an iCal.

A possible, VERY late feature might be allowing students to tell which class they're staying in so people can see which
rooms are free and avoid the trouble of searching for one.

## Workflow

As a language, I chose Rust 󱘗 for the only reason that I'm trying to learn it.

Here is the proposed workflow (==subject to change==)

- Query the calendar using `curl`
- use the [icalendar](https://crates.io/crates/icalendar) crate, with an example being [carl](https://github.com/b1rger/carl)
    - alternative if i'm struggling too much: [ical](https://crates.io/crates/ical) (seems better documented and more feature-rich, but is deprecated)
- Do my thing with them
    - query the planning for each TD room and every IT toom separately for easier navigation maybe
    - for each room, see each time window, if it's free then add it to the time window's `free` list
    - this way, we "reverse" the data, linking rooms to each time window instead of linking time windows to each room
- re-convert it to iCal for easy viewing on the ADE app, because writing my own android APK from scratch might be a bit much

## Implementation

### Getting the calendar

#### Web request

We ask nicely the ADE API using `curl`, and it nicely gives us the needed planning in either `iCal` or `vCal` format.

> [!NOTE]
> Rust bindings for `curl`: [curl](https://docs.rs/curl/latest/curl/)

I randomly chose `iCal` because the 2 formats look the same.

```bash
curl 'https://adeapp.bordeaux-inp.fr/jsp/custom/modules/plannings/anonymous_cal.jsp?resources=5091&projectId=1&calType=ical&firstDate=2024-08-19&lastDate=2025-08-22&displayConfigId=71'
```

#### Analyzing the adress format

- [k] resources
    - Which room / student / whatever to query
- [?] projectId
    - doesn't seem to change, so let's not touch it
- [i] calType
    - the format of the imported calendar, either `ical` or `vcal`
    - As mentionned before, we'll be using `ical`
- [k] firstDate
    - The beginning date of the exported calendar, in `YYYY-MM-DD` format
- [k] lastDate
    - The beginning date of the exported calendar, in `YYYY-MM-DD` format
- [?] displayConfigId
    - idk what this does
    - doesn't seem to change, so let's not touch it

We can manipulate these parameters to get the calendars we need.

!!!
This code requires pkg-config and openssl to work:
```nix
nix-shell -p pkg-config openssl
```

> [!todo]
> Make a nix-shell or nix-flake

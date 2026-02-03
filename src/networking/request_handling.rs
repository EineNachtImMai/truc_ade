use std::{collections::HashMap, net::SocketAddr};

use std::sync::Arc;

use crate::calendar_parsing::rooms::EnseirbRoom;
use crate::cli_params::arg_parsing::Args;
use clap::Parser;

use crate::calendar_parsing::parsing::{get_free_rooms_calendar, get_zik_calendar};

use axum::{body::Body, extract::Query, response::Response, routing::get, Router};

enum Mode {
    FreeRooms,
    Zik,
}

pub async fn serve(_zik_cal: Vec<EnseirbRoom>) {
    let args = Args::parse();
    let port = args.port;

    let addr = SocketAddr::from(([0, 0, 0, 0], port));

    /* let listener = match TcpListener::bind(addr.into()) {
        Ok(_listener) => _listener,
        Err(e) => {
            eprintln!("Failed to bind to address");
            return;
        }
    }; */

    let app = Router::new().route("/", get(handle_connection));

    let listener = match tokio::net::TcpListener::bind(addr.to_string()).await {
        Ok(_listener) => _listener,
        Err(_e) => {
            eprintln!("Failed to set up listener");
            return;
        }
    };

    eprintln!("Listening on {}", addr);

    let _ = axum::serve(listener, app).await; // TODO: LOGGING
}

fn parse_rooms(rooms: String) -> Arc<Vec<EnseirbRoom>> {
    // format: rooms separated by a ,
    let roomlist: Vec<EnseirbRoom> = rooms.split(',').filter_map(|x| EnseirbRoom::from_string(x.to_string())).collect();
    Arc::from(roomlist)
}

async fn handle_connection(Query(params): Query<HashMap<String, String>>) -> Response<Body> {
    // println!["{:?}", req];

    let mut mode: Mode = Mode::FreeRooms;

    let mut roomlist: Arc<Vec<EnseirbRoom>> = Arc::new(vec![
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

    if let Some(thing) = params.get("mode") {
        // println!["mode: {}", v];
        match thing.as_str() {
            "zik" => mode = Mode::Zik,
            _ => mode = Mode::FreeRooms,
        }
    }

    if let Some(thing) = params.get("room-list") {
        roomlist = parse_rooms(thing.into());
    }

    let content: String;

    match mode {
        Mode::Zik => {
            println!["chosen mode: zik"];
            content = format!("{}", get_zik_calendar().await)
        }
        Mode::FreeRooms => {
            println!["chosen mode: free rooms"];
            content = format!["{}", get_free_rooms_calendar(roomlist).await];
        }
    }

    Response::builder()
        .header("Content-Type", "text/calendar;charset=UTF-8")
        .header("Content-Disposition", "inline; filename=ADECal.ics")
        .body(Body::from(content))
        .unwrap() // TODO: error handling
}

mod tests {
    /* #[tokio::test]
    async fn request_handling_test_free_rooms() {
        todo!() // TODO: this test
    }

    #[tokio::test]
    async fn request_handling_test_zik() {
        todo!() // TODO: this test
    } */
}

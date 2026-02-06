use std::{collections::HashMap, net::SocketAddr};

use std::sync::Arc;

use crate::cli_params::*;
use crate::utils::rooms::EnseirbRoom;
use itertools::Itertools;

use crate::calendar_parsing::parsing::{get_free_rooms_calendar, get_zik_calendar};
use crate::utils::modes::Mode;

use axum::{body::Body, extract::Query, response::Response, routing::get, Router};

pub async fn serve() {
    let args = Args::parse();
    let port = args.port;

    let addr = SocketAddr::from(([0, 0, 0, 0], port));

    let app = Router::new().route("/", get(handle_connection));

    let listener = match tokio::net::TcpListener::bind(addr.to_string()).await {
        Ok(_listener) => _listener,
        Err(e) => {
            tracing::error!("Failed to set up listener: {e}");
            return;
        }
    };

    tracing::info!("Listening on {}", addr);

    if let Err(e) = axum::serve(listener, app).await {
        tracing::error!("Failed to setup server: {e}. Shutting down...");
    };
}

fn parse_rooms(rooms: String) -> Arc<Vec<EnseirbRoom>> {
    // format: rooms separated by a ,
    let roomlist: Vec<EnseirbRoom> = rooms
        .split(',')
        .filter_map(|x| EnseirbRoom::from_string(x.to_string()))
        .dedup()
        .collect();
    Arc::from(roomlist)
}

async fn handle_connection(Query(params): Query<HashMap<String, String>>) -> Response<Body> {
    tracing::info!("Got a connection!");
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
            tracing::info!["chosen mode: zik"];
            content = format!("{}", get_zik_calendar().await)
        }
        Mode::FreeRooms => {
            tracing::info!["chosen mode: free rooms"];
            content = format!["{}", get_free_rooms_calendar(roomlist).await];
        }
    }

    match Response::builder()
        .header("Content-Type", "text/calendar;charset=UTF-8")
        .header("Content-Disposition", "inline; filename=ADECal.ics")
        .body(Body::from(content))
    {
        Ok(resp) => resp,
        Err(e) => {
            tracing::warn!("Failed to build new response: {e}. Defaulting to empty response.");
            Response::new(Body::empty())
        }
    }
}

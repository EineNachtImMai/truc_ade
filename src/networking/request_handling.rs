use std::net::SocketAddr;

use form_urlencoded;

use std::sync::Arc;

use crate::cli_params::arg_parsing::Args;
use clap::Parser;

use crate::calendar_parsing::parsing::{get_free_roooms_calendar, get_zik_calendar};

use hyper::service::{make_service_fn, service_fn};
use hyper::{Body, Request, Response, Server};
use std::convert::Infallible;

enum Mode {
    FreeRooms,
    Zik,
}

pub async fn serve(zik_cal: Vec<String>, free_rooms_cal: Vec<String>) {
    let args = Args::parse();
    let port = args.port;

    let addr = SocketAddr::from(([0, 0, 0, 0], port));

    let free_rooms_cal_arc = Arc::new(free_rooms_cal);
    let zik_cal_arc = Arc::new(zik_cal);

    eprintln!("Listening on {}", addr);

    let make_service = make_service_fn(move |_conn| {
        let free_rooms_cal_clone = Arc::clone(&free_rooms_cal_arc);
        let zik_cal_clone = Arc::clone(&zik_cal_arc);
        async move {
            Ok::<_, Infallible>(service_fn(move |req| {
                let free_rooms_cal_clone = Arc::clone(&free_rooms_cal_clone);
                let zik_cal_clone = Arc::clone(&zik_cal_clone);
                async move {
                    Ok::<_, Infallible>(
                        handle_connection(zik_cal_clone, free_rooms_cal_clone, req).await,
                    )
                }
            }))
        }
    });

    if let Err(e) = Server::bind(&addr).serve(make_service).await {
        eprintln!("Error: {:#}", e);
        std::process::exit(1);
    } else {
        println!("Got a connection!")
    }
}

async fn handle_connection(
    zik_room_list: Arc<Vec<String>>,
    calendar_list: Arc<Vec<String>>,
    req: Request<Body>,
) -> Response<Body> {
    println!["{:?}", req];

    let mut mode: Mode = Mode::FreeRooms;

    // parse the request parameters to get the chosen mode
    if let Some(query) = req.uri().query() {
        for (k, v) in form_urlencoded::parse(query.as_bytes()) {
            if k == "mode" {
                println!["mode: {}", v];
                match &*v {
                    "zik" => mode = Mode::Zik,
                    _ => mode = Mode::FreeRooms,
                }
            }
        }
    }

    // default to an empty string in case something fucks up and doesnt change the content (this
    // supposedly can't happen)
    let mut content: String = String::from("");

    match mode {
        Mode::Zik => {
            println!["chosen mode: zik"];
            content = format!("{}", get_zik_calendar(zik_room_list))
        }
        Mode::FreeRooms => {
            println!["chosen mode: free rooms"];
            content = format!["{}", get_free_roooms_calendar(calendar_list)];
        }
    }

    Response::builder()
        .header("Content-Type", "text/calendar;charset=UTF-8")
        .header("Content-Disposition", "inline; filename=ADECal.ics")
        .body(Body::from(content))
        .unwrap()
}

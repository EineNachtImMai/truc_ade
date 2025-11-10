use std::{
    io::{prelude::*, BufReader, Write},
    net::{SocketAddr, TcpListener, TcpStream},
};

use std::sync::Arc;

use crate::cli_params::arg_parsing::Args;
use clap::Parser;

use crate::calendar_parsing::parsing::get_calendar;

use hyper::service::{make_service_fn, service_fn};
use hyper::{Body, Request, Response, Server, StatusCode};
use std::convert::Infallible;

pub async fn serve(cal: Vec<String>) {
    let args = Args::parse();
    let port = args.port;

    let addr = SocketAddr::from(([0, 0, 0, 0], port));

    let cal_arc = Arc::new(cal);

    eprintln!("Listening on {}", addr);

    // The closure passed to `make_service_fn` is executed each time a new
    // connection is established and returns a future that resolves to a
    // service.
    let make_service = make_service_fn(move |_conn| {
        let cal_clone = Arc::clone(&cal_arc);
        async move {
            Ok::<_, Infallible>(service_fn(move |req| {
                let cal_clone = Arc::clone(&cal_clone);
                async move { Ok::<_, Infallible>(handle_connection(cal_clone, req).await) }
            }))
        }
    });
    // Start the server.
    if let Err(e) = Server::bind(&addr).serve(make_service).await {
        eprintln!("Error: {:#}", e);
        std::process::exit(1);
    } else {
        println!("Got a connection!")
    }
}

async fn handle_connection(calendar_list: Arc<Vec<String>>, req: Request<Body>) -> Response<Body> {
    println!["{:?}", req];

    let content = format!["{}", get_calendar(calendar_list)];
    let length = content.len();

    let response = format!["HTTP/1.1 200 OK\r\nContent-Type: text/calendar;charset=UTF-8\r\nContent-Length: {length}\r\nContent-Disposition: inline; filename=ADECal.ics\r\n\r\n{content}"];

    Response::builder()
        .header("Content-Type", "text/calendar;charset=UTF-8")
        .header("Content-Disposition", "inline; filename=ADECal.ics")
        .body(Body::from(content))
        .unwrap()
}

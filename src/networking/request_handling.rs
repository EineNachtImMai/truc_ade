use std::{
    io::{prelude::*, BufReader, Write},
    net::{TcpListener, TcpStream},
};

use crate::cli_params::arg_parsing::Args;
use clap::Parser;

use crate::calendar_parsing::parsing::get_calendar;

use hyper::service::{make_service_fn, service_fn};
use hyper::{Body, Request, Response, Server};
use std::convert::Infallible;

async fn handle_conn() {
    let make_svc = make_service_fn(|_conn| async {
        Ok::<_, Infallible>(service_fn(handle_connection(stream, calendar_list, req)))
    });

    let addr = ([127, 0, 0, 1], 3000).into();
    let server = Server::bind(&addr).serve(make_svc);

    if let Err(e) = server.await {
        eprintln!("server error: {}", e);
    }
}

pub fn serve(calendar_list: Vec<String>) {
    let args = Args::parse();
    let port = args.port;
    let bind_address = format!("0.0.0.0:{port}");
    let listener = TcpListener::bind(bind_address).unwrap();

    for stream in listener.incoming() {
        // NOTE: debug purposes, remove in prod
        println!["Got a connection!"];
        let stream = stream.unwrap();

        handle_connection(stream, calendar_list.clone());
    }
}

async fn handle_connection(
    calendar_list: Vec<String>,
    req: Request<Body>,
) -> Result<Response<Body>, Infallible> {
    println!["{:?}", req];

    let content = format!["{}", get_calendar(calendar_list)];
    let length = content.len();

    let response = format!["HTTP/1.1 200 OK\r\nContent-Type: text/calendar;charset=UTF-8\r\nContent-Length: {length}\r\nContent-Disposition: inline; filename=ADECal.ics\r\n\r\n{content}"];

    stream.write_all(response.as_bytes()).unwrap();

    Ok(Response::builder()
        .header("Content-Type", "text/calendar;charset=UTF-8")
        .header("Content-Disposition", "inline; filename=ADECal.ics")
        .body(Body::from(content))
        .unwrap())
}

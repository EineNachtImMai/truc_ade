mod arg_parsing;
mod parsing;

use std::{
    io::{prelude::*, BufReader, Write},
    net::{TcpListener, TcpStream},
};

use arg_parsing::Args;
use clap::Parser;

use parsing::get_calendar;

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

    let response = format!["HTTP/1.1 200 OK\r\nContent-Type: text/calendar;charset=UTF-8\r\nContent-Length: {length}\r\nContent-Disposition: inline; filename=ADECal.ics\r\n\r\n{content}"];

    stream.write_all(response.as_bytes()).unwrap();
}

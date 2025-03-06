use log::debug;
use std::{
    fs,
    io::{BufRead, BufReader, Write},
    net::{TcpListener, TcpStream},
};

use crate::requestparser::HTTPVersion;
use log::info;
mod requestparser;
mod responsemaker;
mod logging;

fn main() {
    logging::configure_logging().unwrap();
    let listener = TcpListener::bind("[::0]:8080").unwrap();
    for stream in listener.incoming() {
        let stream = stream.unwrap();
        info!("connection established");
        handle_connection(stream);
    }
}

fn handle_connection(mut stream: TcpStream) {
    let buff_read = BufReader::new(&mut stream);
    let http_request_lines: Vec<_> = buff_read
        .lines()
        .map(|result| result.unwrap())
        .take_while(|line| !line.is_empty())
        .collect();
    debug!("receive request: {:#?}", http_request_lines);
    if http_request_lines.len() == 0 {
        return;
    }
    let http_request = requestparser::HTTPRequest::new(&http_request_lines[0]);
    let contents = fs::read_to_string("index.html").unwrap();
    // let response = responsemaker::HTTPResponse::new(HTTPVersion::HTTP1_1, 200, &contents);
    let response = responsemaker::HTTPResponse::new(HTTPVersion::HTTP1_1, 100, "");
    debug!("response: {}", response.to_string());
    stream.write_all(response.to_string().as_bytes()).unwrap();
}

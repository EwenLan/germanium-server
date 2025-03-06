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
    let request_line = buff_read.lines().next().unwrap().unwrap();
    let http_request = requestparser::HTTPRequest::new(&request_line);
    debug!("receive request: {:#?}", http_request);
    let contents = fs::read_to_string("index.html").unwrap();
    let response = responsemaker::HTTPResponse::new(HTTPVersion::HTTP1_1, 200, &contents);
    stream.write_all(response.to_string().as_bytes()).unwrap();
}

use std::collections::HashMap;
use std::fs;
use std::io::prelude::*;
use std::net::TcpListener;
use std::net::TcpStream;
use std::sync::mpsc;
use std::sync::Arc;
use std::sync::mpsc::Sender;
use crate::connections::ConnectionPool;
use crate::http::{HttpRequestHeader, HttpResponse, HttpRequest};
use std::io::Error;

mod connections;
mod http;
pub mod routes;
mod caching;

pub struct ServerConfig {

    connection_pool: ConnectionPool,
    listener: TcpListener
}


impl ServerConfig {

    pub fn create() -> ServerConfig {
        let listener = TcpListener::bind("127.0.0.1:61409").unwrap();
        let connection_pool = ConnectionPool::new(4).unwrap();

        ServerConfig {
            connection_pool,
            listener
        }
    }

    pub fn run(&self) {

        for stream in self.listener.incoming() {
            let stream = stream.unwrap();

            self.connection_pool.execute(move || {
                handle_connection(stream);
            });
        }
    }
}

fn handle_connection(mut stream: TcpStream) {

    let response = match get_request(&stream) {
        Ok(request) => {
            match handle_request(request) {
                Ok(response) => response,
                Err(_) => {
                    handle_500()
                }
            }
        }
        Err(_) => {
            handle_400()
        }
    };

    handle_response(&stream, response);
}

fn get_request(mut stream: &TcpStream) -> Result<HttpRequest, &'static str> {
    let mut buffer = [0; 2048];
    let mut body: Vec<u8> = Vec::new();
    stream.read(&mut buffer).unwrap();

    let (header, body_start_index) = HttpRequestHeader::create_from_buffer(buffer)?;
    let body = match (header.content_length > 0, body_start_index + header.content_length as usize > 2048)  {
        // Short cut -> content length is 0 so no body
        (false, _) => {
            None
        }
        // If the body_start_index + content length 
        // the request of the body is bigger than the buffer and more reads needed
        (true, true) => {
            // TODO handle!
            None
        }
        // If the body_start_index + content length < 2048,
        // the body is in the initial buffer and no more reading is needed.
        (true, false) => {

            let end = body_start_index + header.content_length as usize;

            let body= buffer[body_start_index..end].to_vec();

            Some(body)
        }
    };

    HttpRequest::create(header, body)

}

fn handle_request(request: HttpRequest) -> Result<HttpResponse, &'static str> {


    let body = " { \"message\": \"Hello, World!\"}".as_bytes().to_vec();

    Ok(HttpResponse::create(200, String::from("application/json"), Some(body)))
}

fn handle_response(mut stream: &TcpStream, mut response: HttpResponse) {
    match stream.write(&response.to_bytes()) {
        Ok(_) => {
            stream.flush();
        }
        Err(_) => {}
    }
}

fn handle_500() -> HttpResponse {

    let body = " { \"message\": \"Server error\"}".as_bytes().to_vec();

    HttpResponse::create(500, String::from("application/json"), Some(body))
}

fn handle_400() -> HttpResponse {

    let body = " { \"message\": \"Bad request\"}".as_bytes().to_vec();

    HttpResponse::create(400, String::from("application/json"), Some(body))
}
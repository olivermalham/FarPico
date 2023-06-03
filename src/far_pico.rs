use std::net::TcpStream;
use std::io::prelude::*;
use rust_embed::RustEmbed;
use crate::hal::HalFuncs;


// Pack the client side files into the executable
#[derive(RustEmbed)]
#[folder = "static/"]
struct StaticAsset;


pub fn process_connection<T: HalFuncs>(mut stream: TcpStream, hal: &T) {
    let mut buffer = [0; 1024];  // Fixed size buffer for incoming requests
    let mut request = String::new();

    match stream.read(&mut buffer) {
        Ok(size) => {
            request.push_str(String::from_utf8_lossy(&buffer[..size]).as_ref());

            match &*request {
                r if r.starts_with("GET /farpi") => handle_state_request(stream, hal),
                r if r.starts_with("GET ") => handle_static_request(stream, r),
                r if r.starts_with("POST /farpi") => handle_update_request(stream, hal, r),
                r => handle_not_found(stream, r),
            };
        }
        Err(e) => eprintln!("Unable to read stream: {}", e),
    }
}


fn handle_static_request(mut stream: TcpStream, request: &str){

    let path_parts: Vec<&str> = request.split(" ").collect();
    let mut trimmed_path = &path_parts[1][1..];
    println!("{} - {}", stream.peer_addr().unwrap(), trimmed_path);

    let content_type = match &trimmed_path {
        tp if tp.ends_with(".html") => "text/html",
        tp if tp.ends_with(".js") => "text/javascript",
        tp if tp.ends_with(".css") => "text/css",
        tp if tp.ends_with(".json") => "application/json",
        tp if tp.ends_with(".jpg") => "image/jpeg",
        tp if tp.ends_with(".jpeg") => "image/jpeg",
        tp if tp.ends_with(".gif") => "image/gif",
        tp if tp.ends_with(".png") => "image/png",
        _ => "text/html"
    };

    if trimmed_path == "" {
        trimmed_path = "index.html";
    }

    match StaticAsset::get(trimmed_path) {
        Some(asset) => {
            stream.write(("HTTP/1.1 200 OK\r\nContent-Type: ".to_string() + content_type + "\r\n\r\n").as_bytes()).unwrap();
            stream.write(asset.data.as_ref()).unwrap();
        },
        None => {
            handle_not_found(stream, request);
        }
    };
}


// Return the HAL state serialised as JSON. Request data isn't required, so ignored
fn handle_state_request<T: HalFuncs>(mut stream: TcpStream, hal: &T) {
    stream.write(("HTTP/1.1 200 OK\r\nContent-Type: application/json\r\n\r\n").as_bytes()).unwrap();
    stream.write(hal.to_json().as_bytes()).unwrap();
}


// Handles action calls. Sends the updated state to the client
fn handle_update_request<T: HalFuncs>(stream: TcpStream, hal: &T, request: &str) {
    // Remove headers from request body
    let request_parts: Vec<&str> = request.split("\r\n\r\n").collect();

    if request_parts.len() != 2 { return handle_bad_request(stream, request)};

    let request_body = request_parts[1];

    hal.dispatch(request_body);
    handle_state_request(stream, hal);
}


// 404 - Not Found response handler
fn handle_not_found(mut stream: TcpStream, _: &str){
    stream.write("HTTP/1.1 404 NOT FOUND\r\n\r\n".as_bytes()).unwrap();
}


// 400 - Bad Reqeust response handler
fn handle_bad_request(mut stream: TcpStream, _: &str){
    stream.write("HTTP/1.1 400 BAD REQUEST\r\n\r\n".as_bytes()).unwrap();
}

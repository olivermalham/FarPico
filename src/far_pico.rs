use std::net::TcpStream;
use std::io::prelude::*;
use rust_embed::RustEmbed;



// Pack the client side files into the executable
#[derive(RustEmbed)]
#[folder = "static/"]
struct StaticAsset;



pub fn process_connection(mut stream: TcpStream) {
    let mut buffer = [0; 1024];
    let mut request = String::new();

    match stream.read(&mut buffer) {
        Ok(size) => {
            request.push_str(String::from_utf8_lossy(&buffer[..size]).as_ref());

            match &*request {
                request_contents if request_contents.starts_with("GET /farpi") => handle_state_request(stream, request_contents),
                request_contents if request_contents.starts_with("GET ") => handle_static_request(stream, request_contents),
                request_contents if request_contents.starts_with("PUT /farpi") => handle_update_request(stream, request_contents),
                request_contents => handle_not_found(stream, request_contents),
            };
        }
        Err(e) => eprintln!("Unable to read stream: {}", e),
    }
}


fn handle_static_request(mut stream: TcpStream, request: &str){

    let path_parts: Vec<_> = request.split(" ").collect();
    let trimmed_path = &path_parts[1][1..];
    println!("{:?}", trimmed_path);

    let content_type = match &trimmed_path {
        trimmed_path if trimmed_path.ends_with(".html") => "text/html",
        trimmed_path if trimmed_path.ends_with(".js") => "text/javascript",
        trimmed_path if trimmed_path.ends_with(".css") => "text/css",
        trimmed_path if trimmed_path.ends_with(".json") => "application/json",
        trimmed_path if trimmed_path.ends_with(".jpg") => "image/jpeg",
        trimmed_path if trimmed_path.ends_with(".jpeg") => "image/jpeg",
        trimmed_path if trimmed_path.ends_with(".gif") => "image/gif",
        trimmed_path if trimmed_path.ends_with(".png") => "image/png",
        _ => "text/html" // Default to HTML text
    };

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
fn handle_state_request(mut stream: TcpStream, _: &str) {
    stream.write(("HTTP/1.1 200 OK\r\nContent-Type: application/json\r\n\r\n").as_bytes()).unwrap();
}


// Handles action calls. Sends the updated state to the client
fn handle_update_request(mut stream: TcpStream, request: &str) {

    handle_state_request(stream, request);
}

// Basic 404 response handler
fn handle_not_found(mut stream: TcpStream, request: &str){
    stream.write("HTTP/1.1 404 NOT FOUND\r\n\r\n".as_bytes()).unwrap();
}
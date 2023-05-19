use std::net::TcpStream;
use std::io::prelude::*;
use std::num::IntErrorKind::Empty;
use rust_embed::RustEmbed;


// Constants
const HTML_OK_RESPONSE: &str = "HTTP/1.1 200 OK\r\nContent-Type: text/html\r\n\r\n";
const JSON_OK_RESPONSE: &str = "HTTP/1.1 200 OK\r\nContent-Type: application/json\r\n\r\n";
const NOT_FOUND:        &str = "HTTP/1.1 404 NOT FOUND\r\n\r\n";
const INTERNAL_ERROR:   &str = "HTTP/1.1 500 INTERNAL ERROR\r\n\r\n";



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

            let (status_line, content) = match &*request {
                request_contents if request_contents.starts_with("GET /farpi") => handle_state_request(request_contents),
                request_contents if request_contents.starts_with("GET /") => handle_static_request(request_contents),
                request_contents if request_contents.starts_with("PUT /farpi") => handle_update_request(request_contents),
                _ => (NOT_FOUND.to_string(), "404 not found".to_string()),
            };

            stream.write_all(format!("{}{}", status_line, content).as_bytes()).unwrap();
        }
        Err(e) => eprintln!("Unable to read stream: {}", e),
    }
}


fn handle_static_request(request: &str) -> (String, String){

    let path_parts: Vec<_> = request.split(" ").collect();
    let trimmed_path = &path_parts[1][1..];
    println!("{:?}", trimmed_path);

    match StaticAsset::get(trimmed_path) {
        Some(asset) => (HTML_OK_RESPONSE.to_string(), std::str::from_utf8(asset.data.as_ref()).unwrap().to_string()),
        None => (NOT_FOUND.to_string(), "{\"json\":\"response\"}".to_string())
    }
}


fn handle_state_request(request: &str) -> (String, String){

    (JSON_OK_RESPONSE.to_string(), "{\"json\":\"response\"}".to_string())
}


fn handle_update_request(request: &str) -> (String, String){

    (JSON_OK_RESPONSE.to_string(), "{\"json\":\"update response\"}".to_string())
}

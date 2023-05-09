use std::net::{TcpStream, Shutdown};
use std::io::{BufReader, BufWriter};
use std::io::prelude::*;
use sha1::{Sha1, Digest};
use base64::{engine::general_purpose, Engine as _};


#[derive(Debug)]
enum ConnectionState {
    New,
    Closed,
    HTTP,
    WsHandshake,
    WsOpen
}


#[derive(Debug)]
pub struct Connection {
    connection: TcpStream,
    status: ConnectionState,
    path: String,
    key: String
}


pub fn new_connection(tcp_connection: TcpStream) -> Connection {

    let mut connection : Connection = check_headers(new(tcp_connection));

    println!("New connection:\n {:?}", connection);

    // Determine connection type
    match connection.status {
        // HTTP Connection, so send the boiler plate HTML and close the connection
        ConnectionState::HTTP => {
            connection.connection.write_all(html_response()).unwrap();
            connection.connection.shutdown(Shutdown::Both).expect("shutdown call failed");
            connection.status = ConnectionState::Closed;
            println!("Connection Closed");
        }
        // WebSocket connection attempt so send back the handshake
        ConnectionState::WsHandshake => {
            println!("WsHandshake: {:?}", ws_handshake(&connection));
            connection.connection.write_all(ws_handshake(&connection).as_bytes()).unwrap();
            connection.status = ConnectionState::WsOpen;
        }
        // WebSocket connection open
        ConnectionState::WsOpen => {
            println!("WsOpen!!!");
        }
        _ => ()
    }

    return connection;
}


fn check_headers(mut connection: Connection) -> Connection {
    let reader = BufReader::new(&mut connection.connection);
    let http_request: Vec<_> = reader
        .lines()
        .map(|result| result.unwrap())
        .take_while(|line| !line.is_empty())
        .collect();

    for header in &http_request {
        match header.split_once(':') {
            Some((key, value)) => {
                match key.trim().to_uppercase().as_str(){
                    "UPGRADE" => { connection.status = ConnectionState::WsHandshake }
                    "SEC-WEBSOCKET-KEY" => { connection.key = value.trim().to_string() }
                    _ => ()
                }
            }
            None => {
                // Not a header string, check if it is an HTTP GET action
                if header.to_uppercase().starts_with("GET") {
                    connection.status = ConnectionState::HTTP;
                    let path_parts: Vec<_> = header.split(" ").collect();
                    connection.path = path_parts[1].to_string();

                } else {
                    println!("No header, not a GET");
                }
            }
        }
    }

    return connection;
}


fn html_response() -> &'static [u8]{
r#"HTTP/1.1 200 OK

<!DOCTYPE html>
<html lang="en-US">

<head>
<meta charset="UTF-8">
<meta name="viewport" content="width=device-width, initial-scale=1">
<title>FarPico</title>
<script>
    let webSocket = new WebSocket('ws://localhost:7878/farpi');
    webSocket.onmessage = function(e) { console.log(e); webSocket.send(e);}
</script>
</head>

<body>
FarPico
</body>
</html>"#.as_bytes()
}


fn ws_handshake(connection: &Connection) -> String {
    // Accept header string = sha1(client_key + 258EAFA5-E914-47DA-95CA-C5AB0DC85B11)
    let start: String = String::from("HTTP/1.1 101 Switching Protocols\r
Upgrade: websocket\r
Connection: Upgrade\r
Server: FarPico\r
Sec-WebSocket-Accept: ");

    // Generate the accept token
    let mut hasher = Sha1::new();
    hasher.update((connection.key.clone() + "258EAFA5-E914-47DA-95CA-C5AB0DC85B11").as_bytes());
    let result: String = general_purpose::STANDARD.encode(hasher.finalize());

    start + result.as_str() + &String::from("\r\n\r\n")
}


pub fn new(new_connection: TcpStream) -> Connection {
    Connection {
        connection: new_connection,
        status: ConnectionState::New,
        path: "".to_string(),
        key: "".to_string()
    }
}


pub fn gather_actions(connection: &mut Connection){

    match  connection.status {
        ConnectionState::WsOpen => {
            let reader = BufReader::new(&mut connection.connection);
            let http_request: Vec<_> = reader
                .lines()
                .map(|result| result.unwrap())
                .take_while(|line| !line.is_empty())
                .collect();
            println!("Data read: {:?}", http_request);
        },
        ConnectionState::Closed => (),
        _ => ()
    }
}


pub fn send_state(connection: &mut Connection, data: String){

    match  connection.status {
        ConnectionState::WsOpen => {
            match connection.connection.write(data.as_bytes()){
                Ok(bytes)=> { println!("Sent {:?} bytes", bytes); },
                Err(_)=>{ println!("Error writing bytes to {:?}", connection); }
            }
            println!("Data sent: {:?}", data);
        },
        ConnectionState::Closed => (),
        _ => ()
    }
}


// TODO: Build websocket compatible headers for the provided data
pub fn build_packet(connection: &Connection, data: String){

}

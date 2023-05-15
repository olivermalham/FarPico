use std::net::{TcpStream, Shutdown};
use std::io::BufReader;
use std::io::prelude::*;



#[derive(Debug)]
enum ConnectionState {
    New,
    GET,
    PUT,
}

#[derive(Debug)]
pub struct Connection {
    connection: TcpStream,
    status: ConnectionState,
    path: String,
    key: String,
}


pub fn new(new_connection: TcpStream) -> Connection {
    Connection {
        connection: new_connection,
        status: ConnectionState::New,
        path: "".to_string(),
        key: "".to_string()
    }
}


pub fn process_connection(tcp_connection: TcpStream){

    let mut connection : Connection = read_request(new(tcp_connection));

    println!("New connection:\n {:?}", connection);

    // Determine connection type
    match connection.status {
        // GET request, so send the current state
        ConnectionState::GET => {
            println!("GET request");
            connection.connection.write_all(html_response()).unwrap();
            connection.connection.shutdown(Shutdown::Both).expect("shutdown call failed");
        }
        // PUT request, so process as an action string / RPC call from the client
        ConnectionState::PUT => {
            println!("PUT request");
            connection.connection.shutdown(Shutdown::Both).expect("shutdown call failed");
        }
        _ => ()
    }
}


fn read_request(mut connection: Connection) -> Connection {
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
                    _ => ()
                }
            }
            None => {
                // Not a header string, check if it is an HTTP GET action
                if header.to_uppercase().starts_with("GET") {
                    connection.status = ConnectionState::GET;
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
    // let webSocket = new WebSocket('ws://localhost:7878/farpi');
    // webSocket.onmessage = function(e) { console.log(e); webSocket.send(e);}
</script>
</head>

<body>
FarPico
</body>
</html>"#.as_bytes()
}


pub fn send_state(connection: &mut Connection, data: String){

}

mod hal;
mod far_pico;
use std::io;
use std::net::TcpListener;


/*

FarPico - FarPi compatible server written in pure Rust. Designed to be as lightweight as
possible so that it can run on microcontrollers as long as they have some kind of TCP handling
stack / functionality, such as RaspberryPi Pico W, or Pico with external wifi module like ESP-07.
Implements a rudimentary HTTP server to support connection upgrade to WebSockets. No security
plans at the moment, assume that it will only be used on a private wifi net.

*/

fn main() {
    let listener = TcpListener::bind("127.0.0.1:7878").unwrap();
    listener.set_nonblocking(true).expect("Failed call to set_nonblocking");

    /* To think about:
    Need a HAL. How to handle? Traits probably the best bet, but might be a little advanced for
    me at the moment.
    */
    let mut connections: Vec<far_pico::Connection> = Vec::new();

    // Infinite loop
    loop {
        // Create new connection object if a client waiting
        match listener.accept() {
            Ok((stream, _addr)) => {
                connections.push(far_pico::new_connection(stream));
            },
            Err(ref e) if e.kind() == io::ErrorKind::WouldBlock => {
                // Non-blocking means we get this error when no new connections are available
                // Don't need to do anything here, just let execution continue
                ();
            },
            Err(e) => panic!("encountered IO error: {}", e),
        }

        // println!("Client list: {:?}", connections);

        // Handle connection state changes as required
        for client in &mut connections {
            far_pico::handle_connection(client);
        }

        // Process any action strings received from the clients

        // Send HAL state updates
    }
}


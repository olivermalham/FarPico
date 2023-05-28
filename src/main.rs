mod hal;
mod far_pico;
mod crawl;

use std::io;
use std::net::TcpListener;
use std::string::ToString;
use crate::far_pico::process_connection;

/*

FarPico - FarPi compatible server written in pure Rust. Designed to be as lightweight as
possible so that it can run on microcontrollers as long as they have some kind of TCP handling
stack / functionality, such as RaspberryPi Pico W, Pico with external wifi module, ESP32 module etc.
Implements basic HTTP version of the FarPI interface, web sockets and the binary comms are too
much for my level of Rust skills atm.

*/

// Use "0.0.0.0" for the address to bind to whatever the host address is
const SERVER_ADDRESS: &str = "0.0.0.0:7878";

fn main() {
    println!("FarPico Server {}", env!("CARGO_PKG_VERSION"));
    println!("Starting on {}", SERVER_ADDRESS);
    let listener = TcpListener::bind(SERVER_ADDRESS).unwrap();
    listener.set_nonblocking(true).expect("Failed call to set_nonblocking");

    // Build the HAL structure - update this line to use a project-specific HAL
    let hal = hal::build_hal();

    // Infinite loop
    loop {
        // Create new connection object if a client waiting
        match listener.accept() {
            Ok((stream, _addr)) => {
                process_connection(stream, &hal);
            },
            Err(ref e) if e.kind() == io::ErrorKind::WouldBlock => {
                // Non-blocking means we get this error when no new connections are available
                // Don't need to do anything here, just let execution continue
                ();
            },
            Err(e) => panic!("encountered IO error: {}", e),
        }
    }
}

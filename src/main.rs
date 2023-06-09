mod hal;
mod far_pico;
mod hal_gpio;
mod hal_console;

use std::io;
use std::net::TcpListener;
use far_pico::process_connection;

// TODO: Update these two lines to use the HAL for a specific project
mod example_hal;

use example_hal as active_hal;


/*

FarPico - FarPi compatible server written in pure Rust. Designed to be as lightweight as
possible so that it can run on microcontrollers as long as they have some kind of TCP handling
stack / functionality, such as RaspberryPi Pico W, Pico with external wifi module, ESP32 module etc.
May need to implement a custom version of the TcpListener and TcpStream modules to support them.
Implements basic HTTP version of the FarPI interface, web sockets and the binary comms are too
much for my level of Rust skills atm.

*/

// Use "0.0.0.0" for the address to bind to whatever the host address is. If running in access
// point mode, this should probably be set to a concrete IP.
const SERVER_ADDRESS: &str = "0.0.0.0:7878";

fn main() {
    println!("------------------------------------------------");
    println!("\tFarPico Server {}", env!("CARGO_PKG_VERSION"));
    println!("------------------------------------------------");
    println!("Starting on {}", SERVER_ADDRESS);
    let listener = TcpListener::bind(SERVER_ADDRESS).unwrap();
    listener.set_nonblocking(true).expect("Failed call to set_nonblocking");

    // Build the HAL structure - update this line to use a project-specific HAL
    let mut hal = active_hal::build_hal();

    // Infinite loop
    println!("Running");
    println!("------------------------------------------------");
    loop {
        // Create new connection object if a client waiting
        match listener.accept() {
            Ok((stream, _addr)) => {
                process_connection(stream, &mut hal);
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

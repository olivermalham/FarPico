mod hal;
mod far_pico;
mod crawl;

use std::io;
use std::net::TcpListener;
use crate::far_pico::process_connection;


/*

FarPico - FarPi compatible server written in pure Rust. Designed to be as lightweight as
possible so that it can run on microcontrollers as long as they have some kind of TCP handling
stack / functionality, such as RaspberryPi Pico W, or Pico with external wifi module like ESP-07.
Implements basic HTTP version of the FarPI interface, web sockets and the binary comms are too
much for my level of Rust skills atm.

*/

fn main() {
    let listener = TcpListener::bind("127.0.0.1:7878").unwrap();
    listener.set_nonblocking(true).expect("Failed call to set_nonblocking");

    /* To think about:
    Need a HAL. How to handle? Traits probably the best bet, but might be a little advanced for
    me at the moment.
    */

    // Infinite loop
    loop {
        // Create new connection object if a client waiting
        match listener.accept() {
            Ok((stream, _addr)) => {
                process_connection(stream);
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

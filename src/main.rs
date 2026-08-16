mod core;
use core::Server;
use std::eprint;
use std::{io::stdin, println, process, thread};
use std::sync::Arc;

const IP_ADDRESS: &str = "127.0.0.1";
const PORT_NUMBER: usize = 8080;
const POOL_SIZE: usize = 20;

fn main() {
    let server = Server::new(
        IP_ADDRESS.to_string(),
        PORT_NUMBER,
        POOL_SIZE
    ).expect("failed while initializing the server");

    let server = Arc::new(server);
    thread::spawn(move|| {
        if let Err(error) = server.listen() {
            eprint!("Critical error... {}", error)
        }
    });

    let mut buffer: String = String::new();
    println!("Server running on {}:{}", IP_ADDRESS, PORT_NUMBER);
    println!("Tap enter to stop...");

    stdin().read_line(&mut buffer).unwrap();
    println!("Bye...");
}

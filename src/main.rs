mod core;
use core::Server;
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
    );

    if let Err(err) = server {
        println!("Error... {}", err);
        process::exit(1);
    }

    let server = Arc::new(server.unwrap());
    thread::spawn(move|| {
        server.listen();
    });

    let mut buffer: String = String::new();
    println!("Server running on {}:{}", IP_ADDRESS, PORT_NUMBER);
    println!("Tap enter to stop...");

    stdin().read_line(&mut buffer).unwrap();
    println!("Bye...");
}

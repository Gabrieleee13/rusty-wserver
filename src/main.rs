mod core;
use core::Server;
use std::eprint;
use std::{io::stdin, println, thread};
use std::sync::Arc;

// Info of the server 
const IP_ADDRESS: &str = "127.0.0.1";
const PORT_NUMBER: usize = 8080;
const POOL_SIZE: usize = 20;

fn main() {

    /* 
        Initializing the Server object
        If Server::new return a Err() will be printed a error msg and the program will panic 
    */
    let server = Server::new(
        IP_ADDRESS.to_string(),
        PORT_NUMBER,
        POOL_SIZE
    ).expect("failed while initializing the server");


    // Wrapping the server in an Arc pointer bc the listen method need to be called by an Arc<Server>
    let server = Arc::new(server);

    // Creating a separeted thread for the connections
    thread::spawn(move|| {

        /*
            If the listen method return an Err() for a single connection 
            will be log on screen and the thread will continue to listen for other connections
        */
        if let Err(error) = server.listen() {
            eprint!("Critical error... {}", error)
        }

    });

    // If enter is press the main thread will terminate and even the thread for the connections
    let mut buffer: String = String::new();
    println!("Server running on {}:{}", IP_ADDRESS, PORT_NUMBER);
    println!("Tap enter to stop...");

    stdin().read_line(&mut buffer).unwrap();
    println!("Bye...");
}

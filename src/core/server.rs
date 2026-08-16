use std::{io::{BufRead, BufReader, Write}, net::TcpStream, println, process, sync::Arc};

use threadpool::ThreadPool;
use crate::{core::socket::Socket};

pub struct Server {
    socket: Socket,
    pool: ThreadPool
}

impl Server {

    pub fn new(ip: String, port: usize, pool_size: usize) -> Result<Server, String> {
        let socket = Socket::new(ip, port);

        if let Err(error) = socket {
            return Err(error);
        }

        if pool_size < 1 {
            return Err("invalid pool size".to_string());
        }

        return Ok(Server{ socket: socket.unwrap(), pool: ThreadPool::new(pool_size) });
    }

    pub fn listen(self: Arc<Self>) {
        let listener = self.socket.bind();

        let listener = listener.unwrap_or_else(|error| {
            println!("Error... {}", error);
            process::exit(1);
        });

        for stream in listener.incoming() {
            if let Ok(stream) = stream {
                let server = Arc::clone(&self);
                self.pool.execute(move|| {
                    server.handle_stream(stream);
                });
            }
        } 
    }

    fn handle_stream(&self,  mut stream: TcpStream) -> () {
        let _request: Vec<String> = BufReader::new(&stream)
        .lines()
        .map(|r| r.unwrap())
        .take_while(|s| !s.is_empty())
        .collect();

        let response = "HTTP/1.1 200 OK\r\nContent-Type: text/html\r\n\r\n<h1>Hello, World!</h1>";
        let send_result = stream.write_all(response.as_bytes());

        if let Err(error) = send_result {
            println!("Error... {}", error);
        }
    }
}
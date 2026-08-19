use std::{io::{BufRead, BufReader, Write}, net::TcpStream, println, sync::{Arc, mpsc}};
use crate::http::httphandler::HttpHandler;

use threadpool::ThreadPool;
use crate::{core::socket::Socket};

pub struct Server {
    socket: Socket,
    pool: ThreadPool
}

impl Server {

    pub fn new(ip: String, port: usize, pool_size: usize) -> Result<Server, String> {
        let socket = Socket::new(ip, port)?;

        if pool_size < 1 {
            return Err("invalid pool size".to_string());
        }

        return Ok(Server{ socket: socket, pool: ThreadPool::new(pool_size) });
    }

    pub fn listen(self: Arc<Self>) -> Result<(), String>{
        let listener = self.socket.bind().map_err(|e| e.to_string())?;

        for stream in listener.incoming() {
            if let Ok(stream) = stream {
                let server = Arc::clone(&self);

                self.pool.execute(move|| {
                    let stream_result = server.handle_stream(stream);

                    if let Err(str) = stream_result {
                        println!("Error while manage the client... {}", str)  
                    }
                });
            }
        } 

        return Ok(())
    }

    fn handle_stream(&self, stream: TcpStream) -> Result<(), String> {
        let handler = HttpHandler::new(stream);
        let _ = handler.handle_stream()?;

        Ok(())
    }
}
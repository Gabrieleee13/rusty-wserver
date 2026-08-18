use std::{io::{BufRead, BufReader, Write}, net::TcpStream, sync::{Arc, mpsc}};
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
        let listener = self.socket.bind();

        if let Err(e) = listener {
            return Err(e.to_string());
        }

        for stream in listener.unwrap().incoming() {
            if let Ok(stream) = stream {
                let server = Arc::clone(&self);

                let (sender, reciver) = mpsc::channel();

                self.pool.execute(move|| {
                    let stream_result = server.handle_stream(stream);
                    let _ = sender.send(stream_result);
                });

                reciver.recv().unwrap()?;
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
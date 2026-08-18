use std::{io::{BufRead, BufReader}, net::TcpStream, println};
use std::cell::RefCell;

pub struct HttpHandler {
    stream: TcpStream,
    raw_request: RefCell<Option<Vec<String>>>
}

impl HttpHandler {

    pub fn new(stream: TcpStream) -> HttpHandler {
        return HttpHandler { stream: stream, raw_request: RefCell::new(None) }
    }

    pub fn handle_stream(&self) -> Result<(), String> {
        *self.raw_request.borrow_mut() = self.get_raw_request();

        if let None = *self.raw_request.borrow() {
            return Err("error while reading the request".to_string());
        }

        println!("{:#?}", *self.raw_request.borrow());

        Ok(())
    }

    fn get_raw_request(&self) -> Option<Vec<String>>{
        let reader = BufReader::new(&self.stream);
        let mut vec_req = Vec::new();

        for line in reader.lines() {
            let line = line.ok()?; 

            if line.is_empty() {
                break;
            }

            vec_req.push(line);
        }

        Some(vec_req)
    }
}
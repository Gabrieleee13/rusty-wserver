use std::{io::{BufRead, BufReader}, net::TcpStream};
use std::cell::RefCell;

use crate::http::request::Request;

pub struct HttpHandler {
    stream: TcpStream,
    raw_request: RefCell<Option<Vec<String>>>,
    request: RefCell<Option<Request>>
}

impl HttpHandler {

    pub fn new(stream: TcpStream) -> HttpHandler {
        return HttpHandler {
            stream: stream,
            raw_request: RefCell::new(None),
            request: RefCell::new(None)
        }
    }

    pub fn handle_stream(&self) -> Result<(), String> {

        *self.raw_request.borrow_mut() = self.get_raw_request();

        let mut raw_request_refmut = self.raw_request.borrow_mut();
        let raw_request = raw_request_refmut.get_or_insert_with(Vec::new);

        let _request_result = Request::build_from_raw(raw_request);

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
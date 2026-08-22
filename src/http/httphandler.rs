use std::{io::{BufRead, BufReader}, net::TcpStream};
use std::cell::RefCell;
use std::rc::Rc;

use crate::http::request::{Request, RequestValidator, HttpStatus};

pub struct HttpHandler {
    stream: TcpStream,

    /**
     * Uses Rc<RefCell<Option<Request>>> to manage shared, mutable state without cloning heavy data.
     * Rc allows multiple owners to share the same Request instance by cloning only the lightweight pointer.
     * RefCell is required because Rc only provides immutable access (&T) by default; 
     * RefCell enables interior mutability, allowing us to modify or extract the Option<Request>
     * (e.g., via borrow_mut()) even when we only hold an immutable reference to the Rc. 
     * Since Request contains a Vec (non-Copy), we cannot duplicate it implicitly. 
     * RefCell enforces borrow checking at runtime, ensuring safety while allowing the Request to be moved out (via .take()) 
     * or modified in place when needed
     * 
     * Same thing for the request validator
     * 
     * both request and request_validator are an Option<T> bc when they get created we don't know the msg 
     * that the client is sending to us
     */
    request: Rc<RefCell<Option<Request>>>,
    request_validator: RefCell<Option<RequestValidator>>
}

impl HttpHandler {

    pub fn new(stream: TcpStream) -> HttpHandler {
        return HttpHandler {
            stream: stream,
            request: Rc::new(RefCell::new(None)),
            request_validator: RefCell::new(None)
        }
    }

    pub fn handle_stream(&self) -> Result<(), String> {

        let request = self.get_obj_request();        

        if let Err(http_err) = request {
            // Errore
        }

        let request = request.unwrap();
        *self.request.borrow_mut() = Some(request);

        self.init_request_validator();

        println!("{:?}", *self.request.borrow());


        Result::Ok(())
    }

    fn get_raw_request(&self) -> Option<Vec<String>> {
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

    fn get_obj_request(&self) -> Result<Request, HttpStatus> {

        let raw_request = self.get_raw_request();

        if let None = raw_request {
            return Err(HttpStatus::BadRequest);
        }
        let request_vec = &mut raw_request.unwrap();
        let request = Request::build_from_raw(request_vec);
        return request;

    }

    fn init_request_validator(&self) -> () {

        let request_copy: Rc<RefCell<Option<Request>>> = Rc::clone(&self.request);

        // Don't know why .take() method works on a Rc<T>
        let request = request_copy.take();

        *self.request_validator.borrow_mut() = Some(RequestValidator::new(request.unwrap()));
    }
}
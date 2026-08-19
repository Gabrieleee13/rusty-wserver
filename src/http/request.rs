use core::{fmt};
use std::{collections::{HashMap}, str::FromStr, todo, write};

const HTTP_WHITE_LINE: &str = "\r\n\r\n";

pub enum HttpMethod {
    Get,
    Post,
    Put,
    Delete
}

impl fmt::Display for HttpMethod {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        return match self {
            HttpMethod::Get => write!(f, "GET"),
            HttpMethod::Post => write!(f, "POST"),
            HttpMethod::Put => write!(f, "PUT"),
            HttpMethod::Delete => write!(f, "DELETE")
        }
    }
}

impl FromStr for HttpMethod {
    type Err = ();

    fn from_str(s: &str) -> Result<HttpMethod, ()> {
        return match s.to_uppercase().as_str() {
            "GET" => Ok(HttpMethod::Get),
            "POST" => Ok(HttpMethod::Post),
            "PUT" => Ok(HttpMethod::Put),
            "DELETE" => Ok(HttpMethod::Delete),
            _ => Err(())
        }
    }
}
pub enum HttpVersion {
    Http09, // HTTP/0.9
    Http10, // HTTP/1.0
    Http11, // HTTP/1.1
    Http2,  // HTTP/2.0
    Http3,  // HTTP/3.0
}

pub struct ParseVersionError;

impl fmt::Display for ParseVersionError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "Versione HTTP non valida o non supportata")
    }
}

impl fmt::Display for HttpVersion {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let version_str = match self {
            HttpVersion::Http09 => "HTTP/0.9",
            HttpVersion::Http10 => "HTTP/1.0",
            HttpVersion::Http11 => "HTTP/1.1",
            HttpVersion::Http2  => "HTTP/2.0",
            HttpVersion::Http3  => "HTTP/3.0",
        };
        write!(f, "{version_str}")
    }
}

impl FromStr for HttpVersion {
    type Err = ();

    fn from_str(s: &str) -> Result<Self, ()> {
        match s.trim() {
            "HTTP/0.9" => Ok(HttpVersion::Http09),
            "HTTP/1.0" => Ok(HttpVersion::Http10),
            "HTTP/1.1" => Ok(HttpVersion::Http11),
            "HTTP/2.0" | "HTTP/2" => Ok(HttpVersion::Http2),
            "HTTP/3.0" | "HTTP/3" => Ok(HttpVersion::Http3),
            _ => Err(()),
        }
    }
}
pub struct Request {
    method: HttpMethod,
    uri: String,
    version: HttpVersion,
    headers: HashMap<String, String>,
    body: Option<String>
}

impl Request {

    pub fn build_from_raw(mut raw_lines: Vec<String>) -> Result<Request, String> {
        let (method, uri, version) = Request::get_request_line_args( &mut raw_lines)?;
        let headers = Request::get_headers(&mut raw_lines)?;
        let body = Request::get_body( &mut raw_lines)?;

        let enum_method = method.parse::<HttpMethod>();

        if let Err(()) = enum_method {
            return Err("Not a valid http method".to_string());
        }

        let enum_version = version.parse::<HttpVersion>();

        if let Err(()) = enum_version {
            return Err("Not a valid http version".to_string());
        }

        let method = enum_method.unwrap();
        let version = enum_version.unwrap();


        return Ok (Request {
            method: method,
            uri: uri,
            version: version,
            headers: headers,
            body: body
        })

    }

    fn get_request_line_args(raw_request: &mut Vec<String>) -> Result<(String, String, String), String> {
       
        let first_line = raw_request.remove(0);
        let mut parts = first_line.split_whitespace();

        let method = parts.next().ok_or("error while reading the request method")?;
        let uri = parts.next().ok_or("error while reading the uri")?;
        let version = parts.next().ok_or("error while reading the version")?;

        Ok((method.to_string(), uri.to_string(), version.to_string()))
    }

    fn get_headers(raw_request: &mut Vec<String>) -> Result<HashMap<String, String>, String> {
        let mut headers = HashMap::new();

        let split_pos = raw_request
            .iter()
            .position(|line| line.trim() == HTTP_WHITE_LINE.trim() || line.is_empty())
            .unwrap_or(raw_request.len());

        
        let header_lines: Vec<String> = raw_request.drain(0..split_pos).collect();

        if !raw_request.is_empty() {
            raw_request.remove(0);
        }

        for line in header_lines {
            if let Some((key, value)) = line.split_once(": ") {
                headers.insert(key.trim().to_string(), value.trim().to_string());
            } else {
                return Err(format!("Riga header non valida: '{line}'"));
            }
        }

        Ok(headers)
    }

    fn get_body(raw_lines: &mut Vec<String>) -> Result<Option<String>, String> {
        todo!()
    }

}
use core::{fmt};
use std::{str::FromStr, write};

const HTTP_WHITE_LINE: &str = "\r\n\r\n";

#[derive(Debug)]
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

#[derive(Debug)]
pub enum HttpVersion {
    Http09, // HTTP/0.9
    Http10, // HTTP/1.0
    Http11, // HTTP/1.1
    Http2,  // HTTP/2.0
    Http3,  // HTTP/3.0
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
#[derive(Debug, Clone, Copy)]
#[repr(u16)]
pub enum HttpStatus {
    Ok = 200,
    Created = 201,
    BadRequest = 400,
    Unauthorized = 401,
    NotFound = 404,
    InternalServerError = 500,
}

impl HttpStatus {
    pub fn as_u16(&self) -> u16 {
        *self as u16
    }

    pub fn canonical_reason(&self) -> &'static str {
        match self {
            HttpStatus::Ok => "OK",
            HttpStatus::Created => "Created",
            HttpStatus::BadRequest => "Bad Request",
            HttpStatus::Unauthorized => "Unauthorized",
            HttpStatus::NotFound => "Not Found",
            HttpStatus::InternalServerError => "Internal Server Error",
        }
    }
}

impl TryFrom<u16> for HttpStatus {
    type Error = ();

    fn try_from(value: u16) -> Result<Self, Self::Error> {
        match value {
            200 => Ok(HttpStatus::Ok),
            201 => Ok(HttpStatus::Created),
            400 => Ok(HttpStatus::BadRequest),
            401 => Ok(HttpStatus::Unauthorized),
            404 => Ok(HttpStatus::NotFound),
            500 => Ok(HttpStatus::InternalServerError),
            _ => Err(()),
        }
    }
}

#[derive(Debug)]
pub enum HttpHeaderName {
    Accept,
    AcceptEncoding,
    AcceptLanguage,
    Authorization,
    Host,
    UserAgent,

    CacheControl,
    ContentLength,
    ContentType,
    ETag,
    Location,
    Server,
    SetCookie,

    AccessControlAllowOrigin,
    AccessControlAllowMethods,
    AccessControlAllowHeaders,

    Custom(String),
}

impl fmt::Display for HttpHeaderName {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let name = match self {
            Self::Accept => "Accept",
            Self::AcceptEncoding => "Accept-Encoding",
            Self::AcceptLanguage => "Accept-Language",
            Self::Authorization => "Authorization",
            Self::Host => "Host",
            Self::UserAgent => "User-Agent",
            Self::CacheControl => "Cache-Control",
            Self::ContentLength => "Content-Length",
            Self::ContentType => "Content-Type",
            Self::ETag => "ETag",
            Self::Location => "Location",
            Self::Server => "Server",
            Self::SetCookie => "Set-Cookie",
            Self::AccessControlAllowOrigin => "Access-Control-Allow-Origin",
            Self::AccessControlAllowMethods => "Access-Control-Allow-Methods",
            Self::AccessControlAllowHeaders => "Access-Control-Allow-Headers",
            Self::Custom(custom) => custom.as_str(),
        };
        write!(f, "{}", name)
    }
}

impl FromStr for HttpHeaderName {
    type Err = std::convert::Infallible; 

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let lower = s.to_ascii_lowercase();

        let header = match lower.as_str() {
            "accept" => Self::Accept,
            "accept-encoding" => Self::AcceptEncoding,
            "accept-language" => Self::AcceptLanguage,
            "authorization" => Self::Authorization,
            "host" => Self::Host,
            "user-agent" => Self::UserAgent,
            "cache-control" => Self::CacheControl,
            "content-length" => Self::ContentLength,
            "content-type" => Self::ContentType,
            "etag" => Self::ETag,
            "location" => Self::Location,
            "server" => Self::Server,
            "set-cookie" => Self::SetCookie,
            "access-control-allow-origin" => Self::AccessControlAllowOrigin,
            "access-control-allow-methods" => Self::AccessControlAllowMethods,
            "access-control-allow-headers" => Self::AccessControlAllowHeaders,
            _ => Self::Custom(s.to_string()),
        };
        Ok(header)
    }
}

#[derive(Debug)]
pub struct Header {
    name: HttpHeaderName,
    value: String
}

impl Header {
    pub fn new(name: HttpHeaderName, value: String) -> Header {

        Header {
            name: name,
            value: value            
        }

    }
}

#[derive(Debug)]
pub struct Request {
    method: HttpMethod,
    uri: String,
    version: HttpVersion,
    headers: Vec<Header>,
    body: Option<String>
}

impl Default for Request {

   fn default() -> Self {
        return Request {
            method: HttpMethod::Get,
            uri: String::from("/"),
            version: HttpVersion::Http11,
            headers: Vec::new(),
            body: None
        }        
    } 

}

impl Request {

    pub fn build_from_raw(mut raw_lines: &mut Vec<String>) -> Result<Request, HttpStatus> {

        if raw_lines.is_empty() {
            return Err(HttpStatus::BadRequest)
        }
        
        let (method, uri, version) = Request::get_request_line_args(&mut raw_lines)?;
        let headers = Request::get_headers(&mut raw_lines)?;

        let method = method.parse::<HttpMethod>().map_err(|_err| HttpStatus::BadRequest)?;
        let version = version.parse::<HttpVersion>().map_err(|_err| HttpStatus::BadRequest)?;

        let body = Request::get_body(&mut raw_lines);

        if Request::check_if_body_is_needed(&headers) {

            if body.is_none() {
                return Err(HttpStatus::BadRequest)
            }

            let content_type_header_option = headers.iter().find(|h| matches!(h.name, HttpHeaderName::ContentType));
            let content_type = content_type_header_option.unwrap();  
           
        }

        return Ok (Request {
            method: method,
            uri: uri,
            version: version,
            headers: headers,
            body: body
        })

    }

    fn get_request_line_args(raw_request: &mut Vec<String>) -> Result<(String, String, String), HttpStatus> {
       
        let first_line = raw_request.remove(0);
        let mut parts = first_line.split_whitespace();

        let method = parts.next().ok_or(HttpStatus::BadRequest)?;
        let uri = parts.next().ok_or(HttpStatus::BadRequest)?;
        let version = parts.next().ok_or(HttpStatus::BadRequest)?;

        Ok((method.to_string(), uri.to_string(), version.to_string()))
    }

    fn get_headers(raw_request: &mut Vec<String>) -> Result<Vec<Header>, HttpStatus> {
        let mut headers: Vec<Header> = Vec::new();

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
                headers.push(Header::new(HttpHeaderName::from_str(key).unwrap(), value.to_string()));
            } else {
                return Err(HttpStatus::BadRequest);
            }
        }

        Ok(headers)
    }

    fn get_body(raw_lines: &mut Vec<String>) -> Option<String> {
        if raw_lines.is_empty() {
            return None;
        }

        let body = raw_lines.drain(..).collect::<Vec<String>>().join("\r\n");

        Some(body)
    }

    fn check_if_body_is_needed(headers: &Vec<Header>) -> bool {

        if let Some(_h) = headers.iter().find(|h| matches!(h.name, HttpHeaderName::ContentType)) {
            return true;
        }

        return false;
    }

}

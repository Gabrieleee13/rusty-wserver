use std::{net::{IpAddr, TcpListener}};

const MAX_WELL_KNOWN_PORTS: usize = 1023;

pub struct Socket {
    ip_address: IpAddr,
    port: usize
}

impl Socket {
    pub fn new(ip: String, port_number: usize) -> Result<Socket, String> {
        let ip_address = ip.parse::<IpAddr>();

        if let Err(parse_error) = ip_address {
            return Err(parse_error.to_string());
        }

        if port_number < MAX_WELL_KNOWN_PORTS {
            return Err(String::from("invalid port number"));
        }

        return Ok(Socket { ip_address: ip_address.unwrap(), port: port_number });
    }

    pub fn bind(&self) -> Result<TcpListener, std::io::Error> {
        let dns = self.ip_address.to_string() + ":" + &self.port.to_string();
        return TcpListener::bind(dns);
    }
}
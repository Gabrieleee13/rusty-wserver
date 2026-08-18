use std::{net::{IpAddr, TcpListener}, todo};

// Constant for check if the port isn't a well known port
const MAX_WELL_KNOWN_PORTS: usize = 1023;

// TODO: implementing the SocketAddr instead of using the ip and port splitted
pub struct Socket {

    // Ip address of the socket
    ip_address: IpAddr,

    // The port used by the socket
    port: usize
}

impl Socket {

    /*
        Method for initializing the Socket 
        If the IP or the port are not valid it will return an Err() with the msg
        Otherwise it will return the Socket wrapped in a Ok()
    */ 
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

    /*
        Method for get the TcpListener, it builds the socket address and returns an Err the bind fails
        Otherwise it return the TcpListener wrapped in an Ok() enum
    */
    pub fn bind(&self) -> Result<TcpListener, std::io::Error> {
        let socket_addr = self.ip_address.to_string() + ":" + &self.port.to_string();
        return TcpListener::bind(socket_addr);
    }
}
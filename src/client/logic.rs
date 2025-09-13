use std::net::{SocketAddr};
use std::error::Error;
use std::net::TcpStream;
use crate::client::client_errors::ClientError;


pub fn get_arguments() -> Result<(String, String), ClientError> {
    let mut args = std::env::args().skip(1); // skip the first argument (program name)
    
    let addr = args.next().ok_or(ClientError::NotArgProvided)?;
    let file = args.next().ok_or(ClientError::NotEnoughArgs)?;

    Ok((addr, file))
}

pub fn parse_ip_port(addr: &str) -> Result<SocketAddr, Box<dyn Error>> {
    let socket_addr: SocketAddr = addr.parse().map_err(|e| Box::new(e) as Box<dyn Error>)?;
    Ok(socket_addr)
}

pub fn make_connection(addr: &SocketAddr) -> Result<TcpStream, Box<dyn Error>> {
    let stream = TcpStream::connect(addr)?;
    Ok(stream)
}// src/client/logic.rs
// Lógica del cliente


mod tests {
    use super::*;
    
    #[test]
    fn test_parse_ip_port_valid() {
        let addr = "127.0.0.1:8080";
        let result = parse_ip_port(addr);
        assert!(result.is_ok());
    }
    
    #[test]
    fn test_parse_ip_port_invalid() {
        let addr = "invalid-address";
        let result = parse_ip_port(addr);
        assert!(result.is_err());
    }
}
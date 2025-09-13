use calculadora_distribuida::server;
use std::env;
enum ServerError {
    AddrNotProvided,
    TooManyArgs,
}

impl std::fmt::Display for ServerError {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        match self {
            ServerError::AddrNotProvided => write!(f, "No address provided"),
            ServerError::TooManyArgs => write!(f, "Too many arguments provided"),
        }
    }
}

fn main() {
    let mut args = env::args();
    args.next(); // skip executable path

    if args.len() > 1 {
        eprintln!("ERROR \"{}\"", ServerError::TooManyArgs);
        return;
    }

    let addr_str = match args.next() {
        Some(a) => a,
        None => {
            eprintln!("ERROR \"{}\"", ServerError::AddrNotProvided);
            return;
        }
    };

    let addr = match server::logic::parse_ip_port(&addr_str) {
        Ok(a) => a,
        Err(e) => {
            eprintln!("ERROR \"{}\"", e);
            return;
        }
    };

    if let Err(e) = server::logic::run(&addr) {
        eprintln!("ERROR \"{}\"", e);
    }
}


#[cfg(test)]
mod tests {
    use super::*;
    use std::net::{IpAddr, Ipv4Addr, SocketAddr};

    #[test]
    fn parse_ipv4_with_port_ok() {
        let s = "127.0.0.1:8080";
        let res = server::logic::parse_ip_port(s);
        assert!(res.is_ok());
        let addr = res.unwrap();
        assert_eq!(addr, SocketAddr::new(IpAddr::V4(Ipv4Addr::new(127, 0, 0, 1)), 8080));
    }

    #[test]
    fn parse_missing_port_err() {
        let s = "127.0.0.1";
        assert!(server::logic::parse_ip_port(s).is_err());
    }

    #[test]
    fn parse_invalid_err() {
        let s = "not an addr";
        assert!(server::logic::parse_ip_port(s).is_err());
}
}

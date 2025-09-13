use crate::server::calculator::Calculator;
use crate::server::sv_protocol::Command;
use std::error::Error;
use std::io::{BufRead, BufReader, Write};
use std::net::{SocketAddr, TcpListener, TcpStream};
use std::sync::{Arc, Mutex};
use std::thread;

pub fn parse_ip_port(addr: &str) -> Result<SocketAddr, Box<dyn Error>> {
    let socket_addr: SocketAddr = addr.parse()?;
    Ok(socket_addr)
}

pub fn run(addr: &SocketAddr) -> Result<(), Box<dyn Error>> {
    let listener = TcpListener::bind(addr)?;
    let calc = Arc::new(Mutex::new(Calculator::new()));
    let mut handles = Vec::new();

    for stream in listener.incoming() {
        let stream = stream?;
        let calc = Arc::clone(&calc);

        let handle = thread::spawn(move || match handle_connection(stream, calc) {
            Ok(_) => {}
            Err(e) => eprintln!("ERROR \"{}\"", e),
        });
        handles.push(handle);
    }

    for handle in handles {
        let _ = handle.join();
    }

    Ok(())
}

fn handle_command(
    command_str: &str,
    calc: Arc<Mutex<Calculator>>,
    stream: &mut TcpStream,
) -> Result<(), Box<dyn Error>> {
    let command = match Command::parse(command_str) {
        Ok(cmd) => cmd,
        Err(e) => {
            let error_message = format!("ERROR \"{}\"\n", e);
            stream.write_all(error_message.as_bytes())?;
            stream.flush()?;
            // Protocol error is recoverable - do not terminate connection
            return Ok(());
        }
    };

    let mut calculator = match calc.lock() {
        Ok(guard) => guard,
        Err(poisoned) => poisoned.into_inner(),
    };

    match command {
        Command::Get => {
            let value = calculator.value();
            stream.write_all(format!("VALUE {}\n", value).as_bytes())?;
            stream.flush()?;
        }
        Command::Op { operator, arg } => match calculator.apply(operator, arg) {
            Ok(()) => stream.write_all(b"OK\n")?,
            Err(e) => {
                let error_message = format!("ERROR \"{}\"\n", e);
                stream.write_all(error_message.as_bytes())?;
                stream.flush()?;
                // CalcError is recoverable - do not terminate connection
                return Ok(());
            }
        },
    }

    Ok(())
}

fn handle_connection(
    mut stream: TcpStream,
    calc: Arc<Mutex<Calculator>>,
) -> Result<(), Box<dyn Error>> {
    let reader_stream = stream.try_clone()?;
    let mut reader = BufReader::new(reader_stream);

    loop {
        let mut buffer = String::new();

        match reader.read_line(&mut buffer) {
            Ok(0) => {
                break;
            }
            Ok(_) => {
                if let Err(e) = handle_command(buffer.trim_end(), Arc::clone(&calc), &mut stream) {
                    eprintln!("ERROR \"{}\"", e);
                    break;
                }
            }
            Err(e) => {
                eprintln!("ERROR \"{}\"", e);
                return Err(Box::new(e));
            }
        }
    }

    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::net::{IpAddr, Ipv4Addr};

    #[test]
    fn test_parse_ip_port_valid_ipv4() {
        let addr = "127.0.0.1:8080";
        let sa = parse_ip_port(addr).expect("should parse valid IPv4 socket address");
        assert_eq!(sa, SocketAddr::new(IpAddr::V4(Ipv4Addr::new(127, 0, 0, 1)), 8080));
    }


    #[test]
    fn test_parse_ip_port_invalid() {
        let addr = "not an addr";
        assert!(parse_ip_port(addr).is_err(), "invalid address should return an error");
    }
}

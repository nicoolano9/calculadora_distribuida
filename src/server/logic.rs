use std::error::Error;
use std::io::{Read, BufRead, Write};
use std::net::{TcpListener, TcpStream};
use crate::server::calculator::Calculator;
use crate::server::sv_protocol::{Command};
use std::thread;
use std::sync::{Arc, Mutex};

pub fn run(addr: &String) -> Result<(), Box<dyn Error>> {
    let listener = TcpListener::bind(addr)?;
    let calc = Arc::new(Mutex::new(Calculator::new()));
    let mut handles = Vec::new();

    for stream in listener.incoming() {
        let stream = stream?;
        let calc = Arc::clone(&calc);


        let handle = thread::spawn(move || {
            match handle_client(stream, calc) {
                Ok(_) => {},
                Err(e) => eprintln!("ERROR \"{}\"", e),
            }
        });
        handles.push(handle);
    }

    for handle in handles {
        let _ = handle.join();
    }

    Ok(())
}

fn handle_client(mut stream: TcpStream, calc: Arc<Mutex<Calculator>>) -> Result<(), Box<dyn Error>> {
    let reader_stream = stream.try_clone()?;
    let mut reader = std::io::BufReader::new(reader_stream);
    
    loop {
        let mut buffer = String::new();
        let bytes_read = reader.read_line(&mut buffer)?;
        
        // Check if the connection was closed
        if bytes_read == 0 {
            break;
        }

        let command = match Command::parse(&buffer.trim()) {
            Ok(cmd) => cmd,
            Err(e) => {
                let error_response = format!("ERROR {}\n", e);
                stream.write_all(error_response.as_bytes())?;
                continue;
            }
        };
        
        let cmd_str = buffer.trim().to_string();

        let mut calculator = match calc.lock() {
                Ok(guard) => guard,
                Err(poisoned) => poisoned.into_inner(),
            };

        let response = {
            match command {
                Command::Get => {
                    let value = calculator.value();
                    format!("OK {}\n", value)
                },
                Command::Op { operator, arg } => {
                    match calculator.apply(operator, arg) {
                        Ok(()) => format!("OK\n"),
                        Err(e) => format!("ERROR {}\n", e),
                    }
                }
            }
        };

        stream.write_all(response.as_bytes())?;
    }

    Ok(())
}
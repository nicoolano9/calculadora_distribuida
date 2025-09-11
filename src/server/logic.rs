use std::error::Error;
use std::io::{BufReader, BufRead, Write};
use std::net::{TcpListener, TcpStream};
use crate::server::calculator::Calculator;
use crate::server::sv_protocol::{Command};
use crate::server::sv_protocol_errors::ProtocolError;
use crate::server::calculator_errors::CalcError;
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
            match handle_connection(stream, calc) {
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

fn handle_command(command_str: &str, calc: Arc<Mutex<Calculator>>, stream: &mut TcpStream) -> Result<(), Box<dyn Error>> {
    let command = Command::parse(command_str)?;

    let mut calculator = match calc.lock() {
        Ok(guard) => guard,
        Err(poisoned) => poisoned.into_inner(),
    };

    match command {
        Command::Get => {
            let value = calculator.value();
            stream.write_all(format!("VALUE {}\n", value).as_bytes())?;
            stream.flush()?;
        },
        Command::Op { operator, arg } => {
            match calculator.apply(operator, arg) {
                Ok(()) => stream.write_all(b"OK\n")?,
                Err(e) => {
                    let error_message = format!("ERROR {}\n", e);
                    stream.write_all(error_message.as_bytes())?;
                    stream.flush()?;
                    return Err(Box::new(e));
                }
            }
        }
    }

    Ok(())
}

fn handle_connection(mut stream: TcpStream, calc: Arc<Mutex<Calculator>>) -> Result<(), Box<dyn Error>> {
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
                    // If the error is due to protocol or calculation, continue processing further commands
                    // Otherwise, break the loop and close the connection
                    if e.as_ref().downcast_ref::<ProtocolError>().is_some() {
                        continue;
                    }
                    if e.as_ref().downcast_ref::<CalcError>().is_some() {
                        continue;
                    }
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
use calculadora_distribuida::client;
use std::fs::File;

fn main() {
    let (addr, file) = match client::logic::get_arguments() {
        Ok((a, f)) => (a, f),
        Err(e) => {
            eprintln!("ERROR \"{}\"", e);
            return;
        }
    };

    let socket_addr = match client::logic::parse_ip_port(&addr) {
        Ok(a) => a,
        Err(e) => {
            eprintln!("ERROR \"{}\"", e);
            return;
        }
    };

    let f = match File::open(&file) {
        Ok(f) => f,
        Err(e) => {
            eprintln!("ERROR \"{}\"", e);
            return;
        }
    };
    
}

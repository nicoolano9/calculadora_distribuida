use std::f32::consts::E;
use std::io;
use std::net::{TcpListener, TcpStream};
use std::thread;
use std::sync::{Arc, Mutex};
use calculadora_distribuida::server;

fn main() {
    let addr = "127.0.0.1:8080".to_string();

    if let Err(e) = server::logic::run(&addr) {
        eprintln!("ERROR \"{}\"", e);
    }
}
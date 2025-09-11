use calculadora_distribuida::server;

fn main() {
    let addr = "127.0.0.1:8080".to_string();

    if let Err(e) = server::logic::run(&addr) {
        eprintln!("ERROR \"{}\"", e);
    }

}
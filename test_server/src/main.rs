use std::env;
use std::net::TcpListener;

fn main() {
    let target_port: String = env::var("CLIENT_PORT").unwrap_or("3001".to_string());
    let target_binding = format!("0.0.0.0:{}", target_port);
    let listener = TcpListener::bind(target_binding).unwrap();

    for stream in listener.incoming() {
        let _ = stream.unwrap();
        println!("Connection established!");
    }
}

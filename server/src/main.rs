use core::entity::Entity;
use std::collections::HashMap;
use std::io::prelude::*;
use std::net::{TcpListener, TcpStream};
use std::thread;

struct Server {
    entities: HashMap<String, Entity>,
}
fn handle_client(mut stream: TcpStream) {
    println!("Client connected!");
    loop {
        let mut buffer = [0; 512];
        match stream.read(&mut buffer) {
            Ok(0) => break,
            Err(e) => println!("Error reading from client: {}", e),
            _ => (),
        };
        let packet = String::from_utf8_lossy(&buffer[..]);
        println!("Request: {}", packet);
    }

    println!("Client disconnected!");
}

fn main() -> std::io::Result<()> {
    let listener = TcpListener::bind("127.0.0.1:3030")?;

    for stream in listener.incoming() {
        let stream = stream.unwrap();
        thread::spawn(move || {
            handle_client(stream);
        });
    }

    Ok(())
}

use core::entity::Entity;
use core::io::Event;
use std::collections::HashMap;
use std::io::prelude::*;
use std::net::{TcpListener, TcpStream};
use std::sync::mpsc;
use std::sync::Mutex;
use std::thread;

struct Client {
    sender: mpsc::Sender<Event>,
    receiver: mpsc::Receiver<Event>,
}

impl Client {
    fn handle_io(&self) {}
}

struct Server<'a> {
    listener: TcpListener,
    entities: HashMap<&'a str, Entity>,
    clients: HashMap<&'a str, Client>,
}

impl Server<'_> {
    // Note: blocking
    fn listen_for_new_clients(&self) {}

    fn handle_client_io(&self, stream: TcpStream) {
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
}

fn main() -> std::io::Result<()> {
    let listener = TcpListener::bind("127.0.0.1:3030")?;
    let entities: HashMap<&str, Entity> = HashMap::new();
    let clients: HashMap<&str, Client> = HashMap::new();

    let server = Server {
        entities,
        clients,
        listener,
    };

    thread::spawn(move || {
        for stream in listener.incoming() {
            let stream = stream.unwrap();
            thread::spawn(move || {
                handle_client_io(stream);
            });
            let (sender, receiver) = mpsc::channel();
            let client = Client {
                sender: sender,
                receiver: receiver,
            };
        }
    });

    Ok(())
}

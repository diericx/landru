use std::io::prelude::*;
use std::net::{TcpListener, TcpStream};
use std::io::{BufReader, Result};
use std::thread;
use uuid::Uuid;

mod room;
use crate::room::Room;

struct Server {
}

impl Server {
    fn bind(room: Room) -> Result<()> {
        let listener = TcpListener::bind("127.0.0.1:3030")?;
        for stream in listener.incoming() {
            let stream = stream.unwrap();
            let room = room.clone();
            let _       = thread::spawn(move || Server::create_client(room, stream));
        } Ok(())
    }

    fn create_client(room: Room, stream: TcpStream) {
        let mut reader   = BufReader::new(stream.try_clone().unwrap());
        let id = Uuid::new_v4().to_string(); 
        room.join(id, stream);

        let mut buffer   = String::new();
        while let Ok(_) = reader.read_line(&mut buffer) {
            room.broadcast(&buffer);
            buffer.clear();
        }
        println!("Error reading from client")
    }
}

fn main() -> std::io::Result<()> {
    let room = Room::new();
    let server = Server::bind(room);

    Ok(())
}

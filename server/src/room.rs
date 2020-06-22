use std::sync::{Arc, Mutex};
use std::collections::HashMap;
use std::io::prelude::*;
use std::net::TcpStream;

#[derive(Clone)]
pub struct Room {
    dict : Arc<Mutex<HashMap<String, TcpStream>>>
}

impl Room {
    pub fn new() -> Room {
        Room {
            dict: Arc::new(Mutex::new(HashMap::new()))
        }
    }

    pub fn join(&self, id: String, stream: TcpStream) {
        let mut dict = self.dict.lock().unwrap();
        if !dict.contains_key(&id) {
            dict.insert(id, stream);
        }
    }

    pub fn broadcast(&self, msg: &str) {
        let dict = self.dict.lock().unwrap();
        for (id, stream) in dict.iter() {
            let mut stream = stream.try_clone().unwrap();
            let packet = format!("{}: {}", id, msg);
            match stream.write(packet.as_bytes()) {
                Err(e) => println!("Error sending message to client: {}", e),
                _ => ()
            }
        }
    }
}
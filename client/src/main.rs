use core::entity;
use std::io::prelude::*;
use std::net::TcpStream;

fn main() -> std::io::Result<()> {
    let entity = entity::Entity::new(entity::Kind::Player);
    let serialized: String = entity.serialize();
    println!("{:#?}", entity);

    let test = core::io::Event::New(entity);
    let serialized_test = test.serialize();
    println!("{:#?}", serialized_test);

    let mut stream = TcpStream::connect("127.0.0.1:3030")?;
    stream.write(serialized.as_bytes()).unwrap();

    Ok(())
}

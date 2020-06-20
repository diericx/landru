use core::entity;

fn main() {
    let entity = entity::Basic::new();
    let serialized: String = entity.serialize();
    println!("New entity kind: {:#?}", serialized);
}

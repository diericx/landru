use crate::entity::Entity;
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug)]
pub enum Event {
    New(Entity),
    Update(Entity),
    ServerMsg(String),
}

impl Event {
    pub fn serialize(&self) -> String {
        use crate::io::Event::{New, ServerMsg, Update};

        match self {
            New(e) => e.serialize(),
            Update(e) => e.serialize(),
            // TODO: This seems wrong... either the return value of serialize should be a read only borrow
            // or this should transfer ownership
            ServerMsg(msg) => msg.clone(),
        }
    }
}

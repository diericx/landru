use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug)]
pub enum Kind {
    Player,
    Grass,
    Rock,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct Point {
    x: i32,
    y: i32,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct Entity {
    id: String,
    kind: Kind,
    pos: Point,
}
use uuid::Uuid;
impl Entity {
    pub fn new(kind: Kind) -> Entity {
        return Entity {
            id: Uuid::new_v4().to_string(),
            kind,
            pos: Point { x: 0, y: 0 },
        };
    }

    pub fn pos(&self) -> &Point {
        &self.pos
    }

    pub fn kind(&self) -> &Kind {
        &self.kind
    }

    pub fn serialize(&self) -> String {
        serde_json::to_string(self).unwrap()
    }
}

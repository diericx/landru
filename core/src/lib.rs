pub mod entity {
    use serde::{Deserialize, Serialize};

    #[derive(Serialize, Deserialize, Debug)]
    pub enum Kind {
        Basic,
    }

    #[derive(Serialize, Deserialize, Debug)]
    pub struct Point {
        x: i32,
        y: i32,
    }

    #[derive(Serialize, Deserialize, Debug)]
    pub struct Basic {
        kind: crate::entity::Kind,
        pos: Point,
    }

    impl Basic {
        pub fn new() -> Basic {
            return Basic {
                kind: Kind::Basic,
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
}

#[cfg(test)]
mod tests {
    #[test]
    fn it_works() {
        assert_eq!(2 + 2, 4);
    }
}

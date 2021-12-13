use std::collections::HashMap;

use strum::IntoEnumIterator;
use strum_macros::EnumIter;

use crate::Direction;

#[derive(Debug, EnumIter)]
enum Types {
    AIR,
    GRASS,
    DIRT,
    STONE,
    COBBLESTONE,
}

#[derive(Debug)]
pub struct Type {
    pub name: String,
    pub id: u16,
    pub texture_ids: HashMap<Direction, u16>,
}

impl Types {
    pub fn get(&self) -> Type {
        match self {
            Types::AIR => Type {
                name: String::from("air"),
                id: 0,
                texture_ids: HashMap::from([
                    (Direction::UP,     0),
                    (Direction::DOWN,   0),
                    (Direction::NORTH,  0),
                    (Direction::SOUTH,  0),
                    (Direction::WEST,   0),
                    (Direction::EAST,   0),
                ]),
            },
            Types::GRASS => Type {
                name: String::from("grass"),
                id: 1,
                texture_ids: HashMap::from([
                    (Direction::UP,     1),
                    (Direction::DOWN,   2),
                    (Direction::NORTH,  0),
                    (Direction::SOUTH,  0),
                    (Direction::WEST,   0),
                    (Direction::EAST,   0),
                ]),
            },
            Types::DIRT => Type {
                name: String::from("dirt"),
                id: 2,
                texture_ids: HashMap::from([
                    (Direction::UP,     2),
                    (Direction::DOWN,   2),
                    (Direction::NORTH,  2),
                    (Direction::SOUTH,  2),
                    (Direction::WEST,   2),
                    (Direction::EAST,   2),
                ]),
            },
            Types::STONE => Type {
                name: String::from("stone"),
                id: 3,
                texture_ids: HashMap::from([
                    (Direction::UP,     3),
                    (Direction::DOWN,   3),
                    (Direction::NORTH,  3),
                    (Direction::SOUTH,  3),
                    (Direction::WEST,   3),
                    (Direction::EAST,   3),
                ]),
            },
            Types::COBBLESTONE => Type {
                name: String::from("cobblestone"),
                id: 4,
                texture_ids: HashMap::from([
                    (Direction::UP,     4),
                    (Direction::DOWN,   4),
                    (Direction::NORTH,  4),
                    (Direction::SOUTH,  4),
                    (Direction::WEST,   4),
                    (Direction::EAST,   4),
                ]),
            },
        }
    }
}

pub fn get(id: u16) -> Type {
    for block_type in Types::iter() {
        let the_type = block_type.get();
        if (block_type as u16) == id {
            return the_type;
        }
    }
    panic!("Block type does not exist!");
}
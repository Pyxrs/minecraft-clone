use std::collections::HashMap;

use once_cell::sync::OnceCell;

use crate::Direction;

pub static BLOCK_TYPES: OnceCell<[Type; 4]> = OnceCell::new();

#[derive(Debug)]
pub struct Type<'a> {
    pub name: &'a str,
    pub id: u16,
    pub texture_ids: HashMap<Direction, u16>,
}

pub fn init() {
    match BLOCK_TYPES.set([

        // ==========
        // BLOCKS
        Type {
            name: "grass",
            id: 0,
            texture_ids: HashMap::from([
                (Direction::UP,     1),
                (Direction::DOWN,   2),
                (Direction::NORTH,  0),
                (Direction::SOUTH,  0),
                (Direction::WEST,   0),
                (Direction::EAST,   0),
            ]),
        },
        Type {
            name: "dirt",
            id: 1,
            texture_ids: HashMap::from([
                (Direction::UP,     2),
                (Direction::DOWN,   2),
                (Direction::NORTH,  2),
                (Direction::SOUTH,  2),
                (Direction::WEST,   2),
                (Direction::EAST,   2),
            ]),
        },
        Type {
            name: "stone",
            id: 2,
            texture_ids: HashMap::from([
                (Direction::UP,     3),
                (Direction::DOWN,   3),
                (Direction::NORTH,  3),
                (Direction::SOUTH,  3),
                (Direction::WEST,   3),
                (Direction::EAST,   3),
            ]),
        },
        Type {
            name: "cobblestone",
            id: 3,
            texture_ids: HashMap::from([
                (Direction::UP,     4),
                (Direction::DOWN,   4),
                (Direction::NORTH,  4),
                (Direction::SOUTH,  4),
                (Direction::WEST,   4),
                (Direction::EAST,   4),
            ]),
        },
        // ==========

    ]) {
        Ok(_) => {}
        Err(e) => {
            panic!("Failed to initialize block types: {:?}", e);
        }
    };
}
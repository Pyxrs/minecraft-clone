use cgmath::Vector3;

#[derive(Hash, Eq, PartialEq, Debug, Clone, Copy)]
pub enum Direction {
    UP,
    DOWN,
    NORTH,
    WEST,
    SOUTH,
    EAST
}

impl Direction {
    pub fn get(id: u8) -> Direction {
        match id {
            0 => Direction::UP,
            1 => Direction::DOWN,
            2 => Direction::NORTH,
            3 => Direction::SOUTH,
            4 => Direction::WEST,
            5 => Direction::EAST,
            _ => panic!("Invalid ID!"),
        }
    }
    pub fn get_vec(&self) -> Vector3<i8> {
        match self {
            Direction::UP => Vector3::new(0, 1, 0),
            Direction::DOWN => Vector3::new(0, -1, 0),
            Direction::NORTH => Vector3::new(0, 0, -1),
            Direction::SOUTH => Vector3::new(0, 0, 1),
            Direction::WEST => Vector3::new(1, 0, 0),
            Direction::EAST => Vector3::new(-1, 0, 0),
        }
    }
    pub fn get_id(&self) -> u8 {
        match self {
            Direction::UP => 0,
            Direction::DOWN => 1,
            Direction::NORTH => 2,
            Direction::SOUTH => 3,
            Direction::WEST => 4,
            Direction::EAST => 5,
        }
    }
    pub fn get_string(&self) -> String {
        match self {
            Direction::UP => String::from("up"),
            Direction::DOWN => String::from("down"),
            Direction::NORTH => String::from("north"),
            Direction::SOUTH => String::from("south"),
            Direction::WEST => String::from("west"),
            Direction::EAST => String::from("east"),
        }
    }
}
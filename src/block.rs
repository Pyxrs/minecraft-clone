use cgmath::Vector3;

use crate::block_types::Type;
use crate::quad::Quad;
use crate::Direction;

#[derive(Clone, Copy)]
pub struct Block {
    id: u16,
}

impl Block {
    pub fn new(block_type: Type) -> Self {
        Block {
            id: block_type.id
        }
    }

    pub fn render(&self, pos: Vector3<i32>, offset: u16) -> [Quad; 6] {
        [
            Quad::new(self.get_id(), Direction::UP,  Vector3::new(pos.x as f32, pos.y as f32 + 0.5, pos.z as f32), 0 + (offset * 20)),
            Quad::new(self.get_id(), Direction::DOWN,  Vector3::new(pos.x as f32, pos.y as f32 - 0.5, pos.z as f32), 4 + (offset * 20)),
            Quad::new(self.get_id(), Direction::NORTH, Vector3::new(pos.x as f32, pos.y as f32 as f32, pos.z as f32 - 0.5), 8 + (offset * 20)),
            Quad::new(self.get_id(), Direction::SOUTH, Vector3::new(pos.x as f32, pos.y as f32, pos.z as f32 + 0.5), 12 + (offset * 20)),
            Quad::new(self.get_id(), Direction::WEST, Vector3::new(pos.x as f32 + 0.5, pos.y as f32, pos.z as f32), 16 + (offset * 20)),
            Quad::new(self.get_id(), Direction::EAST, Vector3::new(pos.x as f32 - 0.5, pos.y as f32, pos.z as f32), 20 + (offset * 20)),
        ]
    }

    pub fn get_id(&self) -> u16 {
        self.id
    }
}
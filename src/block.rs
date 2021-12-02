use cgmath::Vector3;

use crate::quad::Quad;
use crate::Direction;

pub struct Block {
    id: u16,
    position: Vector3<f32>,
    mesh: [Quad; 6],
}

impl Block {
    pub fn new(id: u16, pos: Vector3<f32>, offset: u16) -> Self {
        Block {
            id,
            position: pos,
            mesh: [
                Quad::new(id, Direction::UP,  Vector3::new(pos.x, pos.y + 0.5, pos.z), 0 + (offset * 20)),
                Quad::new(id, Direction::DOWN,  Vector3::new(pos.x, pos.y - 0.5, pos.z), 4 + (offset * 20)),
                Quad::new(id, Direction::NORTH, Vector3::new(pos.x, pos.y, pos.z - 0.5), 8 + (offset * 20)),
                Quad::new(id, Direction::SOUTH, Vector3::new(pos.x, pos.y, pos.z + 0.5), 12 + (offset * 20)),
                Quad::new(id, Direction::WEST, Vector3::new(pos.x + 0.5, pos.y, pos.z), 16 + (offset * 20)),
                Quad::new(id, Direction::EAST, Vector3::new(pos.x - 0.5, pos.y, pos.z), 20 + (offset * 20)),
            ],
        }
    }

    pub fn get_mesh(&self) -> &[Quad; 6] {
        &self.mesh
    }
}
use cgmath::Vector3;

use crate::block_types::Type;
use crate::quad::Quad;
use crate::Direction;

#[derive(Copy, Clone)]
pub struct BlockData {
    id: u16,
}

impl BlockData {
    pub fn new(id: u16) -> BlockData {
        BlockData { id }
    }

    pub fn get_id(&self) -> u16 {
        self.id
    }
}

pub struct Block {
    id: u16,
    position: Vector3<i32>,
    mesh: [Quad; 6],
}

impl Block {
    pub fn new_I(id: u16, pos: Vector3<i32>, offset: u16) -> Self {
        Block {
            id,
            position: pos,
            mesh: [
                Quad::new(id, Direction::UP,  Vector3::new(pos.x as f32, pos.y as f32 + 0.5, pos.z as f32), 0 + (offset * 20)),
                Quad::new(id, Direction::DOWN,  Vector3::new(pos.x as f32, pos.y as f32 - 0.5, pos.z as f32), 4 + (offset * 20)),
                Quad::new(id, Direction::NORTH, Vector3::new(pos.x as f32, pos.y as f32 as f32, pos.z as f32 - 0.5), 8 + (offset * 20)),
                Quad::new(id, Direction::SOUTH, Vector3::new(pos.x as f32, pos.y as f32, pos.z as f32 + 0.5), 12 + (offset * 20)),
                Quad::new(id, Direction::WEST, Vector3::new(pos.x as f32 + 0.5, pos.y as f32, pos.z as f32), 16 + (offset * 20)),
                Quad::new(id, Direction::EAST, Vector3::new(pos.x as f32 - 0.5, pos.y as f32, pos.z as f32), 20 + (offset * 20)),
            ],
        }
    }
    pub fn new_D(data: &BlockData, pos: Vector3<i32>, offset: u16) -> Self {
        let id = data.get_id();
        Block {
            id,
            position: pos,
            mesh: [
                Quad::new(id, Direction::UP,  Vector3::new(pos.x as f32, pos.y as f32 + 0.5, pos.z as f32), 0 + (offset * 20)),
                Quad::new(id, Direction::DOWN,  Vector3::new(pos.x as f32, pos.y as f32 - 0.5, pos.z as f32), 4 + (offset * 20)),
                Quad::new(id, Direction::NORTH, Vector3::new(pos.x as f32, pos.y as f32 as f32, pos.z as f32 - 0.5), 8 + (offset * 20)),
                Quad::new(id, Direction::SOUTH, Vector3::new(pos.x as f32, pos.y as f32, pos.z as f32 + 0.5), 12 + (offset * 20)),
                Quad::new(id, Direction::WEST, Vector3::new(pos.x as f32 + 0.5, pos.y as f32, pos.z as f32), 16 + (offset * 20)),
                Quad::new(id, Direction::EAST, Vector3::new(pos.x as f32 - 0.5, pos.y as f32, pos.z as f32), 20 + (offset * 20)),
            ],
        }
    }
    pub fn new_T(block_type: Type, pos: Vector3<i32>, offset: u16) -> Self {
        let id = block_type.id;
        Block {
            id,
            position: pos,
            mesh: [
                Quad::new(id, Direction::UP,  Vector3::new(pos.x as f32, pos.y as f32 + 0.5, pos.z as f32), 0 + (offset * 20)),
                Quad::new(id, Direction::DOWN,  Vector3::new(pos.x as f32, pos.y as f32 - 0.5, pos.z as f32), 4 + (offset * 20)),
                Quad::new(id, Direction::NORTH, Vector3::new(pos.x as f32, pos.y as f32 as f32, pos.z as f32 - 0.5), 8 + (offset * 20)),
                Quad::new(id, Direction::SOUTH, Vector3::new(pos.x as f32, pos.y as f32, pos.z as f32 + 0.5), 12 + (offset * 20)),
                Quad::new(id, Direction::WEST, Vector3::new(pos.x as f32 + 0.5, pos.y as f32, pos.z as f32), 16 + (offset * 20)),
                Quad::new(id, Direction::EAST, Vector3::new(pos.x as f32 - 0.5, pos.y as f32, pos.z as f32), 20 + (offset * 20)),
            ],
        }
    }

    pub fn get_mesh(&self) -> &[Quad; 6] {
        &self.mesh
    }

    pub fn get_pos(&self) -> &Vector3<i32> {
        &self.position
    }

    pub fn get_id(&self) -> u16 {
        self.id
    }
}
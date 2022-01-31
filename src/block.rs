use cgmath::Vector3;

use crate::block_types::Type;
use crate::quad::Quad;
use crate::direction::Direction;

#[derive(Clone, Copy, Debug)]
pub struct Block {
    id: u16,
}

impl Block {
    pub fn new(block_type: &Type) -> Self {
        Block {
            id: block_type.id
        }
    }

    pub fn render(&self, visible: [bool; 6], pos: Vector3<i32>, offset: u32) -> Vec<Quad> {
        let mut quads: Vec<Quad> = Vec::new();
        let mut total = 0;
        for v in visible.iter() {
            if *v {
                total += 1;
            }
        }
        if total > 0 {
            total -= 1;
        }
        total *= 4;
        if visible[0] {
            quads.push(Quad::new(self.get_id(), Direction::UP,  Vector3::new(pos.x as f32, pos.y as f32 + 0.5, pos.z as f32), (quads.len() as u32 * 4) + (offset * total)));
        } if visible[1] {
            quads.push(Quad::new(self.get_id(), Direction::DOWN,  Vector3::new(pos.x as f32, pos.y as f32 - 0.5, pos.z as f32), (quads.len() as u32 * 4) + (offset * total)));
        } if visible[2] {
            quads.push(Quad::new(self.get_id(), Direction::NORTH, Vector3::new(pos.x as f32, pos.y as f32 as f32, pos.z as f32 - 0.5), (quads.len() as u32 * 4) + (offset * total)));
        } if visible[3] {
            quads.push(Quad::new(self.get_id(), Direction::SOUTH, Vector3::new(pos.x as f32, pos.y as f32, pos.z as f32 + 0.5), (quads.len() as u32 * 4) + (offset * total)));
        } if visible[4] {
            quads.push(Quad::new(self.get_id(), Direction::WEST, Vector3::new(pos.x as f32 + 0.5, pos.y as f32, pos.z as f32), (quads.len() as u32 * 4) + (offset * total)));
        } if visible[5] {
            quads.push(Quad::new(self.get_id(), Direction::EAST, Vector3::new(pos.x as f32 - 0.5, pos.y as f32, pos.z as f32), (quads.len() as u32 * 4) + (offset * total)));
        }
        quads
    }

    pub fn get_id(&self) -> u16 {
        self.id
    }
}
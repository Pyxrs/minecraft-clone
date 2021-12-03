
use cgmath::Vector3;

use crate::vertex::Vertex;
use crate::block::{Block, BlockData};

const SIZE: i32 = 13;

pub struct Chunk {
    pos: Vector3<i32>,
    block_index: [BlockData; 2730],
}

impl Chunk {
    pub fn new(pos: Vector3<i32>) -> Self {
        Self {
            pos,
            block_index: [BlockData::new(1); 2730],
        }
    }

    pub fn build(&self) -> (Vec<Vertex>, Vec<u16>) {
        let mut vertices: Vec<Vertex> = Vec::new();
        let mut indices: Vec<u16> = Vec::new();
        for i in 0..(self.block_index.len()) {
            let block = Block::new_D(&self.block_index[i], index_r(i), i as u16);
            for quad in block.get_mesh().iter() {
                vertices.extend(quad.get_vertices().iter());
                indices.extend(quad.get_indices(i as u16).iter());
            }
        }
        return (vertices, indices);
    }
}

fn index(pos: Vector3<i32>) -> usize {
    ((pos.x * (SIZE * SIZE)) + (pos.y * SIZE) + pos.z) as usize
}
fn index_r(index: usize) -> Vector3<i32> {
    Vector3::new(
        index as i32 / (SIZE * SIZE),
        (index as i32 % (SIZE * SIZE)) / SIZE,
        (index as i32 % (SIZE * SIZE)) % SIZE,
    )
}
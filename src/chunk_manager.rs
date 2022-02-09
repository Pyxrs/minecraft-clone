use cgmath::Vector3;

use crate::chunk::{Chunk, self};

pub struct ChunkManager {
    pub chunks: Vec<Chunk>
}

impl ChunkManager {
    pub fn new() -> ChunkManager {
        ChunkManager {
            chunks: Vec::new()
        }
    }

    pub fn add_chunk(&mut self, chunk: Chunk) {
        self.chunks.push(chunk);
    }

    pub fn get_chunk(&self, index: usize) -> &Chunk {
        &self.chunks[index]
    }
    pub fn get_chunk_mut(&mut self, index: usize) -> &mut Chunk {
        &mut self.chunks[index]
    }

    pub fn get_pos_chunk(&mut self, pos: Vector3<i32>) -> Option<&mut Chunk> {
        for chunk in &mut self.chunks {
            if pos.x < chunk.position.x + (chunk::SIZE / 2) as i32 && pos.x >= chunk.position.x - (chunk::SIZE / 2) as i32 &&
               pos.y < chunk.position.y + (chunk::SIZE / 2) as i32 && pos.y >= chunk.position.y - (chunk::SIZE / 2) as i32 &&
               pos.z < chunk.position.z + (chunk::SIZE / 2) as i32 && pos.z >= chunk.position.z - (chunk::SIZE / 2) as i32 {
                return Some(chunk);
            }
        }
        None
    }

    pub fn get_pos_index(&mut self, pos: Vector3<i32>) -> Option<usize> {
        for index in 0..self.chunks.len() {
            let chunk = &self.chunks[index];
            if pos.x < chunk.position.x + (chunk::SIZE / 2) as i32 && pos.x >= chunk.position.x - (chunk::SIZE / 2) as i32 &&
               pos.y < chunk.position.y + (chunk::SIZE / 2) as i32 && pos.y >= chunk.position.y - (chunk::SIZE / 2) as i32 &&
               pos.z < chunk.position.z + (chunk::SIZE / 2) as i32 && pos.z >= chunk.position.z - (chunk::SIZE / 2) as i32 {
                return Some(index);
            }
        }
        None
    }

    pub fn get_chunk_count(&self) -> usize {
        self.chunks.len()
    }

    pub fn set_block(&mut self, pos: Vector3<i32>, block: u16) {
        let chunk = self.get_pos_chunk(pos);
        match chunk {
            Some(_) => {
                chunk.unwrap().set_block_global(pos, block);
            }
            None => {}
        }
    }
}
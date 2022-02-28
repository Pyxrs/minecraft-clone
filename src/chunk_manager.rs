use std::collections::HashMap;

use cgmath::{Vector3, Point3};
use rand::{Rng, prelude::ThreadRng};

use crate::{chunk::{Chunk, self}, math, render::chunk_buffers::ChunkBuffers, RENDER_DISTANCE};

pub struct ChunkManager {
    pub chunks: Vec<Chunk>,
    random: ThreadRng
}

impl ChunkManager {
    pub fn new() -> ChunkManager {
        ChunkManager {
            chunks: Vec::new(),
            random: rand::thread_rng()
        }
    }

    pub fn update(&mut self, player_pos: Point3<i32>, chunk_buffers: &mut ChunkBuffers, device: &wgpu::Device) {
        for posx in (-RENDER_DISTANCE + math::get_chunk_position(player_pos).x)..(RENDER_DISTANCE + math::get_chunk_position(player_pos).x) {
            for posy in (-RENDER_DISTANCE + math::get_chunk_position(player_pos).y)..(RENDER_DISTANCE + math::get_chunk_position(player_pos).y) {
                for posz in (-RENDER_DISTANCE + math::get_chunk_position(player_pos).z)..(RENDER_DISTANCE + math::get_chunk_position(player_pos).z) {
                    if self.get_pos_chunk(player_pos).is_none() {
                        let chunk = Chunk::new_perlin(Point3::new(posx, posy, posz), self.random.gen_range(1..4));
                        let index = self.add_chunk(chunk);
                        chunk_buffers.update_chunk(device, index, self.chunks.get(index).unwrap());
                    }
                }
            }
        }   
    }
    
    pub fn add_chunk(&mut self, chunk: Chunk) -> usize {
        let index = self.chunks.len();
        self.chunks.push(chunk);
        index
    }

    pub fn remove_chunk(&mut self, index: usize) {
        self.chunks.remove(index);
    }

    pub fn get_chunk(&self, index: usize) -> &Chunk {
        &self.chunks[index]
    }
    pub fn get_chunk_mut(&mut self, index: usize) -> &mut Chunk {
        &mut self.chunks[index]
    }

    pub fn get_pos_chunk(&self, pos: Point3<i32>) -> Option<&Chunk> {
        for chunk in &self.chunks {
            if pos.x < chunk.position.x + (chunk::SIZE / 2) as i32 && pos.x >= chunk.position.x - (chunk::SIZE / 2) as i32 &&
               pos.y < chunk.position.y + (chunk::SIZE / 2) as i32 && pos.y >= chunk.position.y - (chunk::SIZE / 2) as i32 &&
               pos.z < chunk.position.z + (chunk::SIZE / 2) as i32 && pos.z >= chunk.position.z - (chunk::SIZE / 2) as i32 {
                return Some(&chunk);
            }
        }
        None
    }
    pub fn get_pos_chunk_mut(&mut self, pos: Point3<i32>) -> Option<&mut Chunk> {
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

    /*pub fn get_index_pos(&mut self, index: usize) -> Option<Vector3<i32>> {
        for chunk in self.chunks {
            let chunk = &self.chunks[index];
            if pos.x < chunk.position.x + (chunk::SIZE / 2) as i32 && pos.x >= chunk.position.x - (chunk::SIZE / 2) as i32 &&
               pos.y < chunk.position.y + (chunk::SIZE / 2) as i32 && pos.y >= chunk.position.y - (chunk::SIZE / 2) as i32 &&
               pos.z < chunk.position.z + (chunk::SIZE / 2) as i32 && pos.z >= chunk.position.z - (chunk::SIZE / 2) as i32 {
                return Some(index);
            }
        }
        None
    }*/

    pub fn get_chunk_count(&self) -> usize {
        self.chunks.len()
    }

    pub fn set_block(&mut self, pos: Point3<i32>, block: u16) {
        let chunk = self.get_pos_chunk_mut(pos);
        match chunk {
            Some(_) => {
                chunk.unwrap().set_block_global(pos, block);
            }
            None => {}
        }
    }

    pub fn get_block(&self, pos: Point3<i32>) -> u16 {
        let chunk = self.get_pos_chunk(pos);
        match chunk {
            Some(_) => {
                return chunk.unwrap().get_block_global(pos)
            }
            None => 0
        }
    }
}
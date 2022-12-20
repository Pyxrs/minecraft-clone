use cgmath::{Point3};
use noise::NoiseFn;

use crate::PERLIN;

pub const SIZE: u8 = 16;

#[derive(PartialEq)]
pub struct Chunk {
    pub position: Point3<i32>,
    pub blocks: [[[u32; SIZE as usize]; SIZE as usize]; SIZE as usize],
}

impl Chunk {
    pub fn new_empty(pos: Point3<i32>) -> Chunk {
        Chunk {
            position: pos * SIZE as i32,
            blocks: [[[0; SIZE as usize]; SIZE as usize]; SIZE as usize],
        }
    }
    pub fn new_filled(pos: Point3<i32>, block: u32) -> Chunk {
        Chunk {
            position: pos * SIZE as i32,
            blocks: [[[block; SIZE as usize]; SIZE as usize]; SIZE as usize],
        }
    }
    pub fn new_layered(pos: Point3<i32>, surface: u32, shallow: u32, deep: u32) -> Chunk {
        let mut blocks: [[[u32; SIZE as usize]; SIZE as usize]; SIZE as usize] =
            [[[0; SIZE as usize]; SIZE as usize]; SIZE as usize];
        for x in 0..blocks.len() {
            for y in 0..blocks[x].len() {
                for z in 0..blocks[x][y].len() {
                    if y as u8 >= SIZE - 1 {
                        blocks[x][y][z] = surface;
                    } else if (y as u8) < SIZE - 1 && (y as f32) >= SIZE as f32 / 1.5 {
                        blocks[x][y][z] = shallow;
                    } else {
                        blocks[x][y][z] = deep;
                    }
                }
            }
        }
        Chunk {
            position: pos * SIZE as i32,
            blocks,
        }
    }
    pub fn new_perlin(pos: Point3<i32>, block: u32) -> Chunk {
        let mut blocks: [[[u32; SIZE as usize]; SIZE as usize]; SIZE as usize] =
            [[[0; SIZE as usize]; SIZE as usize]; SIZE as usize];
        for x in 0..blocks.len() {
            for y in 0..blocks[x].len() {
                for z in 0..blocks[x][y].len() {
                    blocks[x][y][z] = if PERLIN.get().unwrap().get([
                        ((x as f64 + 0.4) + (pos.x * SIZE as i32) as f64) / 10.0,
                        ((y as f64 + 0.7) + (pos.y * SIZE as i32) as f64) / 10.0,
                        ((z as f64 + 0.8) + (pos.z * SIZE as i32) as f64) / 10.0]
                    ) > 0.15 {
                        block
                    } else {
                        0
                    };
                }
            }
        }
        Chunk {
            position: pos * SIZE as i32,
            blocks,
        }
    }

    pub fn get_block(&self, pos: Point3<u8>) -> u32 {
        if pos.x >= SIZE || pos.y >= SIZE || pos.z >= SIZE {
            return 0;
        }
        self.blocks[pos.x as usize][pos.y as usize][pos.z as usize]
    }
    pub fn get_block_s(&self, pos: Point3<i16>) -> u32 {
        if pos.x >= SIZE as i16 || pos.y >= SIZE as i16 || pos.z >= SIZE as i16 || pos.x < 0 || pos.y < 0 || pos.z < 0 {
            return 0;
        }
        self.blocks[pos.x as usize][pos.y as usize][pos.z as usize]
    }

    pub fn set_block(&mut self, pos: Point3<u8>, block: u32) {
        self.blocks[pos.x as usize][pos.y as usize][pos.z as usize] = block;
    }
    pub fn set_block_global(&mut self, pos: Point3<i32>, block: u32) {
        let x = (pos.x - self.position.x) + SIZE as i32 / 2;
        let y = (pos.y - self.position.y) + SIZE as i32 / 2;
        let z = (pos.z - self.position.z) + SIZE as i32 / 2;
        if x >= SIZE as i32 || y >= SIZE as i32 || z >= SIZE as i32 || x < 0 || y < 0 || z < 0 {
            return;
        };
        self.blocks[(x as i32) as usize][(y as i32) as usize][(z as i32) as usize] = block;
    }
    pub fn get_block_global(&self, pos: Point3<i32>) -> u32 {
        let x = (pos.x - self.position.x) + SIZE as i32 / 2;
        let y = (pos.y - self.position.y) + SIZE as i32 / 2;
        let z = (pos.z - self.position.z) + SIZE as i32 / 2;
        if x >= SIZE as i32 || y >= SIZE as i32 || z >= SIZE as i32 || x < 0 || y < 0 || z < 0 {
            return 0;
        };
        self.blocks[(x as i32) as usize][(y as i32) as usize][(z as i32) as usize]
    }
}
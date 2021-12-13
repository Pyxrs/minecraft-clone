
use cgmath::Vector3;
use rand::Rng;
use try_catch::catch;

use crate::vertex::Vertex;
use crate::block::Block;
use crate::{block_types, Direction};

const SIZE: usize = 8;

pub struct Chunk {
    pos: Vector3<i16>,
    blocks: [[[Block; SIZE]; SIZE]; SIZE],
}

impl Chunk {
    pub fn new(pos: Vector3<i16>) -> Self {
        Self {
            pos,
            blocks: [[[Block::new(block_types::get(0)); SIZE]; SIZE]; SIZE],
        }
    }

    pub fn render(&mut self) -> (Vec<Vertex>, Vec<u16>) {
        let mut rng = rand::thread_rng();

        let mut vertices: Vec<Vertex> = Vec::new();
        let mut indices: Vec<u16> = Vec::new();
        for locx in 0..(self.blocks.len()) {
            for locy in 0..(self.blocks[locx].len()) {
                for locz in 0..(self.blocks[locy].len()) {
                    self.blocks[locx][locy][locz] = Block::new(block_types::get(4));

                    let block = self.get_loc_block(Vector3::new(locx as u8, locy as u8, locz as u8));
                    let offset = index(Vector3::new(locx as u32, locy as u32, locz as u32));
                    if block.get_id() != 0 {
                        for quad in block.render(self.get_glo_pos(Vector3::new(locx as u8, locy as u8, locz as u8)), offset).iter() {
                            if quad.get_direction().get_id() == 1 && is_on_edge(Vector3::new(locx as u8, locy as u8, locz as u8), quad.get_direction().get_id()) {
                                vertices.extend(quad.get_vertices().iter());
                                indices.extend(quad.get_indices(offset).iter());
                            } else {
                                //let vec = quad.get_direction().get_vec();
                                //let faced_block = self.blocks[(locx as i8 + vec.x) as usize][(locy as i8 + vec.y) as usize][(locz as i8 + vec.z) as usize];
                                //if faced_block.get_id() == 0 {
                                    //vertices.extend(quad.get_vertices().iter());
                                    //indices.extend(quad.get_indices(offset).iter());
                                //}
                            }
                        }
                    }
                }
            }
        }
        return (vertices, indices);
    }

    pub fn get_loc_block(&self, pos: Vector3<u8>) -> Block {
        self.blocks[pos.x as usize][pos.y as usize][pos.z as usize]
    }

    pub fn get_glo_block(&self, pos: Vector3<i32>) -> Block {
        self.blocks[(pos.x + self.pos.x as i32) as usize][(pos.y + self.pos.y as i32) as usize][(pos.z + self.pos.z as i32) as usize]
    }

    fn get_glo_pos(&self, loc: Vector3<u8>) -> Vector3<i32> {
        Vector3::new(loc.x as i32 + self.pos.x as i32, loc.y as i32 + self.pos.y as i32, loc.z as i32 + self.pos.z as i32)
    }
}

fn is_on_edge(pos: Vector3<u8>, direction: u8) -> bool {
    //if pos.y <= 1 || pos.y >= 3 {// || pos.x >= (SIZE - 1) as u8 || pos.y >= (SIZE - 1) as u8 || pos.z >= (SIZE - 1) as u8 {
    ////if pos.x <= 0 {//&& direction == &Direction::EAST {
        return true;
    //}
    //false
}

fn index(pos: Vector3<u32>) -> u16 {
    let index = (pos.x * (SIZE * SIZE) as u32) + (pos.y * SIZE as u32) + pos.z;
    if index >= 65535 {
        panic!("Chunk size too big! Exceeded buffer limit");
    }
    index as u16
}
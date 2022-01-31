use cgmath::Vector3;
use wgpu::util::DeviceExt;
use wgpu::{Device, Buffer};

use crate::vertex::Vertex;
use crate::block::Block;
use crate::block_types;

const SIZE: usize = 16;

pub struct Chunk {
    pos: Vector3<i16>,
    blocks: [[[Block; SIZE]; SIZE]; SIZE],
}

impl Chunk {
    pub fn new(pos: Vector3<i16>) -> Self {
        Self {
            pos,
            blocks: [[[Block::new(block_types::get(4)); SIZE]; SIZE]; SIZE],
        }
    }

    pub fn render(&mut self, device: &Device) -> (Buffer, Buffer, u32) {
        println!("{}, {}, {}, {}", self.get_loc_block(Vector3::new(0, 1, 0)).get_id(), self.get_loc_block(Vector3::new(2, 0, 7)).get_id(), self.get_loc_block(Vector3::new(5, 10, 5)).get_id(), self.get_loc_block(Vector3::new(9, 5, 2)).get_id());

        let mut vertices: Vec<Vertex> = Vec::new();
        let mut indices: Vec<u32> = Vec::new();

        for x in 0..(self.blocks.len()) {
            for y in 0..(self.blocks[x].len()) {
                for z in 0..(self.blocks[y].len()) {
                    let x = x as u8;
                    let y = y as u8;
                    let z = z as u8;

                    let block = self.get_loc_block(Vector3::new(x, y, z));
                    let offset = index(Vector3::new(x as u32, y as u32, z as u32));

                    if block.get_id() != 0 {
                        let mut visible = [false, false, false, false, false, false];
                        let up_block = self.get_loc_block(Vector3::new(x, y + 1, z));

                        if up_block.get_id() == 0 {
                            visible = [true, true, true, true, true, true];
                        }

                        for quad in block.render(visible, self.get_glo_pos(Vector3::new(x, y, z)), offset).iter() {
                            vertices.extend(quad.get_vertices().iter());
                            indices.extend(quad.get_indices(offset).iter());
                        }
                    }
                }
            }
        }

        let vertex_buffer = device.create_buffer_init(&wgpu::util::BufferInitDescriptor {
            label: Some("Vertex Buffer"),
            contents: bytemuck::cast_slice(&vertices),
            usage: wgpu::BufferUsages::VERTEX,
        });
        let index_buffer = device.create_buffer_init(&wgpu::util::BufferInitDescriptor {
            label: Some("Index Buffer"),
            contents: bytemuck::cast_slice(&indices),
            usage: wgpu::BufferUsages::INDEX,
        });
        let num_indices = indices.len() as u32;
        return (vertex_buffer, index_buffer, num_indices);
    }

    pub fn get_loc_block(&self, pos: Vector3<u8>) -> Block {
        // Check if out of bounds
        if pos.x >= SIZE as u8 { return Block::new(block_types::get(0)) }
        if pos.y >= SIZE as u8 { return Block::new(block_types::get(0)) }
        if pos.z >= SIZE as u8 { return Block::new(block_types::get(0)) }
        
        self.blocks[pos.x as usize][pos.y as usize][pos.z as usize]
    }

    //pub fn get_glo_block(&self, pos: Vector3<i32>) -> Block {
    //    self.blocks[(pos.x + self.pos.x as i32) as usize][(pos.y + self.pos.y as i32) as usize][(pos.z + self.pos.z as i32) as usize]
    //}

    fn get_glo_pos(&self, loc: Vector3<u8>) -> Vector3<i32> {
        Vector3::new(loc.x as i32 + self.pos.x as i32, loc.y as i32 + self.pos.y as i32, loc.z as i32 + self.pos.z as i32)
    }
}

fn index(pos: Vector3<u32>) -> u32 {
    (pos.x * (SIZE * SIZE) as u32) + (pos.y * SIZE as u32) + pos.z
}
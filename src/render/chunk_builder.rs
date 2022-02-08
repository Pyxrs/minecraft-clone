use cgmath::Vector3;
use wgpu::{util::DeviceExt, Device};

use crate::chunk;
use crate::{chunk::Chunk, direction::Direction, Vertex};
use crate::render::quad::Quad;

use super::quad::block_quad;

pub fn build(device: &Device, pos: Vector3<f32>, chunk: &mut Chunk) -> (wgpu::Buffer, wgpu::Buffer, u32) {
    let chunk_size = chunk.blocks.len();

    let mut quads: Vec<Quad> = Vec::new();
    let mut index = 0;
    for x in 0..chunk_size {
        for y in 0..chunk.blocks[x].len() {
            for z in 0..chunk.blocks[x][y].len() {
                if chunk.get_block(x as u8, y as u8, z as u8) == 0 {
                    continue;
                }
                let mut count = 0;

                if chunk.get_block(x as u8, y as u8 + 1, z as u8) == 0 {
                    quads.push(block_quad(chunk.get_block(x as u8, y as u8, z as u8), count + index, Direction::UP,  Vector3::new(pos.x + x as f32 - (chunk_size as f32 / 2.0), pos.y + y as f32 + 0.5 - (chunk_size as f32 / 2.0), pos.z + z as f32 - (chunk_size as f32 / 2.0))));
                    count += 1;
                } if chunk.get_block_s(x as i16, y as i16 - 1, z as i16) == 0 {
                    quads.push(block_quad(chunk.get_block(x as u8, y as u8, z as u8), count + index, Direction::DOWN,  Vector3::new(pos.x + x as f32 - (chunk_size as f32 / 2.0), pos.y + y as f32 - 0.5 - (chunk_size as f32 / 2.0), pos.z + z as f32 - (chunk_size as f32 / 2.0))));
                    count += 1;
                } if chunk.get_block_s(x as i16, y as i16, z as i16 - 1) == 0 {
                    quads.push(block_quad(chunk.get_block(x as u8, y as u8, z as u8), count + index, Direction::NORTH, Vector3::new(pos.x + x as f32 - (chunk_size as f32 / 2.0), pos.y + y as f32 - (chunk_size as f32 / 2.0), pos.z + z as f32 - 0.5 - (chunk_size as f32 / 2.0))));
                    count += 1;
                } if chunk.get_block(x as u8, y as u8, z as u8 + 1) == 0 {
                    quads.push(block_quad(chunk.get_block(x as u8, y as u8, z as u8), count + index, Direction::SOUTH, Vector3::new(pos.x + x as f32 - (chunk_size as f32 / 2.0), pos.y + y as f32 - (chunk_size as f32 / 2.0), pos.z + z as f32 + 0.5 - (chunk_size as f32 / 2.0))));
                    count += 1;
                } if chunk.get_block(x as u8 + 1, y as u8, z as u8) == 0 {
                    quads.push(block_quad(chunk.get_block(x as u8, y as u8, z as u8), count + index, Direction::WEST, Vector3::new(pos.x + x as f32 + 0.5 - (chunk_size as f32 / 2.0), pos.y + y as f32 - (chunk_size as f32 / 2.0), pos.z + z as f32 - (chunk_size as f32 / 2.0))));
                    count += 1;
                } if chunk.get_block_s(x as i16 - 1, y as i16, z as i16) == 0 {
                    quads.push(block_quad(chunk.get_block(x as u8, y as u8, z as u8), count + index, Direction::EAST, Vector3::new(pos.x + x as f32 - 0.5 - (chunk_size as f32 / 2.0), pos.y + y as f32 - (chunk_size as f32 / 2.0), pos.z + z as f32 - (chunk_size as f32 / 2.0))));
                    count += 1;
                }
                index += count;
            }
        }
    }

    let mut vertices: Vec<Vertex> = vec![];
    let mut indices: Vec<u32> = vec![];
    for quad in quads.iter() {
        vertices.extend(quad.vertices);
        indices.extend(quad.indices);
    }
    
    (
        device.create_buffer_init(&wgpu::util::BufferInitDescriptor {
            label: Some("Vertex Buffer"),
            contents: bytemuck::cast_slice(&vertices),
            usage: wgpu::BufferUsages::VERTEX,
        }),
        device.create_buffer_init(&wgpu::util::BufferInitDescriptor {
            label: Some("Index Buffer"),
            contents: bytemuck::cast_slice(&indices),
            usage: wgpu::BufferUsages::INDEX,
        }),
        indices.len() as u32,
    )
}

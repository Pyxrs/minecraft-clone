use cgmath::Vector3;
use wgpu::{Device, util::DeviceExt};

use crate::{direction::Direction, Vertex};

use super::quad::sky_quad;

pub fn build(device: &Device, pos: Vector3<f32>, time: f32, weather: u8) -> (wgpu::Buffer, wgpu::Buffer, u32) {
    let mut vertices: Vec<Vertex> = vec![];
    let mut indices: Vec<u32> = vec![];
    for quad in [
        sky_quad(0, Direction::UP,  Vector3::new(pos.x, pos.y - 0.5, pos.z)),
        sky_quad(1, Direction::DOWN,  Vector3::new(pos.x, pos.y + 0.5, pos.z)),
        sky_quad(2, Direction::NORTH,  Vector3::new(pos.x, pos.y, pos.z + 0.5)),
        sky_quad(3, Direction::SOUTH,  Vector3::new(pos.x, pos.y, pos.z - 0.5)),
        sky_quad(4, Direction::WEST,  Vector3::new(pos.x - 0.5, pos.y, pos.z)),
        sky_quad(5, Direction::EAST,  Vector3::new(pos.x + 0.5, pos.y, pos.z)),
    ].iter() {
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
use cgmath::Vector3;

use crate::Direction;
use crate::vertex::Vertex;

const TEXTURE_INCREMENT: f32 = 1.0 / 256.0;

pub struct Quad {
    texture_id: u16,
    position: Vector3<f32>,
    direction: Direction,
    vertices: [Vertex; 4],
    indices: [u32; 6]
}

impl Quad {
    pub fn new(texture_id: u16, direction: Direction, position: Vector3<f32>, index_offset: u32) -> Quad {
        let calculations = calc_quad(texture_id, index_offset, &direction, position);
        Quad {
            texture_id,
            direction,
            position,
            vertices: calculations.0,
            indices: calculations.1,
        }
    }

    pub fn get_position(&self) -> Vector3<f32> {
        self.position
    }

    pub fn get_direction(&self) -> &Direction {
        &self.direction
    }

    pub fn get_vertices(&self) -> &[Vertex] {
        &self.vertices
    }

    pub fn get_indices(&self, offset: u32) -> [u32; 6] {
        let mut indices = self.indices.clone();
        for i in 0..indices.len() {
            indices[i] = indices[i] + offset * 4;
        };
        indices
    }
}

fn calc_quad(id: u16, index: u32, direction: &Direction, position: Vector3<f32>) -> ([Vertex; 4], [u32; 6]) {
    let block_type = crate::block_types::get(id);
    let text_id = block_type.get_texture(direction);
    let text_0: f32 = TEXTURE_INCREMENT * text_id as f32;
    let text_1: f32 = TEXTURE_INCREMENT * (text_id + 1) as f32;

    let indices_f = [0 + index, 1 + index, 2 + index, 2 + index, 1 + index, 3 + index];
    let indices_b = [2 + index, 1 + index, 0 + index, 3 + index, 1 + index, 2 + index];

    let light = match direction {
        Direction::UP => 1.0,
        Direction::DOWN => 0.25,
        Direction::NORTH => 0.5,
        Direction::SOUTH => 0.75,
        Direction::WEST => 0.75,
        Direction::EAST => 0.5,
    };

    let vertices_ud = [
        Vertex {
            position: [-0.5 + position.x, position.y, 0.5 + position.z],
            tex_coords: [text_0, 1.0],
            light
        }, // A
        Vertex {
            position: [0.5 + position.x, position.y, 0.5 + position.z],
            tex_coords: [text_1, 1.0],
            light
        }, // B
        Vertex {
            position: [-0.5 + position.x, position.y, -0.5 + position.z],
            tex_coords: [text_0, 0.0],
            light
        }, // C
        Vertex {
            position: [0.5 + position.x, position.y, -0.5 + position.z],
            tex_coords: [text_1, 0.0],
            light
        }, // D
    ];
    let vertices_ns = [
        Vertex {
            position: [-0.5 + position.x, 0.5 + position.y, position.z],
            tex_coords: [text_1, 0.0],
            light
        }, // A
        Vertex {
            position: [0.5 + position.x, 0.5 + position.y, position.z],
            tex_coords: [text_0, 0.0],
            light
        }, // B
        Vertex {
            position: [-0.5 + position.x, -0.5 + position.y, position.z],
            tex_coords: [text_1, 1.0],
            light
        }, // C
        Vertex {
            position: [0.5 + position.x, -0.5 + position.y, position.z],
            tex_coords: [text_0, 1.0],
            light
        }, // D
    ];
    let vertices_we = [
        Vertex {
            position: [position.x, 0.5 + position.y, -0.5 + position.z],
            tex_coords: [text_1, 0.0],
            light
        }, // A
        Vertex {
            position: [position.x, 0.5 + position.y, 0.5 + position.z],
            tex_coords: [text_0, 0.0],
            light
        }, // B
        Vertex {
            position: [position.x, -0.5 + position.y, -0.5 + position.z],
            tex_coords: [text_1, 1.0],
            light
        }, // C
        Vertex {
            position: [position.x, -0.5 + position.y, 0.5 + position.z],
            tex_coords: [text_0, 1.0],
            light
        }, // D
    ];

    match direction {
        Direction::UP => (vertices_ud, indices_f),
        Direction::DOWN => (vertices_ud, indices_b),
        Direction::NORTH => (vertices_ns, indices_f),
        Direction::SOUTH => (vertices_ns, indices_b),
        Direction::WEST => (vertices_we, indices_f),
        Direction::EAST => (vertices_we, indices_b),
    }
}
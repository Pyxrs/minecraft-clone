use cgmath::Vector3;

use crate::Direction;
use crate::vertex::Vertex;

const TEXTURE_INCREMENT: f32 = 1.0 / 256.0;

pub struct Quad {
    texture_id: u16,
    position: Vector3<f32>,
    direction: Direction,
    verticies: [Vertex; 4],
    indices: [u16; 6]
}

impl Quad {
    pub fn new(texture_id: u16, direction: Direction, position: Vector3<f32>, index_offset: u16) -> Quad {
        let calculations = calc_quad(texture_id, index_offset, &direction, position);
        Quad {
            texture_id,
            direction,
            position,
            verticies: calculations.0,
            indices: calculations.1,
        }
    }

    pub fn get_position(&self) -> Vector3<f32> {
        self.position
    }

    pub fn get_direction(&self) -> &Direction {
        &self.direction
    }

    pub fn get_verticies(&self) -> &[Vertex] {
        &self.verticies
    }

    pub fn get_indices(&self) -> &[u16] {
        &self.indices
    }
}

fn calc_quad(id: u16, index: u16, direction: &Direction, position: Vector3<f32>) -> ([Vertex; 4], [u16; 6]) {
    let text_id = crate::block_types::BLOCK_TYPES.get().unwrap()[id as usize].texture_ids.get(direction).unwrap();
    let text_0: f32 = TEXTURE_INCREMENT * *text_id as f32;
    let text_1: f32 = TEXTURE_INCREMENT * (text_id + 1) as f32;

    let indices_f = [0 + index, 1 + index, 2 + index, 2 + index, 1 + index, 3 + index];
    let indices_b = [2 + index, 1 + index, 0 + index, 3 + index, 1 + index, 2 + index];

    let verticies_ud = [
        Vertex {
            position: [-0.5 + position.x, position.y, 0.5 + position.z],
            tex_coords: [text_0, 1.0],
        }, // A
        Vertex {
            position: [0.5 + position.x, position.y, 0.5 + position.z],
            tex_coords: [text_1, 1.0],
        }, // B
        Vertex {
            position: [-0.5 + position.x, position.y, -0.5 + position.z],
            tex_coords: [text_0, 0.0],
        }, // C
        Vertex {
            position: [0.5 + position.x, position.y, -0.5 + position.z],
            tex_coords: [text_1, 0.0],
        }, // D
    ];
    let verticies_ns = [
        Vertex {
            position: [-0.5 + position.x, 0.5 + position.y, position.z],
            tex_coords: [text_1, 0.0],
        }, // A
        Vertex {
            position: [0.5 + position.x, 0.5 + position.y, position.z],
            tex_coords: [text_0, 0.0],
        }, // B
        Vertex {
            position: [-0.5 + position.x, -0.5 + position.y, position.z],
            tex_coords: [text_1, 1.0],
        }, // C
        Vertex {
            position: [0.5 + position.x, -0.5 + position.y, position.z],
            tex_coords: [text_0, 1.0],
        }, // D
    ];
    let verticies_we = [
        Vertex {
            position: [position.x, 0.5 + position.y, -0.5 + position.z],
            tex_coords: [text_1, 0.0],
        }, // A
        Vertex {
            position: [position.x, 0.5 + position.y, 0.5 + position.z],
            tex_coords: [text_0, 0.0],
        }, // B
        Vertex {
            position: [position.x, -0.5 + position.y, -0.5 + position.z],
            tex_coords: [text_1, 1.0],
        }, // C
        Vertex {
            position: [position.x, -0.5 + position.y, 0.5 + position.z],
            tex_coords: [text_0, 1.0],
        }, // D
    ];

    match direction {
        Direction::UP => (verticies_ud, indices_f),
        Direction::DOWN => (verticies_ud, indices_b),
        Direction::NORTH => (verticies_ns, indices_f),
        Direction::SOUTH => (verticies_ns, indices_b),
        Direction::WEST => (verticies_we, indices_f),
        Direction::EAST => (verticies_we, indices_b),
    }
}
use cgmath::Vector3;

use crate::{Vertex, direction::Direction};

const TEXTURE_INCREMENT: f32 = 1.0 / 256.0;
const SKY_INCREMENT: f32 = 1.0 / 6.0;

pub struct Quad {
    pub vertices: [Vertex; 4],
    pub indices: [u32; 6],
}

impl Quad {
    fn new(vertices: [Vertex; 4], indices: [u32; 6]) -> Self {
        Self { vertices, indices }
    }
}

pub fn block_quad(
    id: u16,
    index: u32,
    direction: Direction,
    position: Vector3<f32>,
) -> Quad {
    let block_type = crate::block_types::get(id);
    let text_id = block_type.get_texture(direction);
    let text_0: f32 = TEXTURE_INCREMENT * text_id as f32;
    let text_1: f32 = TEXTURE_INCREMENT * (text_id + 1) as f32;
    quad(text_0, text_1, index, direction, position, true)
}

pub fn sky_quad(
    index: u32,
    direction: Direction,
    position: Vector3<f32>,
) -> Quad {
    let text_0: f32 = SKY_INCREMENT * direction.get_id() as f32;
    let text_1: f32 = SKY_INCREMENT * (direction.get_id() + 1) as f32;
    quad(text_0, text_1, index, direction, position, false)
}

pub fn quad(
    text_0: f32,
    text_1: f32,
    index: u32,
    direction: Direction,
    position: Vector3<f32>,
    lighting: bool,
) -> Quad {
    let indices_f = [
        0 + (index * 4),
        1 + (index * 4),
        2 + (index * 4),
        2 + (index * 4),
        1 + (index * 4),
        3 + (index * 4),
    ];
    let indices_b = [
        2 + (index * 4),
        1 + (index * 4),
        0 + (index * 4),
        3 + (index * 4),
        1 + (index * 4),
        2 + (index * 4),
    ];

    let light = if lighting {
        match direction {
            Direction::UP => 1.0,
            Direction::DOWN => 0.25,
            Direction::NORTH => 0.5,
            Direction::SOUTH => 0.5,
            Direction::WEST => 0.75,
            Direction::EAST => 0.25,
        }
    } else {
        1.0
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
        Direction::UP => Quad::new(vertices_ud, indices_f),
        Direction::DOWN => Quad::new(vertices_ud, indices_b),
        Direction::NORTH => Quad::new(vertices_ns, indices_f),
        Direction::SOUTH => Quad::new(vertices_ns, indices_b),
        Direction::WEST => Quad::new(vertices_we, indices_f),
        Direction::EAST => Quad::new(vertices_we, indices_b),
    }
}
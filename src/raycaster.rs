use cgmath::{Point3, Vector2, Vector3};

use crate::chunk_manager::ChunkManager;

pub struct BlockRayHit {
    pub position: Point3<f32>,
    pub block: u16
}

pub fn block_ray(chunk_manager: &ChunkManager, start: Point3<f32>, end: Point3<f32>, step: f32, max_distance: f32) -> Option<BlockRayHit> {
    let distance = ((start.x - end.x).powi(2) + (start.z - end.z).powi(2)).sqrt();
    let slope = Vector3::new(start.y - end.y, start.x - end.x, start.z - end.z) / distance;
    let mut current_offset = 0_f32;
    while current_offset < max_distance {
        current_offset += step;
        let current_pos = Point3::new(current_offset * slope.x, current_offset * slope.y, current_offset * slope.z);
        let block = chunk_manager.get_block(Point3::new(current_pos.x as i32, current_pos.y as i32, current_pos.z as i32));
        if block != 0 {
            return Some(BlockRayHit {
                position: current_pos,
                block
            });
        }
    }
    None
}
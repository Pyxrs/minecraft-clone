use cgmath::Point3;

use crate::chunk;

pub fn get_chunk_position(pos: Point3<i32>) -> Point3<i32> {
    Point3::new(
        (pos.x as f32 / chunk::SIZE as f32).round() as i32,
        (pos.y as f32 / chunk::SIZE as f32).round() as i32,
        (pos.z as f32 / chunk::SIZE as f32).round() as i32)
}
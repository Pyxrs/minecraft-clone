use cgmath::{Point3, Vector3, num_traits::Float, InnerSpace};

use crate::chunk;

pub fn to_point(pos: Vector3<i32>) -> Point3<i32> {
    Point3::new(pos.x, pos.y, pos.z)
}
pub fn to_vector(pos: Point3<i32>) -> Vector3<i32> {
    Vector3::new(pos.x, pos.y, pos.z)
}
pub fn pointi32(pos: Point3<f32>) -> Point3<i32> {
    Point3::new(pos.x as i32, pos.y as i32, pos.z as i32)
}
pub fn pointi64(pos: Point3<i32>) -> Point3<i64> {
    Point3::new(pos.x as i64, pos.y as i64, pos.z as i64)
}
pub fn pointf32(pos: Point3<i32>) -> Point3<f32> {
    Point3::new(pos.x as f32, pos.y as f32, pos.z as f32)
}
pub fn pointf64(pos: Point3<i32>) -> Point3<f64> {
    Point3::new(pos.x as f64, pos.y as f64, pos.z as f64)
}
pub fn multiply(pos1: Point3<i32>, pos2: Point3<i32>) -> Point3<i32> {
    Point3::new(pos1.x * pos2.x, pos1.y * pos2.y, pos1.z * pos2.z)
}
pub fn add(pos1: Point3<i32>, pos2: Point3<i32>) -> Point3<i32> {
    Point3::new(pos1.x + pos2.x, pos1.y + pos2.y, pos1.z + pos2.z)
}
pub fn root(pos: Point3<i32>) -> Point3<i32> {
    Point3::new((pos.x as f32).sqrt() as i32, (pos.y as f32).sqrt() as i32, (pos.z as f32).sqrt() as i32)
}
pub fn distance2(pos1: Point3<i32>, pos2: Point3<i32>) -> i32 {
    (pos1 - pos2).magnitude2()
}
pub fn get_chunk_position(pos: Point3<i32>) -> Point3<i32> {
    Point3::new(
        (pos.x as f32 / chunk::SIZE as f32).round() as i32,
        (pos.y as f32 / chunk::SIZE as f32).round() as i32,
        (pos.z as f32 / chunk::SIZE as f32).round() as i32)
}
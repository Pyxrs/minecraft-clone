use wgpu::{Buffer, Device};

use crate::{chunk_manager::ChunkManager, chunk::Chunk};

use super::chunk_builder;

pub struct ChunkBuffers {
    buffers: Vec<(Buffer, Buffer, u32)>
}

impl ChunkBuffers {
    pub fn new(device: &Device, manager: &ChunkManager) -> ChunkBuffers {
        let mut buffers: Vec<(Buffer, Buffer, u32)> = Vec::new();
        for chunk in &manager.chunks {
            buffers.push(chunk_builder::build(device, chunk));
        }
        ChunkBuffers {
            buffers
        }
    }

    pub fn refresh(&mut self, device: &Device, manager: &ChunkManager) {
        self.buffers.clear();
        for chunk in &manager.chunks {
            self.buffers.push(chunk_builder::build(device, chunk));
        }
    }

    pub fn update_chunk(&mut self, device: &Device, index: usize, chunk: &Chunk) {
        self.buffers[index] = chunk_builder::build(device, chunk);
    }

    pub fn get_buffers(&self) -> &Vec<(wgpu::Buffer, wgpu::Buffer, u32)> {
        &self.buffers
    }

    pub fn get_buffer(&self, index: usize) -> &(Buffer, Buffer, u32) {
        &self.buffers[index]
    }

    pub fn get_buffer_count(&self) -> usize {
        self.buffers.len()
    }
}
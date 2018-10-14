
use std::sync::Arc;
use vulkano::buffer::{CpuAccessibleBuffer, BufferUsage};
use vulkano::device::Device;
use render::Vertex;

pub struct Mesh {
    pub vertex_buffer: Arc<CpuAccessibleBuffer<[Vertex]>>
}

impl Mesh {
    pub fn new(data: Vec<Vertex>, device: Arc<Device>, ) -> Self {

        let vertex_buffer = CpuAccessibleBuffer::from_iter(device.clone(), BufferUsage::all(),
            data
                .iter()
                .cloned())
            .expect("Failed to create buffer");

        Self {
            vertex_buffer,
        }
    }
}
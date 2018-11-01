
//use std;
use std::sync::Arc;
use std::path::Path;
//use vulkano::pipeline::{ GraphicsPipeline, vertex::SingleBufferDefinition};
use vulkano::buffer::{CpuAccessibleBuffer, BufferUsage};
use vulkano::device::{ Device, Queue };
use render::Vertex;
use image;
use vulkano;
use render;
use render::transform::Transform;
use render::vs;
use cgmath::{
    SquareMatrix,
    Matrix4,
    Vector3,

};
//type ConcreteGraphicsPipeline = GraphicsPipeline<SingleBufferDefinition<Vertex>, std::boxed::Box<vulkano::descriptor::PipelineLayoutAbstract + std::marker::Send + std::marker::Sync>, std::sync::Arc<vulkano::framebuffer::RenderPassAbstract + std::marker::Send + std::marker::Sync>>;
//type ConcreteDescriptor = vulkano::descriptor::descriptor_set::PersistentDescriptorSet<std::sync::Arc<vulkano::pipeline::GraphicsPipeline<vulkano::pipeline::vertex::SingleBufferDefinition<render::Vertex>, std::boxed::Box<vulkano::descriptor::PipelineLayoutAbstract + std::marker::Send + std::marker::Sync>, std::sync::Arc<vulkano::framebuffer::RenderPassAbstract + std::marker::Send + std::marker::Sync>>>, (((), vulkano::descriptor::descriptor_set::PersistentDescriptorSetImg<std::sync::Arc<vulkano::image::ImmutableImage<vulkano::format::R8G8B8A8Srgb>>>), vulkano::descriptor::descriptor_set::PersistentDescriptorSetSampler)>;

#[derive(Clone)]
pub struct Mesh {
    pub vertex_buffer: Arc<CpuAccessibleBuffer<[Vertex]>>,
    pub texture: Arc<vulkano::image::ImmutableImage<vulkano::format::R8G8B8A8Srgb>>,
    pub sampler: Arc<vulkano::sampler::Sampler>,
    pub transform: Transform,

}

impl Mesh {
    pub fn new(data: Vec<Vertex>, device: Arc<Device>, queue: Arc<Queue>, path: String) -> Self {

        let vertex_buffer = CpuAccessibleBuffer::from_iter(device.clone(), BufferUsage::all(),
            data
                .iter()
                .cloned())
            .expect("Failed to create buffer");

        let (texture, _tex_future) = {
            //let image = image::load_from_memory_with_format(include_bytes!("test"),image::ImageFormat::PNG).unwrap().to_rgba();

            let image = image::open(&Path::new(&path)).unwrap().to_rgba();
            let (width, height) = image.dimensions();

            let image_data = image.into_raw().clone();

            vulkano::image::immutable::ImmutableImage::from_iter(
                image_data.iter().cloned(),
                vulkano::image::Dimensions::Dim2d { width, height },
                vulkano::format::R8G8B8A8Srgb,
                queue.clone()).unwrap()
        };


        let sampler = vulkano::sampler::Sampler::new(device.clone(), vulkano::sampler::Filter::Linear,
                                                    vulkano::sampler::Filter::Linear, vulkano::sampler::MipmapMode::Nearest,
                                                    vulkano::sampler::SamplerAddressMode::Repeat,
                                                    vulkano::sampler::SamplerAddressMode::Repeat,
                                                    vulkano::sampler::SamplerAddressMode::Repeat,
                                                    0.0, 1.0, 0.0, 0.0).unwrap();

        let transform = Transform::new();
        Self {
            vertex_buffer,
            texture,
            sampler,
            transform,
        }
    }


    pub fn update(&self, view: [[f32; 4]; 4], projection: [[f32; 4]; 4],  world: [[f32; 4]; 4] ) -> render::vs::ty::Data {

            let rotation: [[f32; 4]; 4] = self.transform.rotation.into();
            let scale: [[f32; 4]; 4] = self.transform.scale.into();

            

            let world = self.transform.scale * self.transform.translation_matrix();
            let translation: [[f32; 4]; 4] = world.into();

            let vieww: Matrix4<f32> = view.into();
            let scale =  vieww * Matrix4::from_scale(0.1);

            let uniform_data = vs::ty::Data {
                model: world.into(),
                projection: projection,
                view: view,
            };

            uniform_data
    }

}

/*
impl Clone for Mesh {
    fn clone(&self) -> Mesh { *self }
}
*/
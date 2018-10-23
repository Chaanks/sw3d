
use std;
use std::sync::Arc;
use std::path::Path;
use vulkano::pipeline::{ GraphicsPipeline, vertex::SingleBufferDefinition};
use vulkano::buffer::{CpuAccessibleBuffer, BufferUsage};
use vulkano::device::{ Device, Queue };
use render::Vertex;
use image;
use vulkano;
use render;

type ConcreteGraphicsPipeline = GraphicsPipeline<SingleBufferDefinition<Vertex>, std::boxed::Box<vulkano::descriptor::PipelineLayoutAbstract + std::marker::Send + std::marker::Sync>, std::sync::Arc<vulkano::framebuffer::RenderPassAbstract + std::marker::Send + std::marker::Sync>>;
type ConcreteDescriptor = vulkano::descriptor::descriptor_set::PersistentDescriptorSet<std::sync::Arc<vulkano::pipeline::GraphicsPipeline<vulkano::pipeline::vertex::SingleBufferDefinition<render::Vertex>, std::boxed::Box<vulkano::descriptor::PipelineLayoutAbstract + std::marker::Send + std::marker::Sync>, std::sync::Arc<vulkano::framebuffer::RenderPassAbstract + std::marker::Send + std::marker::Sync>>>, (((), vulkano::descriptor::descriptor_set::PersistentDescriptorSetImg<std::sync::Arc<vulkano::image::ImmutableImage<vulkano::format::R8G8B8A8Srgb>>>), vulkano::descriptor::descriptor_set::PersistentDescriptorSetSampler)>;

pub struct Mesh {
    pub vertex_buffer: Arc<CpuAccessibleBuffer<[Vertex]>>,
    pub texture: Arc<vulkano::image::ImmutableImage<vulkano::format::R8G8B8A8Srgb>>,
    pub sampler: Arc<vulkano::sampler::Sampler>,
    pub set: Arc<ConcreteDescriptor>,
}

impl Mesh {
    pub fn new(data: Vec<Vertex>, device: Arc<Device>, queue: Arc<Queue>, pipeline: Arc<ConcreteGraphicsPipeline>, path: String) -> Self {

        let vertex_buffer = CpuAccessibleBuffer::from_iter(device.clone(), BufferUsage::all(),
            data
                .iter()
                .cloned())
            .expect("Failed to create buffer");

        let (texture, tex_future) = {
            //let image = image::load_from_memory_with_format(include_bytes!("test"),image::ImageFormat::PNG).unwrap().to_rgba();

            let image = image::open(&Path::new(&path)).unwrap().to_rgba();
            let image_data = image.into_raw().clone();

            vulkano::image::immutable::ImmutableImage::from_iter(
                image_data.iter().cloned(),
                vulkano::image::Dimensions::Dim2d { width: 960, height: 640 },
                vulkano::format::R8G8B8A8Srgb,
                queue.clone()).unwrap()
        };


        let sampler = vulkano::sampler::Sampler::new(device.clone(), vulkano::sampler::Filter::Linear,
                                                    vulkano::sampler::Filter::Linear, vulkano::sampler::MipmapMode::Nearest,
                                                    vulkano::sampler::SamplerAddressMode::Repeat,
                                                    vulkano::sampler::SamplerAddressMode::Repeat,
                                                    vulkano::sampler::SamplerAddressMode::Repeat,
                                                    0.0, 1.0, 0.0, 0.0).unwrap();

        let set = Arc::new(vulkano::descriptor::descriptor_set::PersistentDescriptorSet::start(pipeline.clone(), 0)
            .add_sampled_image(texture.clone(), sampler.clone()).unwrap()
            .build().unwrap()
        );

        Self {
            vertex_buffer,
            texture,
            sampler,
            set,
        }
    }
}
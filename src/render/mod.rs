mod mesh;


use std::sync::Arc;
use std;
use vulkano;
use winit;

use vulkano::pipeline::{ GraphicsPipeline, vertex::SingleBufferDefinition };
use vulkano::framebuffer::*;
use vulkano::command_buffer::{ AutoCommandBuffer, DynamicState };
use vulkano::buffer::{ BufferUsage, CpuAccessibleBuffer };
use vulkano::device::{ Device, Queue};
use vulkano::image::{ SwapchainImage};
use vulkano::swapchain::Swapchain;


#[derive(Debug, Clone)]
struct Vertex {
    position: [f32; 2],
    //color: [f32; 3],
}
impl_vertex!(Vertex, position);

#[allow(unused)]
mod vs {
    #[derive(VulkanoShader)]
    #[ty = "vertex"]
    #[path = "src/shaders/vertex_shader.glsl"]
    #[allow(dead_code)]

    struct Dummy;
}
#[allow(unused)]
mod fs {
    #[derive(VulkanoShader)]
    #[ty = "fragment"]
    #[path = "src/shaders/fragment_shader.glsl"]
    #[allow(dead_code)]

    struct Dummy;
}

type ConcreteGraphicsPipeline = GraphicsPipeline<SingleBufferDefinition<Vertex>, std::boxed::Box<vulkano::descriptor::PipelineLayoutAbstract + std::marker::Send + std::marker::Sync>, std::sync::Arc<vulkano::framebuffer::RenderPassAbstract + std::marker::Send + std::marker::Sync>>;


pub struct Renderer {
    //dimensions: Dimensions,
    pub dpi_factor: f64,
    graphics_pipeline: Arc<ConcreteGraphicsPipeline>,
    pub swapchain_framebuffers: Vec<Arc<FramebufferAbstract + Send + Sync>>,
    //textures: Vec<Image>,
}


impl Renderer {
    pub fn new(device: Arc<Device>, render_pass: Arc<RenderPassAbstract + Send + Sync>,
        images: Vec<Arc<SwapchainImage<winit::Window>>>, dpi_factor: f64) -> Self {

        let vs = vs::Shader::load(device.clone())
            .expect("Failed to create shader module");
        
        let fs = fs::Shader::load(device.clone())
            .expect("Failed to create fragment module");
        
        let graphics_pipeline = Arc::new(GraphicsPipeline::start()
            //.vertex_input(vulkano::pipeline::vertex::TwoBuffersDefinition::new())
            .vertex_input_single_buffer::<Vertex>()
            .vertex_shader(vs.main_entry_point(), ())
            .triangle_list()
            .viewports_dynamic_scissors_irrelevant(1)
            .fragment_shader(fs.main_entry_point(), ())
            .render_pass(Subpass::from(render_pass.clone(), 0).unwrap())
            .build(device.clone())
            .expect("Failed to create graphics pipeline")
            
        );

        let swapchain_framebuffers = images.iter()
            .map(|image| {
                let fba: Arc<FramebufferAbstract + Send + Sync> = Arc::new(Framebuffer::start(render_pass.clone())
                    .add(image.clone()).unwrap()
                    .build().expect("Failed to create framebuffers"));
                    
                    fba
            }
        ).collect::<Vec<_>>();

        Self {
            dpi_factor,
            graphics_pipeline,
            swapchain_framebuffers,
        }
    }

    pub fn draw(&mut self, device: Arc<Device>, queue: Arc<Queue>, dynamic_state: & DynamicState, id: usize) -> AutoCommandBuffer {
        let vertex_positions = [ 
            Vertex { position: [0.0, -0.5] },
            Vertex { position: [0.5, 0.5] },
            Vertex { position: [-0.5, 0.5] }

        ];

        let vertex_buffer: Arc<CpuAccessibleBuffer<[Vertex]>> = CpuAccessibleBuffer::from_iter(device.clone(), BufferUsage::all(),
            vertex_positions
                .into_iter()
                .cloned())
            .expect("Failed to create buffer");
        
        let mut _command_buffer = vulkano::command_buffer::AutoCommandBufferBuilder::primary_one_time_submit(device.clone(), queue.family()).unwrap()
            .begin_render_pass(
                self.swapchain_framebuffers[id].clone(), false,vec![[0.0, 0.0, 0.0, 1.0].into()])
                    .unwrap()
                    .draw(self.graphics_pipeline.clone(), &dynamic_state, vertex_buffer.clone(), (), ()).unwrap();
/*
        for mesh in meshes.iter() {
            command_buffer = command_buffer
                .draw_indexed(
                pipeline.clone(),
                &dynamic_state,
                (mesh.vertex_buffer.clone(), mesh.normals_buffer.clone()), 
                mesh.index_buffer.clone(), set.clone(), ()).unwrap()   
        }
*/      
        let command_buffer = _command_buffer.end_render_pass().unwrap()
            .build().unwrap();
                
        command_buffer
    }




}

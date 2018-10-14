mod mesh;


use std::sync::Arc;

use winit;


use vulkano::pipeline::*;
use vulkano::framebuffer::*;
use vulkano::device::Device;

use vulkano::image::*;


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


pub struct Renderer {
    //dimensions: Dimensions,
    pub dpi_factor: f64,
    pub graphics_pipeline: Box<Arc<GraphicsPipelineAbstract+Send+Sync>>,
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
            graphics_pipeline: Box::new(graphics_pipeline),
            swapchain_framebuffers,
        }
    }
}
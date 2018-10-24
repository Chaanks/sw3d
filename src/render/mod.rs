pub mod mesh;

use std::sync::Arc;
use std;
use vulkano;
use vulkano::{
    device::{
        Device,
        Queue,
    },
    swapchain,
    framebuffer::RenderPassAbstract,
    image::SwapchainImage,
    instance::{
        Instance,
        PhysicalDevice,
        ApplicationInfo,
        Version,
        layers_list,
        InstanceExtensions,
        debug::DebugCallback,
    },
    swapchain::{
        PresentMode,
        Surface,
        SurfaceTransform,
        Swapchain,
    },
    pipeline::{
        GraphicsPipeline,
        vertex::SingleBufferDefinition,
        viewport::Viewport,
    },
};

use vulkano::framebuffer::*;
use vulkano::command_buffer::{ AutoCommandBuffer, DynamicState, AutoCommandBufferBuilder};
use vulkano::sync::now;
use vulkano::sync::GpuFuture;

use vulkano_win::{ VkSurfaceBuild, required_extensions };

use winit;
use winit::EventsLoop;


const VALIDATION_LAYERS: &[&str] =  &[
    "VK_LAYER_LUNARG_standard_validation"
];


#[cfg(all(debug_assertions))]
const ENABLE_VALIDATION_LAYERS: bool = true;
#[cfg(not(debug_assertions))]
const ENABLE_VALIDATION_LAYERS: bool = false;


#[derive(Debug, Clone)]
pub struct Vertex {
    pub pos: [f32; 3],
    pub uv: [f32; 2],
    // color: [f32; 3],
}
impl_vertex!(Vertex, pos, uv);

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


pub struct Render {
    pub surface: Arc<Surface<winit::Window>>,
    pub swapchain: Arc<Swapchain<winit::Window>>,
    pub queue: Arc<Queue>,
    pub events_loop: EventsLoop,
    pub device: Arc<Device>,
    pub images: Vec<Arc<SwapchainImage<winit::Window>>>,
    pub render_pass: Arc<RenderPassAbstract + Send + Sync>,
    pub debug_callback: Arc<DebugCallback>,
    pub dynamic_state: DynamicState,
    pub dpi_factor: f64,
    pub graphics_pipeline: Arc<ConcreteGraphicsPipeline>,
    pub swapchain_framebuffers: Vec<Arc<FramebufferAbstract + Send + Sync>>,
    pub meshs: Vec<mesh::Mesh>,
}

impl Render{
    
    pub fn new(width: u32, height: u32, title: &str) -> Self {

        let instance = {

            match InstanceExtensions::supported_by_core() {
                Ok(i) => println!("Supportted extensions: {:?}", i),
                Err(err) => panic!("Failed to retreive supported extensions: {:?}", err),
            };
            let mut extensions = required_extensions();
            extensions.ext_debug_report = true;

            if ENABLE_VALIDATION_LAYERS && !Self::check_validation_layer_support() {
                println!("Validation layers requested, but not available!")
            } else {
                for layer in vulkano::instance::layers_list().unwrap() {
                    println!("Available layer: {}", layer.name());
                }
            }

            let app_info = ApplicationInfo {
                application_name: Some(title.into()),
                application_version: Some(Version { major: 1, minor: 0, patch: 0 }),
                engine_name: Some("sw3d".into()),
                engine_version: Some(Version {major: 1, minor: 0, patch: 0}),
            };

            if ENABLE_VALIDATION_LAYERS && Self::check_validation_layer_support() {
                Instance::new(Some(&app_info), &extensions, VALIDATION_LAYERS.iter().map(|s| *s))
                    .expect("failed to create Vulkan instance")
            } else {
                Instance::new(Some(&app_info), &extensions, None)
                    .expect("failed to create Vulkan instance")
            }
        };

        let debug_types = vulkano::instance::debug::MessageTypes {
            error: true,
            warning: true,
            performance_warning: true,
            information: true,
            debug: true,
        };
        
        let debug_callback = Arc::new(DebugCallback::new(&instance.clone(), debug_types, |msg| {
            println!("Debug callback: {:?}", msg.description);
        }).expect("Failed to creaye debug callback"));


        for physical_device in PhysicalDevice::enumerate(&instance.clone()) {
            println!("Available device: {}", physical_device.name());
        }

        let cloned_instance = instance.clone();
        let physical = match PhysicalDevice::enumerate(&cloned_instance).next() {
            Some(i) => i,
            None => panic!("No device available")
        } ;

        let events_loop = winit::EventsLoop::new();
        let surface = winit::WindowBuilder::new()
            .with_dimensions((width, height).into())
            .with_title(title)
            .build_vk_surface(&events_loop, instance.clone()).unwrap();

        let queue_familie = physical.queue_families().find(|&q| {
            q.supports_graphics() && surface.is_supported(q).unwrap_or(false)
        }).expect("Failed to find a graphical queue family");

        let device_ext = vulkano::device::DeviceExtensions {
            khr_swapchain: true,
            .. vulkano::device::DeviceExtensions::none()
        };

        let (device, mut queues) = match Device::new(physical, physical.supported_features(), &device_ext,
            [(queue_familie, 0.5)].iter().cloned()) {
                Ok(i) => i,
                Err(err) => panic!("Failed to create device: {:?}", err),
        };

        let queue = queues.next().expect("Failed to get our queue");

        let ((swapchain, images), surface_dimensions) = {

            let caps = surface.capabilities(physical)
                .expect("Failed to get surface capabilities");
            
            let surface_dimensions = caps.current_extent.unwrap_or([width, height]);
            let alpha = caps.supported_composite_alpha.iter().next().unwrap();
            let format = caps.supported_formats[0].0;

            (Swapchain::new(device.clone(), surface.clone(), caps.min_image_count, format,
                             surface_dimensions, 1, caps.supported_usage_flags, &queue,
                             SurfaceTransform::Identity, alpha, PresentMode::Fifo, true,
                             None).expect("failed to create swapchain"), surface_dimensions)
        };

        let dynamic_state = DynamicState {
            line_width: None,
            viewports: Some(vec![Viewport {
                origin: [0.0, 0.0],
                dimensions: [surface_dimensions[0] as f32, surface_dimensions[1] as f32],
                depth_range: 0.0 .. 1.0,
            }]),
            scissors: None,
        };

        let render_pass: Arc<RenderPassAbstract + Send + Sync> = Arc::new(single_pass_renderpass!(device.clone(),
            attachments: {
                color: {
                    load: Clear,
                    store: Store,
                    format: swapchain.format(),
                    samples: 1,
                }
            },
            pass: {
                color: [color],
                depth_stencil: {}
            }
        ).unwrap());

        let dpi_factor = 1.0;

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

        let meshs = Vec::new();

        Self {
            surface,
            swapchain,
            queue,
            events_loop,
            device,
            images,
            render_pass,
            debug_callback: debug_callback.clone(),
            dynamic_state,
            dpi_factor,
            swapchain_framebuffers,
            graphics_pipeline,
            meshs,
        }

    }

    pub fn run(&mut self) {
    let mut previous_frame_end = Box::new(now(self.device.clone())) as Box<GpuFuture>;
        loop {
            previous_frame_end.cleanup_finished();
            
            
            let (image_num, acquire_future) = swapchain::acquire_next_image(self.swapchain.clone(), None).unwrap();
            let command_buffer: AutoCommandBuffer = self.draw(image_num);

            let future = previous_frame_end.join(acquire_future)
                .then_execute(self.queue.clone(), command_buffer).unwrap()
                .then_swapchain_present(self.queue.clone(), self.swapchain.clone(), image_num)
                .then_signal_fence_and_flush().unwrap();
            previous_frame_end = Box::new(future) as Box<_>;

            let mut done = false;
            self.events_loop.poll_events(|event| {
                match event {
                    winit::Event::WindowEvent { event: winit::WindowEvent::CloseRequested, .. } => done = true,
                    _ => (),
                }

            });
            
            if done {return;}
        }               

    }
    fn check_validation_layer_support() -> bool {
        let layers: Vec<_> = layers_list().unwrap().map(|l| l.name().to_owned()).collect();
        VALIDATION_LAYERS.iter()
            .all(|layer_name| layers.contains(&layer_name.to_string()))
    }

    pub fn draw(&mut self, image_num: usize ) -> AutoCommandBuffer {         
        let mut _command_buffer = AutoCommandBufferBuilder::primary_one_time_submit(self.device.clone(), self.queue.family()).unwrap()
            .begin_render_pass(
                self.swapchain_framebuffers[image_num].clone(), false,vec![[0.0, 0.0, 0.0, 1.0].into()])
                    .unwrap()
                    //.draw(self.graphics_pipeline.clone(), &dynamic_state, vertex_buffer.clone(), (), ()).unwrap()
                    .draw_mesh(self);

        let command_buffer = _command_buffer.end_render_pass().unwrap()
            .build().unwrap();
                
        command_buffer
    }

    fn draw_mesh(&mut self, mut command_buffer: AutoCommandBufferBuilder) -> AutoCommandBufferBuilder {
        for mesh in self.meshs.iter() {
            command_buffer = command_buffer
                .draw(
                self.graphics_pipeline.clone(),
                &self.dynamic_state,
                mesh.vertex_buffer.clone(), 
                mesh.set.clone(), ()).unwrap()
        }

        command_buffer
    }


}


pub trait DrawMeshTrait {
    fn draw_mesh(self, data: &mut Render) -> AutoCommandBufferBuilder;
}

impl DrawMeshTrait for AutoCommandBufferBuilder {
    fn draw_mesh(self, data: &mut Render) -> AutoCommandBufferBuilder {
        data.draw_mesh(self)
    }
}

pub const CUBE : [Vertex;36] = [
        Vertex { pos: [-0.5, -0.5, -0.5], uv: [ 0.0, 0.0]},
        Vertex { pos: [0.5, -0.5, -0.5], uv: [1.0, 0.0]},
        Vertex { pos: [0.5,  0.5, -0.5], uv: [1.0, 1.0]},
        Vertex { pos: [0.5,  0.5, -0.5], uv: [1.0, 1.0]},
        Vertex { pos: [-0.5,  0.5, -0.5], uv: [ 0.0, 1.0]},
        Vertex { pos: [-0.5, -0.5, -0.5], uv: [ 0.0, 0.0]},

        Vertex { pos: [-0.5, -0.5,  0.5], uv: [ 0.0, 0.0]},
        Vertex { pos: [0.5, -0.5,  0.5], uv: [1.0, 0.0]},
        Vertex { pos: [0.5,  0.5,  0.5], uv: [1.0, 1.0]},
        Vertex { pos: [0.5,  0.5,  0.5], uv: [1.0, 1.0]},
        Vertex { pos: [-0.5,  0.5,  0.5], uv: [ 0.0, 1.0]},
        Vertex { pos: [-0.5, -0.5,  0.5], uv: [ 0.0, 0.0]},

        Vertex { pos: [-0.5,  0.5,  0.5], uv: [ 1.0, 0.0]},
        Vertex { pos: [-0.5,  0.5, -0.5], uv: [ 1.0, 1.0]},
        Vertex { pos: [-0.5, -0.5, -0.5], uv: [ 0.0, 1.0]},
        Vertex { pos: [-0.5, -0.5, -0.5], uv: [ 0.0, 1.0]},
        Vertex { pos: [-0.5, -0.5,  0.5], uv: [ 0.0, 0.0]},
        Vertex { pos: [-0.5,  0.5,  0.5], uv: [ 1.0, 0.0]},

        Vertex { pos: [0.5,  0.5,  0.5], uv: [1.0, 0.0]},
        Vertex { pos: [0.5,  0.5, -0.5], uv: [1.0, 1.0]},
        Vertex { pos: [0.5, -0.5, -0.5], uv: [0.0, 1.0]},
        Vertex { pos: [0.5, -0.5, -0.5], uv: [0.0, 1.0]},
        Vertex { pos: [0.5, -0.5,  0.5], uv: [0.0, 0.0]},
        Vertex { pos: [0.5,  0.5,  0.5], uv: [1.0, 0.0]},

        Vertex { pos: [-0.5, -0.5, -0.5], uv: [ 0.0, 1.0]},
        Vertex { pos: [0.5, -0.5, -0.5], uv: [1.0, 1.0]},
        Vertex { pos: [0.5, -0.5,  0.5], uv: [1.0, 0.0]},
        Vertex { pos: [0.5, -0.5,  0.5], uv: [1.0, 0.0]},
        Vertex { pos: [-0.5, -0.5,  0.5], uv: [ 0.0, 0.0]},
        Vertex { pos: [-0.5, -0.5, -0.5], uv: [ 0.0, 1.0]},

        Vertex { pos: [-0.5,  0.5, -0.5], uv: [ 0.0, 1.0]},
        Vertex { pos: [0.5,  0.5, -0.5], uv: [1.0, 1.0]},
        Vertex { pos: [0.5,  0.5,  0.5], uv: [1.0, 0.0]},
        Vertex { pos: [0.5,  0.5,  0.5], uv: [1.0, 0.0]},
        Vertex { pos: [-0.5,  0.5,  0.5], uv: [ 0.0, 0.0]},
        Vertex { pos: [-0.5,  0.5, -0.5], uv: [ 0.0, 1.0]},
        ];
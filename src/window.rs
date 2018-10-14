use std::sync::Arc;
use vulkano;
use vulkano::{
    device::{
        Device,
        Queue,
    },
    framebuffer::RenderPassAbstract,
    image::SwapchainImage,
    instance::{
        Instance,
        PhysicalDevice,
        ApplicationInfo,
        Version,
        layers_list,
        InstanceExtensions,
    },
    swapchain::{
        PresentMode,
        Surface,
        SurfaceTransform,
        Swapchain,
    },
};
use vulkano_win::VkSurfaceBuild;

use winit;
use winit::EventsLoop;
use vulkano_win::required_extensions;
use vulkano::instance::debug::DebugCallback;


const VALIDATION_LAYERS: &[&str] =  &[
    "VK_LAYER_LUNARG_standard_validation"
];


#[cfg(all(debug_assertions))]
const ENABLE_VALIDATION_LAYERS: bool = true;
#[cfg(not(debug_assertions))]
const ENABLE_VALIDATION_LAYERS: bool = false;


pub struct Window {
    pub surface: Arc<Surface<winit::Window>>,
    pub swapchain: Arc<Swapchain<winit::Window>>,
    pub queue: Arc<Queue>,
    pub events_loop: EventsLoop,
    pub device: Arc<Device>,
    pub images: Vec<Arc<SwapchainImage<winit::Window>>>,
    pub render_pass: Arc<RenderPassAbstract + Send + Sync>,
    pub debug_callback: Arc<DebugCallback>,
}

impl Window {
    
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
        let surface = winit::WindowBuilder::new().build_vk_surface(&events_loop, instance.clone()).unwrap();

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

        let ((swapchain, images), _surface_dimensions) = {

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

        let render_pass = Arc::new(single_pass_renderpass!(device.clone(),
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

        Self {
            surface,
            swapchain,
            queue,
            events_loop,
            device,
            images,
            render_pass,
            debug_callback: debug_callback.clone(), 
        }

    }

    pub fn run(&mut self) {
        self.events_loop.run_forever(|event| {
        match event {
            winit::Event::WindowEvent { event: winit::WindowEvent::CloseRequested, .. } => {
                winit::ControlFlow::Break
            },

            _ => winit::ControlFlow::Continue,
        }
    });

    }
    fn check_validation_layer_support() -> bool {
        let layers: Vec<_> = layers_list().unwrap().map(|l| l.name().to_owned()).collect();
        VALIDATION_LAYERS.iter()
            .all(|layer_name| layers.contains(&layer_name.to_string()))
    }
}
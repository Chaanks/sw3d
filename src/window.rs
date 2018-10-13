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
        SwapchainCreationError,
    },
};
use vulkano_win::VkSurfaceBuild;

use winit;
use winit::EventsLoop;
use vulkano_win::required_extensions;
use vulkano::instance::debug::DebugCallback;
use std::mem;


const VALIDATION_LAYERS: &[&str] =  &[
    "VK_LAYER_LUNARG_standard_validation"
];


#[cfg(all(debug_assertions))]
const ENABLE_VALIDATION_LAYERS: bool = true;
#[cfg(not(debug_assertions))]
const ENABLE_VALIDATION_LAYERS: bool = false;


pub struct Window {
    surface: Arc<Surface<winit::Window>>,
}

impl Window {
    pub fn new(with: u32, height: u32, title: &str) {
        let instance = {

            match InstanceExtensions::supported_by_core() {
                Ok(i) => println!("Supportted extensions: {:?}", i),
                Err(err) => panic!("Failed to retreive supported extensions: {:?}", err),
            };
            let extensions = required_extensions();

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


    }

    fn check_validation_layer_support() -> bool {
        let layers: Vec<_> = layers_list().unwrap().map(|l| l.name().to_owned()).collect();
        VALIDATION_LAYERS.iter()
            .all(|layer_name| layers.contains(&layer_name.to_string()))
    }
}
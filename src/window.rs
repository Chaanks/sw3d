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


pub struct Window {
    surface: Arc<Surface<winit::Window>>,
}

impl Window {
    fn new(with: u32, height: u32, title: &str) -> Self {
        let instance = {
            let extensions = required_extensions();

        let app_info = ApplicationInfo {
            application_name: Some("Triangle".into()),
            application_version: Some(Version { major: 1, minor: 0, patch: 0 }),
            engine_name: Some("No engine".into()),
            engine_version: Some(Version {major: 1, minor: 0, patch: 0}),
        };
            Instance::new();
        };



        Self {
            
        }
    }
}
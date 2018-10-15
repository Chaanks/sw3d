use winit;
use vulkano::sync::now;
use vulkano::sync::GpuFuture;
use vulkano::swapchain;
use vulkano::command_buffer::{AutoCommandBuffer, AutoCommandBufferBuilder};
use vulkano::device::Device;
use state::{State, StateData};
use game_data::{GameData, GameDataBuilder, DataInit};
use specs::{System, VecStorage, Component, Builder, ReadStorage, DispatcherBuilder, World};
use bundle::SystemBundle;
use std::marker::PhantomData;
use std::sync::Arc;
use window::Window;
use render::Renderer;

pub struct Application<T> {
    world: World,
    data: T,
    window: Window,
}

impl <T>Application<T> {
    pub fn new<I>(init: I, width: u32, height: u32, title: &str) -> Self
    where
        I: DataInit<T>,  {

        let mut world = World::new();
        let data = init.build(&mut world);
        let window = Window::new(width, height, title);
        let device = world.add_resource:: <Arc<Device>> (window.device.clone());

        //let mut renderer = Renderer::new(window.device.clone(), window.render_pass.clone(), 
        //                                             window.images.clone(), 1.0);
        
        //world.add_resource:: <Renderer> (renderer);
        
        Self {
            world,
            data,
            window,
        }

    }


pub fn run<S>(&mut self, mut state: S) 
    where
        S: State<T>, {

    let mut state_data = StateData::new(&mut self.world,&mut self.data);
          
    state.on_start(&mut state_data);
    state.update(&mut state_data);
    state.handle_event(&mut state_data);
    state.update(&mut state_data);


    let mut previous_frame_end = Box::new(now(self.window.device.clone())) as Box<GpuFuture>;
    loop {
        previous_frame_end.cleanup_finished();
        
        
        let (image_num, acquire_future) = swapchain::acquire_next_image(self.window.swapchain.clone(), None).unwrap();
        let command_buffer: AutoCommandBuffer = render.update(self.window.device.clone(), self.window.queue.clone(), &self.window.dynamic_state, image_num);

        let future = previous_frame_end.join(acquire_future)
            .then_execute(self.window.queue.clone(), command_buffer).unwrap()
            .then_swapchain_present(self.window.queue.clone(), self.window.swapchain.clone(), image_num)
            .then_signal_fence_and_flush().unwrap();
        previous_frame_end = Box::new(future) as Box<_>;

        let mut done = false;
        self.window.events_loop.poll_events(|event| {
            match event {
                winit::Event::WindowEvent { event: winit::WindowEvent::CloseRequested, .. } => done = true,
                _ => (),
            }

        });
        
        if done {return;}
    }
}

}
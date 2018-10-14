extern crate sw3d;


use sw3d::winit;
use sw3d::vulkano::sync::now;
use sw3d::vulkano::sync::GpuFuture;
use sw3d::vulkano::swapchain;
use sw3d::vulkano::command_buffer::AutoCommandBuffer;


fn main () {
    println!("ez");
    let mut window = sw3d::window::Window::new(800, 600, "Test");
    let mut render = sw3d::render::Renderer::new(window.device.clone(), window.render_pass.clone(), 
                                                 window.images.clone(), 1.0);

    let mut previous_frame_end = Box::new(now(window.device.clone())) as Box<GpuFuture>;
    loop {
        previous_frame_end.cleanup_finished();
        
        

/*
        let command_buffers: Vec<Arc<AutoCommandBuffer>> = render.draw(window.device.clone(), window.queue.clone(), &window.dynamic_state);       
        let command_buffer = command_buffers[image_num].clone();
*/

        let (image_num, acquire_future) = swapchain::acquire_next_image(window.swapchain.clone(), None).unwrap();
        let command_buffer: AutoCommandBuffer = render.draw(window.device.clone(), window.queue.clone(), &window.dynamic_state, image_num);

        let future = previous_frame_end.join(acquire_future)
            .then_execute(window.queue.clone(), command_buffer).unwrap()
            .then_swapchain_present(window.queue.clone(), window.swapchain.clone(), image_num)
            .then_signal_fence_and_flush().unwrap();
        previous_frame_end = Box::new(future) as Box<_>;

        let mut done = false;
        window.events_loop.poll_events(|event| {
            match event {
                winit::Event::WindowEvent { event: winit::WindowEvent::CloseRequested, .. } => done = true,
                _ => (),
            }

        });
        
        if done {return;}
    }                                
}
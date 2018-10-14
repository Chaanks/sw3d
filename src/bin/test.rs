extern crate sw3d;

extern crate winit;



fn main () {
    println!("ez");
    let mut window = sw3d::window::Window::new(800, 600, "Test");
    window.events_loop.run_forever(|event| {
        match event {
            winit::Event::WindowEvent { event: winit::WindowEvent::CloseRequested, .. } => {
                winit::ControlFlow::Break
            },

            _ => winit::ControlFlow::Continue,
        }
    });
}
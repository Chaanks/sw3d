extern crate sw3d;

extern crate winit;



fn main () {
    println!("ez");
    let mut window = sw3d::window::Window::new(800, 600, "Test");
    let render = sw3d::render::Renderer::new(window.device.clone(), window.render_pass.clone(), 
                                                 window.images.clone(), 1.0);
    window.run();
}
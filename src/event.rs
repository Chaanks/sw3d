use render::Context;
use winit::VirtualKeyCode;


pub trait EventHandler {
    fn update(&mut self, _ctx: &mut Context);
    fn draw(&mut self, _ctx: &mut Context);
    fn key_down_event(&mut self, _ctx: &mut Context, key: VirtualKeyCode) {
        if key == VirtualKeyCode::W {
            println!("fdp2");
        }
    }
}
extern crate sw3d;

use sw3d::application::Application;
use sw3d::event::EventHandler;
use sw3d::render::Context;
use sw3d::render::mesh::Mesh;
use sw3d::render::CUBE;
use sw3d::time::Clock;

pub struct State {
    pub cube: Mesh,
    pub clock: Clock,
}

impl State {
    fn new(app: &Application) -> Self {
        let mut cube = Mesh::new(CUBE.to_vec(), app.ctx.device.clone(), app.ctx.queue.clone(), "tex.png".into());
        let clock = Clock::default();
        cube.transform.translate(0.5, -0.5 , 0.0);
        Self {
            cube,
            clock,
        }
    }
}

impl EventHandler for State {
    fn update(&mut self, _ctx: &mut Context) {
        self.clock.tick();
        let duration = self.clock.get_time_since_start();
        //self.cube.transform.translate(0.5, -0.5 , 0.0);
        //self.cube.transform.rotate_z(duration.into());
        self.cube.transform.rotate(duration.into(), [0.0, 0.0, 1.0]);
    }

    fn draw(&mut self, ctx: &mut Context) {
        ctx.draw(self.cube.clone());
    }
}

fn main () {

    let mut app = Application::new(800, 800, "Voxel");

    let mut state = State::new(&app);
    app.run(&mut state);


}
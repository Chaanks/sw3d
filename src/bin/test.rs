extern crate sw3d;

use sw3d::application::Application;
use sw3d::event::EventHandler;
use sw3d::render::Context;
use sw3d::render::mesh::Mesh;
use sw3d::render::CUBE;
use sw3d::time::Clock;
use cgmath::{
    SquareMatrix,
    Matrix4,
    Vector3,

};
use sw3d::cgmath;


pub struct State {
    pub cube1: Mesh,
    pub cube2: Mesh,
    pub cube3: Mesh,
    pub clock: Clock,

    pub world: Matrix4<f32>,
    pub view:  Matrix4<f32>,
    pub projection: Matrix4<f32>,


}

impl State {
    fn new(app: &Application) -> Self {
        let mut cube1 = Mesh::new(CUBE.to_vec(), app.ctx.device.clone(), app.ctx.queue.clone(), "tex.png".into());
        let mut cube2 = Mesh::new(CUBE.to_vec(), app.ctx.device.clone(), app.ctx.queue.clone(), "tex.png".into());
        let mut cube3 = Mesh::new(CUBE.to_vec(), app.ctx.device.clone(), app.ctx.queue.clone(), "tex.png".into());
        let clock = Clock::default();
        //cube.transform.translate(0.5, -0.5 , 0.0);

        cube2.transform.translate(2.0, 0.0, -3.0);
        cube3.transform.translate(0.0, 2.0, 2.0);


        cube1.transform.scale(0.1);
        cube2.transform.scale(0.1);
        cube3.transform.scale(0.1);

        let world = Matrix4::identity();
        let view = Matrix4::from_translation(Vector3::new(0.0, 0.0, 0.0));
        let projection = cgmath::perspective(cgmath::Deg(45.0), 800.0/800.0, 0.1, 100.0);

        Self {
            cube1,
            cube2,
            cube3,
            clock,
            world,
            view,
            projection,
        }
    }
}

impl EventHandler for State {
    fn update(&mut self, _ctx: &mut Context) {
        self.clock.tick();
        let duration = self.clock.get_time_since_start();
        self.cube2.transform.translate(0.00, 0.0 , 0.005);
        self.cube1.transform.translate(0.005, -0.0 , 0.0);
        self.cube3.transform.translate(0.0, -0.005 , 0.0);
        //self.cube.transform.rotate_z(duration.into());
        //self.cube1.transform.rotate(10.0, [0.0, 0.0, 1.0]);
        

    }

    fn draw(&mut self, ctx: &mut Context) {
        ctx.draw(self.cube1.clone());
        ctx.draw(self.cube2.clone());
        ctx.draw(self.cube3.clone());
    }
}

fn main () {

    let mut app = Application::new(800, 800, "Voxel");

    let mut state = State::new(&app);
    app.run(&mut state);


}
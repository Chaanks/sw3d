
use render::Context;
use winit::EventsLoop;
use event::EventHandler;
use time::Clock;
use std::time;

pub struct Application {
    pub ctx: Context,
    events_loop: EventsLoop,
    pub clock: Clock,
    pub last_frame: time::Instant,
}

impl Application {
    pub fn new(width: u32, height: u32, title: &str) -> Self {
        let (ctx, events_loop) = Context::new(width, height, title);
        let clock = Clock::default();
        let last_frame = time::Instant::now();

        Self {
            ctx,
            events_loop,
            clock,
            last_frame,
        }

    }

    pub fn run(&mut self, state: &mut EventHandler) {
        let ctx = &mut self.ctx;

        let mut closed = false;
        while !closed {
            self.clock.tick();

            let now = time::Instant::now();
            let delta = now - self.last_frame;
            let delta_s = delta.as_secs() as f32 + delta.subsec_nanos() as f32 / 1_000_000_000.0;
            self.last_frame = now;
            let fps = (1.0 / delta_s) as i32;
            println!("FPS: {}", fps);
            //println!("FPS: {}", self.clock.get_fps());

            self.events_loop.poll_events(|event| {
                match event {
                    winit::Event::WindowEvent { event: winit::WindowEvent::CloseRequested, .. } => closed = true,
                    _ => (),
                }

            });

            state.update(ctx);
            state.draw(ctx);
            ctx.update();

        }
    }

}
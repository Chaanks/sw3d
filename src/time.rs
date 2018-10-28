use std::time;

pub struct Clock {
    init_instant: time::Instant,
    last_instant: time::Instant,
    frame_durations: time::Duration,
    residual_update_dt: time::Duration,
    frame_count: usize,
}

impl Clock {
    pub fn new() -> Self {
        Self {
            init_instant: time::Instant::now(),
            last_instant: time::Instant::now(),
            frame_durations: time::Duration::new(0, 0),
            residual_update_dt: time::Duration::from_secs(0),
            frame_count: 0,
        }
    }

    pub fn tick(&mut self) {
        let now = time::Instant::now();
        let time_since_last = now - self.last_instant;
        self.frame_durations = time_since_last;
        self.last_instant = now;
        self.frame_count += 1;

        self.residual_update_dt += time_since_last;
    }

    pub fn get_fps(&mut self) -> f64 {
        let duration_per_frame = self.frame_durations;
        let seconds_per_frame = duration_per_frame.as_secs() as f64;
        1.0 / seconds_per_frame
    }

    pub fn get_time_since_start(&mut self) -> f32 {
        (time::Instant::now() - self.init_instant).as_secs() as f32
    }

}

impl Default for Clock {
    fn default() -> Self {
        Self::new()
    }
}
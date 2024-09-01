use crate::engine::gfx::frame::Frame;
use super::timer::Timer;

pub static mut RUNNING: bool = false;

pub struct Engine {
    frame: Frame,
    delta_timer: Timer,
    delta_time: f32,
}

impl Engine {
    pub fn new() -> Result<Engine, String> {
        let _frame = Frame::new("robert einstein",1600, 900);

        let engine = Engine {
            frame: _frame?,
            delta_timer: Timer::new(),
            delta_time: 0.0,
        };

        Ok(engine)
    }

    pub unsafe fn start(&mut self) {
        RUNNING = true;
        self.delta_timer.start();

        while RUNNING {
            self.frame.poll_events();
            // println!("{}", self.delta_time);
            self.delta_time = self.delta_timer.get_ticks() as f32 / 1000.0;
            self.delta_timer.start();

            if self.frame.queued_resize() {
                self.frame.update_scale();
                self.frame.update_frame_space();
            }

            self.frame.render();
        }
    }
}

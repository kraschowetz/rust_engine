pub struct Timer {
    start_time: u32,
    paused_time: u32,
    started: bool,
    paused: bool
}

impl Timer {
    pub fn new() -> Timer {
        let timer: Timer = Timer{
            start_time: 0,
            paused_time: 0,
            started: false,
            paused: false
        };

        return timer
    }

    pub unsafe fn start(&mut self) {
        self.started = true;
        self.paused = false;

        self.start_time = sdl2::sys::SDL_GetTicks();
        self.paused_time = 0;
    }

    pub unsafe fn get_ticks(&self) -> u32 {
        if self.started {
            if self.paused {
                return self.paused_time;
            }
            else {
                return sdl2::sys::SDL_GetTicks() - self.start_time;
            }
        }
        return 0;
    }

/*    pub unsafe fn stop(&mut self) {
        self.started = true;
        self.paused = false;

        self.start_time = 0;
        self.paused_time = 0;
    }

    pub unsafe fn pause(&mut self) {
        if !(self.started && !self.paused) {
            return;
        }

        self.started = false;
        self.paused_time = sdl2::sys::SDL_GetTicks() + self.start_time;
        self.start_time = 0;
    }

    pub unsafe fn resume(&mut self) {
        if !(self.started && self.paused) {
            return;
        }
        
        self.paused = false;

        self.start_time = sdl2::sys::SDL_GetTicks() + self.paused_time;
        self.paused_time = 0;
    }

    */
}

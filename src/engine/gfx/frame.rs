/*
* GfxWindow 
* 
* - renders stuff
* - parses input
*/

use sdl2::{event::{Event, WindowEvent}, pixels::Color, render::Canvas, video::Window, Sdl};

use crate::engine::main::{engine::RUNNING, input_manager::InputManager};

extern crate sdl2;

const ASPECT_RATIO: [i32; 2] = [16, 9];

pub struct Frame {
    canvas: Canvas<Window>,
    event_queue: sdl2::EventPump,
    input_manager: InputManager,

    width: i32,
    height: i32,
    scale: f32,
    frame_space: sdl2::rect::Rect,
    queued_frame_resize: bool
}

impl Frame {
    pub fn new(name: &'static str, width: i32, height: i32) -> Result<Frame, String> {
        let sdl_context = Ok(sdl2::init()?);
        let video_subsystem = <Result<Sdl, String> as Clone>::clone(&sdl_context)?.video()?;
        let window = video_subsystem.window(name, width as u32, height as u32)    //TODO: flex window size
            .position_centered()
            .resizable()
            .build()
            .expect("failet to init window!");
        let mut _canvas = window.into_canvas().build()
            .expect("failed to init canvas!");

        let _event_queue: sdl2::EventPump = <Result<Sdl, String> as Clone>::clone(&sdl_context)?.event_pump()?;

        _canvas.set_blend_mode(sdl2::render::BlendMode::Blend);


        let frame: Frame = Frame {
            canvas: _canvas,
            event_queue: _event_queue,
            input_manager: InputManager::new(),
            width,
            height,
            scale: 1.0,
            frame_space: sdl2::rect::Rect::new(0, 0, width as u32, height as u32),
            queued_frame_resize: false
        };

        Ok(frame)
    }

    pub unsafe fn poll_events(&mut self) {
        for event in self.event_queue.poll_iter() {
            match event{
                Event::Quit {..} => {
                    RUNNING = false
                },
                Event::Window {win_event, ..} => {
                    if let WindowEvent::Resized(w, h) = win_event {
                        self.width = w;
                        self.height = h;
                    }
                    println!("{}, {}", self.width, self.height);
                    self.queued_frame_resize = true;
                },
                _=> {},
            }
        }

        self.input_manager.poll_events(&mut self.event_queue);
    }

    pub fn render(&mut self) {
        self.canvas.set_draw_color(Color::RGB(0, 0, 0));
        self.canvas.clear();

        self.canvas.set_draw_color(Color::RGBA(255, 255, 255, 128));
        self.canvas.fill_rect(self.frame_space).unwrap();

        self.canvas.present();
    }

    pub fn queued_resize(&self) -> bool {
        self.queued_frame_resize
    }

    pub fn update_scale(&mut self) {
        let h_scale: f32 = self.width as f32 / ASPECT_RATIO[0] as f32;
        let v_scale: f32 = self.height as f32 / ASPECT_RATIO[1] as f32;

        if h_scale < v_scale {
            self.scale = h_scale;
        }
        else {
            self.scale = v_scale;
        }
    }

    pub fn update_frame_space(&mut self) {
        self.frame_space.x = ((self.width as f32 - (ASPECT_RATIO[0] as f32 * self.scale)) / 2.0) as i32;
        self.frame_space.y = ((self.height as f32 - (ASPECT_RATIO[1] as f32 * self.scale)) / 2.0) as i32;
        self.frame_space.w = (ASPECT_RATIO[0] as f32 * self.scale) as i32;
        self.frame_space.h = (ASPECT_RATIO[1] as f32 * self.scale) as i32;

        self.queued_frame_resize = false;
    }
}

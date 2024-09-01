/****************************************************************/
/*  frame.rs                                                    */
/****************************************************************/
/*  manages:                                                    */
/*      -game window                                            */
/*      -input polling                                          */
/*      -rendering                                              */
/****************************************************************/

use sdl2::{event::{Event, WindowEvent}, pixels::Color, render::Canvas, video::Window, Sdl};
use rand::Rng;

use crate::core::main::{engine::RUNNING, input_manager::InputManager};

extern crate sdl2;

const ASPECT_RATIO: [i32; 2] = [16, 9];

pub struct Frame {
    canvas: Canvas<Window>,
    event_queue: sdl2::EventPump,
    input_manager: InputManager,

    width: i32,
    height: i32,
    scale: f32,
    render_space: sdl2::rect::Rect,
    queued_frame_resize: bool
}

impl Frame {
    /*  frame constructor   */
    pub fn new(name: &'static str, width: i32, height: i32) -> Result<Frame, String> {
        let sdl_context = Ok(sdl2::init()?);
        let video_subsystem = <Result<Sdl, String> as Clone>::clone(&sdl_context)?.video()?;
        let window = video_subsystem.window(name, width as u32, height as u32)
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
            render_space: sdl2::rect::Rect::new(0, 0, width as u32, height as u32),
            queued_frame_resize: false
        };

        Ok(frame)
    }

    /*  event, polling, get window resized queue, close window  */
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
                    // println!("{}, {}", self.width, self.height);
                    self.queued_frame_resize = true;
                },
                _=> {},
            }
        }

        self.input_manager.poll_events(&mut self.event_queue);
    }

    /*  main render method */
    pub fn render(&mut self) {
        self.canvas.set_draw_color(Color::RGB(0, 0, 0));
        self.canvas.clear();

        self.canvas.set_draw_color(Color::RGBA(255, 255, 255, 64));
        self.canvas.fill_rect(self.render_space).unwrap();
        
        /* stress test: ~65k rects being created & rendered per frame = ~35fps */
        let mut rng = rand::thread_rng();
        for _i in 0..0xFFFF {
            let rect = sdl2::rect::Rect::new(
                rng.gen_range(self.render_space.x, self.render_space.x + self.render_space.w),
                rng.gen_range(self.render_space.y, self.render_space.y + self.render_space.h),
                16,
                16
            );
            self.canvas.set_draw_color(Color::RGB(
                rng.gen_range(0, 255),
                rng.gen_range(0, 255),
                rng.gen_range(0, 255)
            ));
            self.canvas.fill_rect(rect).unwrap();
        }
        /* end of stress test */

        self.canvas.present();
    }

    /* queued resize getter */
    pub fn queued_resize(&self) -> bool {
        self.queued_frame_resize
    }

    /*  update render_space scale   */
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
    
    /*  update render_space rect    */
    pub fn update_render_space(&mut self) {
        self.render_space.x = ((self.width as f32 - (ASPECT_RATIO[0] as f32 * self.scale)) / 2.0) as i32;
        self.render_space.y = ((self.height as f32 - (ASPECT_RATIO[1] as f32 * self.scale)) / 2.0) as i32;
        self.render_space.w = (ASPECT_RATIO[0] as f32 * self.scale) as i32;
        self.render_space.h = (ASPECT_RATIO[1] as f32 * self.scale) as i32;

        self.queued_frame_resize = false;
    }
}

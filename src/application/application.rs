use super::super::*;
use math::*;
use glium;
use glium::{
    glutin::{
        event_loop::{
            EventLoop,
            ControlFlow,
        },
        event::{
            Event,
            StartCause,
            WindowEvent,
        },
        window::WindowBuilder,
        ContextBuilder,      
    },
    Display,
    Program,
};
use std::time::{Instant, Duration};

pub struct Application {
    pub title: &'static str,
    pub frame_time: Duration,
    pub scale_aspect_ratio: bool,
}

impl Application {
    pub fn new() -> Application {
        Application {
            title: "gui application",
            frame_time: Duration::from_secs_f32(1.0/60.0),
            scale_aspect_ratio: true,
        }
    }

    pub fn with_title(mut self, title: &'static str) -> Application {
        self.title = title;
        self
    }

    pub fn with_fps(mut self, fps: f32) -> Application {
        self.frame_time = Duration::from_secs_f32(1.0/fps);
        self
    }

    pub fn scale_aspect_ratio(mut self, scale_aspect_ratio: bool) -> Application {
        self.scale_aspect_ratio = scale_aspect_ratio;
        self
    }

    pub fn run(self, mut state: Box<dyn State>) {
        //
        // initialization
        //

        #[cfg(debug_assertions)]
        println!("GUI::INITIALIZATION Initializing OPEN_GL");

        let event_loop = EventLoop::new();

        let wb = WindowBuilder::new()
            .with_title(self.title);

        let cb = ContextBuilder::new();

        let display = Display::new(wb, cb, &event_loop)
            .expect("GUI::INITIALIZATION Failed to create glium::Display");
        

        // programs

        #[cfg(debug_assertions)]
        println!("GUI::INITIALIZATION Loading Shaders");

        let fill_polygon = Program::from_source(
            &display, 
            include_str!("..\\shaders\\simple_transform.glsl"), 
            include_str!("..\\shaders\\fill.glsl"), 
            None
        ).expect("GUI::INITIALIZATION Failed to load Fill Polygon shader");

        #[cfg(debug_assertions)]
        println!("GUI::INITIALIZATION Fill Polygon program loaded\n");



        //
        // main loop
        //

        #[cfg(debug_assertions)]
        println!("GUI::APPLICATION Starting Application");

        let mut last_frame = Instant::now();

        #[cfg(debug_assertions)]
        println!("GUI::APPLICATION Running main loop");
        
        event_loop.run(move |event, _, flow| {
            *flow = ControlFlow::WaitUntil(Instant::now() + self.frame_time);

            // event handleing

            match event {
                Event::WindowEvent {event, ..} => match event {
                    WindowEvent::CloseRequested => {
                        *flow = ControlFlow::Exit;
                        return;
                    }
                    _ => *flow = ControlFlow::Poll,
                },
                Event::NewEvents(cause) => match cause {
                    StartCause::ResumeTimeReached { .. } => (),
                    StartCause::Init => (),
                    _ => return,
                }, 
                _ => *flow = ControlFlow::Poll,
            } 

            let delta_time = Instant::now().duration_since(last_frame);
            last_frame = Instant::now();

            let dims = display.get_framebuffer_dimensions();

            let w = dims.0 as f32;
            let h = dims.1 as f32;

            let aspect_ratio = w / h;
            
            let window_dimensions = Vec2::new(w, h)/1080.0 * 2.0;

            let state_data = StateData {
                delta_time: delta_time.as_secs_f32(),
                window_dimensions,
                aspect_ratio
            };

            // drawing

            let mut frame = display.draw();

            state.draw(
                Frame {
                    frame: &mut frame,
                    fill_polygon: &fill_polygon,
                    display: &display,
                    scale_aspect_ratio: self.scale_aspect_ratio,
                    window_dimensions: [dims.0 as f32, dims.1 as f32],
                },
                state_data,
            );

            frame.finish()
                .expect("GUI::APPLICATION Failed to finish frame");
        });
    }
}




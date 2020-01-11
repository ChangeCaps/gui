use super::Application;
use glium;
use glium::{
    glutin::{
        event_loop::EventLoop,
        window::WindowBuilder,
        ContextBuilder,
    },
    Display,
};
use std::time::Duration;

pub struct ApplicationBuilder {
    title: &'static str,
    frame_time: Duration,
}

impl ApplicationBuilder {
    pub fn new() -> ApplicationBuilder {
        ApplicationBuilder {
            title: "gui application",
            frame_time: Duration::from_secs_f32(1.0/60.0),
        }
    }

    pub fn with_title(mut self, title: &'static str) -> ApplicationBuilder {
        self.title = title;
        self
    }

    pub fn with_fps(mut self, fps: f32) -> ApplicationBuilder {
        self.frame_time = Duration::from_secs_f32(1.0/fps);
        self
    }

    pub fn build(self) -> Application {
        let event_loop = EventLoop::new();

        let wb = WindowBuilder::new()
            .with_title(self.title);

        let cb = ContextBuilder::new();

        let display = Display::new(wb, cb, &event_loop).expect("failed to create display");
        

        Application {
            event_loop,
            display,
            frame_time: self.frame_time,
        }
    }
}
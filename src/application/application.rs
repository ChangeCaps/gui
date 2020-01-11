use super::ApplicationBuilder;
use super::super::State;
use super::super::Frame;
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
    },
    Display,
};
use std::time::{Instant, Duration};

pub struct Application {
    pub display: Display,
    pub event_loop: EventLoop<()>, 
    pub frame_time: Duration,
}

impl Application {
    pub fn new() -> ApplicationBuilder {
        ApplicationBuilder::new()
    }

    pub fn run(self, mut state: Box<dyn State>) {

        let frame_time = self.frame_time;
        let display = self.display;

        self.event_loop.run(move |event, _, flow| {
            *flow = ControlFlow::WaitUntil(Instant::now() + frame_time);

            match event {
                Event::WindowEvent {event, ..} => match event {
                    WindowEvent::CloseRequested => {
                        *flow = ControlFlow::Exit;
                        return;
                    }
                    _ => return,
                },
                Event::NewEvents(cause) => match cause {
                    StartCause::ResumeTimeReached { .. } => (),
                    StartCause::Init => (),
                    _ => return,
                }, 
                _ => return,
            }

            let mut frame = display.draw();

            state.draw(Frame::new(&mut frame));

            frame.finish().expect("failed to finish frame");
        });
    }
}




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
            ElementState,
            DeviceEvent,
        },
        window::WindowBuilder,
        ContextBuilder,
    },
    Display,
    Program,
};
use std::time::{Instant, Duration};
use std::collections::HashSet;

pub struct Application {
    pub title: &'static str,
    pub frame_time: Duration,
    pub aspect_ratio: Option<f32>,
    pub resizable: bool,
    pub window_size: Vec2<u32>,
}

impl Application {
    pub fn new() -> Application {
        Application {
            title: "gui application",
            frame_time: Duration::from_secs_f32(1.0/60.0),
            aspect_ratio: None,
            resizable: true,
            window_size: Vec2::new(800, 600),
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

    pub fn with_aspect_ratio(mut self, aspect_ratio: f32) -> Application {
        self.aspect_ratio = Some(aspect_ratio);
        self
    }

    pub fn not_resizable(mut self) -> Application {
        self.resizable = false;
        self
    }

    pub fn with_window_size(mut self, width: u32, height: u32) -> Application {
        self.window_size = Vec2::new(width, height);
        self
    }

    pub fn run<F>(self, mut start: F) 
        where F: FnMut(&mut super::super::Loader) -> Box<dyn State>
    {
        //
        // initialization
        //

        #[cfg(debug_assertions)]
        println!("GUI::INITIALIZATION Initializing OPEN_GL"); 
        
        let event_loop = EventLoop::new();

        let wb = WindowBuilder::new()
            .with_resizable(self.resizable)
            .with_inner_size(
                glium::glutin::dpi::LogicalSize::new(self.window_size.x as f64, self.window_size.y as f64)
            )
            .with_title(self.title);

        let cb = ContextBuilder::new();

        let display = Display::new(wb, cb, &event_loop)
            .expect("GUI::INITIALIZATION Failed to create glium::Display");
        

        // programs

        #[cfg(debug_assertions)]
        println!("GUI::INITIALIZATION Loading Shaders");

        let simple_transform_fill = Program::from_source(
            &display, 
            include_str!("../shaders/vertex/simple_transform.glsl"), 
            include_str!("../shaders/fragment/fill.glsl"), 
            None
        ).expect("GUI::INITIALIZATION Failed to load Simple Transform Fill shader");

        #[cfg(debug_assertions)]
        println!("GUI::INITIALIZATION Simple Transform Fill program loaded");

        let simple_transform_ellipse = Program::from_source(
            &display, 
            include_str!("../shaders/vertex/simple_transform.glsl"), 
            include_str!("../shaders/fragment/ellipse.glsl"), 
            None
        ).expect("GUI::INITIALIZATION Failed to load Simple Transform Ellipse shader");

        #[cfg(debug_assertions)]
        println!("GUI::INITIALIZATION Simple Transform Ellipse program loaded");

        let no_transform_line = Program::from_source(
            &display, 
            include_str!("../shaders/vertex/no_transform.glsl"), 
            include_str!("../shaders/fragment/line.glsl"), 
            None
        ).expect("GUI::INITIALIZATION Failed to load No Transform Line shader");
        
        #[cfg(debug_assertions)]
        println!("GUI::INITIALIZATION No Transform Line program loaded\n");

        let texture = Program::from_source(
            &display, 
            include_str!("../shaders/vertex/texture.glsl"), 
            include_str!("../shaders/fragment/texture.glsl"), 
            None
        ).expect("GUI::INITIALIZATION Failed to load Texture shader");
        
        #[cfg(debug_assertions)]
        println!("GUI::INITIALIZATION Texture loaded\n");   

        let text = Program::from_source(
            &display, 
            include_str!("../shaders/vertex/texture.glsl"), 
            include_str!("../shaders/fragment/text.glsl"), 
            None
        ).expect("GUI::INITIALIZATION Failed to load Text shader");
        
        #[cfg(debug_assertions)]
        println!("GUI::INITIALIZATION Text loaded\n");   


        //
        // main loop
        //

        #[cfg(debug_assertions)]
        println!("GUI::APPLICATION Starting Application");

        let mut last_frame = Instant::now();
        let mut mouse_position = Vec2::new(0.0, 0.0);
        let mut scaled_mouse_position = Vec2::new(0.0, 0.0);

        let window_dimensions_multiplier = {
            let buffer_dimensions = display.get_framebuffer_dimensions();
            let buffer_dimensions = Vec2::new(buffer_dimensions.0 as f32, buffer_dimensions.1 as f32);

            let window_dimensions = Vec2::new(self.window_size.x as f32, self.window_size.y as f32);

            buffer_dimensions / window_dimensions
        };

        let aspect_ratio = self.aspect_ratio.unwrap_or(self.window_size.x as f32/self.window_size.y as f32);

        // inputs
        
        let mut keys_held = HashSet::new();
        let mut mouse_buttons_held = HashSet::new();
            
        let mut keys_pressed = HashSet::new();
        let mut keys_released = HashSet::new();

        let mut mouse_buttons_pressed = HashSet::new();
        let mut mouse_buttons_released = HashSet::new();


        #[cfg(debug_assertions)]
        println!("GUI::APPLICATION Running start function");

        let mut images = Vec::new();
        let mut fonts = Vec::new();
        let mut text_inputs = Vec::new();

        let mut loader = super::super::Loader {
            display: &display,
            images: &mut images,
            fonts: &mut fonts,
            text_inputs: &mut text_inputs,
        };

        let mut states = vec![start(&mut loader)];

        // used to ensure that we don't go above the desired frame rate
        let mut next_frame_time = Instant::now() + self.frame_time;

        #[cfg(debug_assertions)]
        println!("GUI::APPLICATION Running main loop");

        event_loop.run(move |event, _, flow| {
            // update next_frame_time
            if next_frame_time <= Instant::now() {
                next_frame_time = Instant::now() + self.frame_time;
            }

            // set ControlFlow
            *flow = ControlFlow::WaitUntil(next_frame_time);

            // get window dimensions
            let dims = display.get_framebuffer_dimensions();

            let w = dims.0 as f32;
            let h = dims.1 as f32;
            
            let scaled_aspect_ratio = w / h;
            
            let frame_dimensions = Vec2::new(aspect_ratio, 1.0);
            let scaled_frame_dimensions = Vec2::new(scaled_aspect_ratio, 1.0);
            let window_dimensions = Vec2::new(w, h);

            // event handling 
            match event {
                Event::WindowEvent {event, ..} => match event {
                    WindowEvent::CloseRequested => {
                        *flow = ControlFlow::Exit;
                        return;
                    },
                    WindowEvent::CursorMoved {position, ..} => {
                        mouse_position = Vec2::new(position.x as f32, position.y as f32) * window_dimensions_multiplier / window_dimensions * 2.0 - 1.0;
                        mouse_position.y = -mouse_position.y;

                        scaled_mouse_position = mouse_position;
                        scaled_mouse_position.x *= scaled_aspect_ratio;

                        mouse_position.x *= aspect_ratio;

                        return;
                    },
                    WindowEvent::KeyboardInput {input, ..} => {
                        match input.state {
                            ElementState::Pressed => {
                                input.virtual_keycode.map(|key| { 
                                    keys_held.insert(key);
                                    keys_pressed.insert(key);
                                });
                            },
                            ElementState::Released => { 
                                input.virtual_keycode.map(|key| { 
                                    keys_held.remove(&key);
                                    keys_released.insert(key);
                                });
                            }
                        }
                        
                        return;
                    },
                    WindowEvent::MouseInput {button, state, ..} => {
                        match state {
                            ElementState::Pressed => {
                                mouse_buttons_held.insert(button);
                                mouse_buttons_pressed.insert(button);
                            },
                            ElementState::Released => { 
                                mouse_buttons_held.remove(&button);
                                mouse_buttons_released.insert(button);
                            },
                        }

                        return;
                    },
                    WindowEvent::ReceivedCharacter(c) => { 
                        text_inputs.iter_mut().for_each(|input| {
                            match c as u8 {
                                13 => {
                                    
                                },
                                08 => {
                                    let mut s = input.borrow_mut();

                                    s.0.pop();
                                },
                                _ => {
                                    let mut s = input.borrow_mut();
     
                                    if true {
                                        s.0.push(c);
                                    }
                                }
                            }
                        });                        

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

            // state data

            let delta_time = Instant::now().duration_since(last_frame);
            last_frame = Instant::now();

            let state_data = StateData {
                delta_time: delta_time.as_secs_f32(),
                frame_dimensions,
                scaled_frame_dimensions,
                window_dimensions: Vec2::new(w, h),
                aspect_ratio,
                mouse_position,
                scaled_mouse_position,
                keys_pressed: &keys_pressed,
                keys_held: &keys_held,
                keys_released: &keys_released,
                mouse_buttons_pressed: &mouse_buttons_pressed,
                mouse_buttons_held: &mouse_buttons_held,
                mouse_buttons_released: &mouse_buttons_released,
            };

            let mut frame = display.draw();

            if states.len() == 0 {
                frame.finish()
                    .expect("GUI::APPLICATION Failed to finish frame");

                return;
            }

            let index = states.len() - 1;
        
            states.iter_mut().for_each(|state| state.draw(
                &mut Frame {
                    frame: &mut frame,
                    simple_transform_fill: &simple_transform_fill,
                    simple_transform_ellipse: &simple_transform_ellipse,
                    no_transform_line: &no_transform_line,
                    texture: &texture,
                    display: &display,
                    text: &text,
                    window_dimensions: Vec2::new(w, h),
                    aspect_ratio,
                    scaled_aspect_ratio,
                    images: &images,
                    fonts: &fonts
                },
                &state_data,
            ));

            let trans = states[index].update(&state_data);

            states.iter_mut().for_each(|state| state.shadow_update(&state_data));

            keys_pressed = HashSet::new();
            mouse_buttons_pressed = HashSet::new();

            keys_released = HashSet::new();
            mouse_buttons_released = HashSet::new();

            frame.finish()
                .expect("GUI::APPLICATION Failed to finish frame");

            match trans {
                Transition::Trans(state) => {
                    states = vec![state];
                },
                Transition::Push(state) => {
                    states.push(state);
                },
                Transition::Pop => {
                    states.pop();

                    if states.len() == 0 {
                        *flow = ControlFlow::Exit;
                    }
                },
                Transition::None => (),
            }
        });
    }
}




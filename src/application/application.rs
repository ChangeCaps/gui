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
        },
        window::WindowBuilder,
        ContextBuilder,
    },
    Display,
    Program,
    Surface,
};
use std::time::{Instant, Duration};
use std::collections::{HashMap, HashSet};

pub struct Application {
    pub title: &'static str,
    pub frame_time: Duration,
    pub aspect_ratio: Option<f32>,
    pub resizable: bool,
    pub window_size: Vec2<u32>,
    pub pixel_window_size: Option<Vec2<u32>>,
    pub depth_sorting: bool,
}

impl Application {
    pub fn new() -> Application {
        Application {
            title: "gui application",
            frame_time: Duration::from_secs_f32(1.0/60.0),
            aspect_ratio: None,
            resizable: true,
            window_size: Vec2::new(800, 600),
            pixel_window_size: None,
            depth_sorting: false,
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

    pub fn with_pixel_window_size(mut self, width: u32, height: u32) -> Application {
        self.pixel_window_size = Some(Vec2::new(width, height));
        self
    }

    pub fn with_depth_sorting(mut self, depth_sorting: bool) -> Application {
        self.depth_sorting = depth_sorting;
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
        println!("GUI::APPLICATION Starting Application");

        let mut last_frame = Instant::now();
        let mut mouse_position = Vec2::new(0.0, 0.0);
        let mut scaled_mouse_position = Vec2::new(0.0, 0.0);

        let buffer_dimensions = display.get_framebuffer_dimensions();
        let buffer_dimensions_u32 = Vec2::new(buffer_dimensions.0, buffer_dimensions.1);
        let buffer_dimensions = Vec2::new(buffer_dimensions.0 as f32, buffer_dimensions.1 as f32);

        let window_dimensions = Vec2::new(self.window_size.x as f32, self.window_size.y as f32);

        let window_dimensions_multiplier = buffer_dimensions / window_dimensions;

        let aspect_ratio = if let Some(size) = self.pixel_window_size {
            size.x as f32 / size.y as f32
        } else {
            self.aspect_ratio.unwrap_or(self.window_size.x as f32/self.window_size.y as f32)
        };

        // inputs
        
        let mut keys_held = HashSet::new();
        let mut mouse_buttons_held = HashSet::new();
            
        let mut keys_pressed = HashSet::new();
        let mut keys_released = HashSet::new();

        let mut mouse_buttons_pressed = HashSet::new();
        let mut mouse_buttons_released = HashSet::new();


        #[cfg(debug_assertions)]
        println!("GUI::APPLICATION Running start function");

        let mut image_indecies = HashMap::new();
        let mut image_dimensions = Vec::new();
        let mut font_indecies = HashMap::new();
        let mut font_character_infos = Vec::new(); 
        let mut font_dimensions = Vec::new();
        let mut text_inputs = Vec::new();

        let mut loader = super::super::Loader {
            display: &display,
            image_indecies: &mut image_indecies,
            font_indecies: &mut font_indecies,
            images: Vec::new(),
            fonts: Vec::new(),
            image_dimensions: &mut image_dimensions,
            font_dimensions: &mut font_dimensions,
            font_character_infos: &mut font_character_infos,
            text_inputs: &mut text_inputs,
        };

        let mut states = vec![start(&mut loader)];

        
        let (image_atlas, image_positions, image_atlas_dimensions) = crate::texture_atlas::crate_atlas(&display, &mut loader.images, &mut loader.image_dimensions);
        let (font_atlas, font_positions, font_atlas_dimensions) = crate::texture_atlas::crate_atlas(&display, &mut loader.fonts, &mut loader.font_dimensions);

        // if pixel mode is set, there is no reason to keep remaking the frame buffer every frame
        // therefore we make it now and clear it every frame which is considerably faster

        let size = self.pixel_window_size.unwrap_or(buffer_dimensions_u32);

        // create texture_buffer
        let mut texture_buffer = glium::texture::texture2d::Texture2d::empty(
            &display,
            size.x, 
            size.y
        ).expect("failed to create texture buffer");

        // used to ensure that we don't go above the desired frame rate
        let mut next_frame_time = Instant::now() + self.frame_time;
        let mut last_frame_vertex_count = 128;

        #[cfg(debug_assertions)]
        println!("GUI::APPLICATION Running main loop");

        // main loop
        event_loop.run(move |event, _, flow| { 
            // if there are no states, close the application
            if states.len() == 0 {
                *flow = ControlFlow::Exit;

                return;
            } 

            // update next_frame_time
            if next_frame_time <= Instant::now() {
                next_frame_time = Instant::now() + self.frame_time;
            }

            // set ControlFlow as to wait until the time for the next frame is reached before
            // redrawing
            *flow = ControlFlow::WaitUntil(next_frame_time);

            // get window dimensions
            let dims = display.get_framebuffer_dimensions();

            // dims as f32
            let w = dims.0 as f32;
            let h = dims.1 as f32;
            
            // used for scaling shapes
            let scaled_aspect_ratio = w / h;
            
            let frame_dimensions = Vec2::new(aspect_ratio * 2.0, 2.0);
            let scaled_frame_dimensions = Vec2::new(scaled_aspect_ratio * 2.0, 2.0);
            let window_dimensions = Vec2::new(w, h);

            // event handling 
            match event {
                Event::WindowEvent {event, ..} => match event {
                    // if the window requests closing it, do so
                    WindowEvent::CloseRequested => {
                        *flow = ControlFlow::Exit;
                        return;
                    },
                    // update cursor position when it is moved
                    WindowEvent::CursorMoved {position, ..} => {
                        mouse_position = (Vec2::new(
                            position.x as f32, 
                            position.y as f32
                        ) * window_dimensions_multiplier 
                          / window_dimensions * 2.0 - 1.0) * if let Some(size) = self.pixel_window_size {
                              size.y as f32 / 2.0
                          } else {
                              1.0
                          };
                        mouse_position.y = -mouse_position.y;

                        scaled_mouse_position = mouse_position;
                        scaled_mouse_position.x *= scaled_aspect_ratio;

                        mouse_position.x *= aspect_ratio;

                        return;
                    },
                    // record keyboard inputs
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
                    // record mouse inputs
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
                        // go through each text inputs and modify their text according to the given
                        // character input
                        text_inputs.iter_mut().for_each(|input| {
                            // since the inputs are of the type Rc<RefCell<(String, bool)>> we need
                            // to use borrow_mut to mutate it
                            let mut s = input.borrow_mut();

                            // if the text input is not reading, do not modify its text
                            if !s.1 {
                                return;
                            }

                            match c as u8 {
                                // ignore carriage returns
                                13 => {
                                    
                                },
                                // in the case of backspace pop a character from the text
                                08 => {
                                    s.0.pop();
                                },
                                // default to pushing the character to the String
                                _ => {    
                                    s.0.push(c);
                                }
                            }
                        });                        

                        return;
                    },
                    WindowEvent::Resized(size) => {
                        if self.pixel_window_size.is_none() {
                            texture_buffer = glium::texture::texture2d::Texture2d::empty(
                                &display,
                                size.width as u32,
                                size.height as u32,
                            ).expect("failed to create texture buffer for resized window");
                        }
                    },
                    _ => return,
                }, 
                Event::NewEvents(cause) => match cause {
                    // update the screen if the time limit is reached
                    StartCause::ResumeTimeReached { .. } => (),
                    StartCause::Init => (),
                    _ => return,
                }, 
                _ => return,
            } 

            // calculate delta-time
            let delta_time = Instant::now().duration_since(last_frame);
            last_frame = Instant::now(); // reset last frame

            // crate state data
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

            // get a frame for drawing to the window and clear it
            let mut frame = display.draw();
            frame.clear_color_and_depth((0.0, 0.0, 0.0, 1.0), 0.0);

            // calculate the index of the current state
            let index = states.len() - 1;
        
            let (w, h, aspect_ratio) = if let Some(size) = self.pixel_window_size {
                (size.x, size.y, size.x as f32/size.y as f32)
            } else {
                (dims.0, dims.1, aspect_ratio)
            };
          
            // run state functions
            let trans = {
                // construct drawing data
                let mut drawing_data = DrawingData {
                    pixel_window_dimensions:    self.pixel_window_size.map(|size| Vec2::new(size.x as f32, size.y as f32)),
                    line_points:                Vec::new(),
                    line_widths:                Vec::new(),
                    verts:                      Vec::with_capacity(last_frame_vertex_count),

                    // FIXME: cloning is bad, find another way
                    image_indecies:             image_indecies.clone(),
                    image_dimensions:           image_dimensions.clone(),
                    font_indecies:              font_indecies.clone(),
                    font_dimensions:            font_dimensions.clone(),
                    font_character_infos:       font_character_infos.clone(),
                    scaled_aspect_ratio:        scaled_aspect_ratio,
                    aspect_ratio:               aspect_ratio,
                    font_atlas_dimensions:      font_atlas_dimensions,
                    font_positions:             font_positions.clone(),
                    image_atlas_dimensions:     image_atlas_dimensions,
                    image_positions:            image_positions.clone(),
                };

                let mut _frame = Frame { 
                    drawing_data: &mut drawing_data,
                };
                
                // run update for current state
                let trans = states[index].update(&state_data);
    
                // run shadow update for all states
                states.iter_mut().for_each(|state| state.shadow_update(&state_data));
    
                // run draw for current state
                states[index].draw(
                    &mut _frame,
                    &state_data,
                );

                // run shadow draw for all states
                states.iter_mut().for_each(|state| state.shadow_draw(
                    &mut _frame,
                    &state_data,
                )); 
    
                // set last frame vertex count
                last_frame_vertex_count = _frame.drawing_data.verts.len();

                // clear color
                texture_buffer.as_surface().clear_color(0.0, 0.0, 0.0, 1.0); 

                // line buffers

                let line_point_buffer = glium::buffer::Buffer::<[[f32; 4]]>::new(
                    &display, 
                    &_frame.drawing_data.line_points,
                    glium::buffer::BufferType::ArrayBuffer,
                    glium::buffer::BufferMode::Default,
                ).unwrap();

                let line_width_buffer = glium::buffer::Buffer::<[f32]>::new(
                    &display, 
                    &_frame.drawing_data.line_widths,
                    glium::buffer::BufferType::ArrayBuffer,
                    glium::buffer::BufferMode::Default,
                ).unwrap();

                // uniforms for scaling draw call
                let uniforms = uniform!{
                    window_dimensions: [w as f32, h as f32],

                    // line buffers
                    line_point_buffer: &line_point_buffer,
                    line_width_buffer: &line_width_buffer,

                    // image
                    image_atlas: &image_atlas,
                    image_atlas_dimensions: image_atlas_dimensions.as_array(),

                    // text
                    font_atlas: &font_atlas,
                    font_atlas_dimensions: font_atlas_dimensions.as_array(),
                };

                if _frame.drawing_data.verts.len() > 0 {
                    // FIXME: creating a new vertex buffer every frame is slow, but for some
                    // reason it would keep crashing if I were to write

                    if self.depth_sorting {
                        _frame.drawing_data.verts.sort_by(|a, b| a.depth.partial_cmp(&b.depth).unwrap());
                    }

                    let vertex_buffer = glium::VertexBuffer::new(&display, &_frame.drawing_data.verts).unwrap();

                    let draw_parameters = glium::draw_parameters::DrawParameters {
                        blend: glium::Blend::alpha_blending(), 
                        .. Default::default()
                    };
    
                    let t = std::time::Instant::now();
                
                    // draw the frame buffer to the window and handle errors
                    let _ = frame.draw(&vertex_buffer,
                                       &glium::index::NoIndices(glium::index::PrimitiveType::TriangleStrip), 
                                       &simple_transform_fill,
                                       &uniforms,
                                       &draw_parameters);

                    
                    frame.finish()
                        .expect("GUI::APPLICATION Failed to finish frame");
                }
   
                trans
            };
            
            // 
            // transition handling
            //

            // reset keypressed and released
            keys_pressed = HashSet::new();
            keys_released = HashSet::new();
            
            // reset mousepressed and released
            mouse_buttons_pressed = HashSet::new();
            mouse_buttons_released = HashSet::new();

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




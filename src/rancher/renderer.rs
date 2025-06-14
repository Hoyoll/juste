
use std::{collections::HashSet, mem::replace, sync::Arc};

use pixels::{ Pixels, SurfaceTexture};
use winit::{
    application::ApplicationHandler, event::{ ElementState, MouseButton, WindowEvent}, event_loop::{ ActiveEventLoop, EventLoop}, keyboard::{KeyCode, PhysicalKey}, window::{Window, WindowAttributes, WindowId}
    
};

use super::{element::Element, io::{Button, Input, Key, Mouse, On, When, Win}, vector::Vec2};

pub fn run<T: ApplicationHandler>(renderer: &mut T) {
    let event_loop = EventLoop::new().unwrap();
    let _ = event_loop.run_app(renderer);
}

struct Renderer<'a> {
    elements: &'a mut Element,
    attr: WindowAttributes,
    window: Option<Arc<Window>>,
    pixels: Option<Pixels<'a>>,
    input: Input,
    bucket: Option<HashSet<On>>
}

impl<'a> Renderer<'a> {
    pub fn build(elements: &'a mut Element, attr: WindowAttributes) -> Self {
        Self {
            elements,
            attr,
            window: None,
            pixels: None,
            input: Input::None,
            bucket: None
        }
    }

    fn draw(&mut self, event_loop: &ActiveEventLoop) {
        if let Some(pixels) = self.pixels.as_mut() {
            let frame = pixels.frame_mut();
            frame.fill(0); // Clear to black

            // Example: draw red rectangle
            draw_rect(frame, 100, 100, 100, 50, [255, 0, 0, 255], 640);
            draw_circle(frame, 120, 200, 50, [0, 0, 255, 255], 640, 480);
            if pixels.render().is_err() {
                eprintln!("Render error");
                event_loop.exit();
            } else {
                self.window.as_ref().unwrap().request_redraw(); // Continuous redraw
            }
        }
    }

    fn input_pool(&mut self, event: On) {
        match &mut self.input {
            Input::None => {
                self.input = Input::Single(event);
            }

            Input::Single(input) => {
                let mut hash = self.bucket.take().unwrap_or_else(HashSet::new);
                hash.insert(*input);
                hash.insert(event);
                self.input = Input::Combo(hash);
            }
            Input::Combo(hash) => {
                hash.insert(event);
            }
        }
    }

    fn check_io(&mut self) {
        self.elements.listen(&self.input);
    }

    fn clean_pipe(&mut self) {
        if let Input::Combo(mut hash) = replace(&mut self.input, Input::None) {
            hash.clear();
            self.bucket = Some(hash);
        }
    }
   

    fn key_filter(&self, key: KeyCode) -> Key {
        //to-do i suppose
    }

    fn mouse_filter(&self, button: MouseButton) -> Mouse {
        //to-do i suppose
    }
}

impl<'a> ApplicationHandler for Renderer<'a> {
    fn resumed(&mut self, event_loop: &ActiveEventLoop) {
        // Create window
        let window = Arc::new(event_loop.create_window(self.attr.clone()).unwrap());

        let size = window.inner_size();
        let surface_texture = SurfaceTexture::new(size.width, size.height, window.clone());

        let pixels = Pixels::new(640, 480, surface_texture).expect("Failed to create Pixels");
        self.window = Some(window);
        self.pixels = Some(pixels);

        self.window.as_ref().unwrap().request_redraw();
    }

    fn window_event(&mut self, event_loop: &ActiveEventLoop, _id: WindowId, event: WindowEvent) {
        match event {
            WindowEvent::KeyboardInput {device_id: _, event, .. } => {
                
                if let PhysicalKey::Code(code) = event.physical_key {
                    let key = self.key_filter(code);
                        
                    let when = match event.state {
                        ElementState::Pressed => Button::Press(key),
                        ElementState::Released => Button::Release(key),
                    };
                    self.input_pool(On::Key(when));                
                 }   
            }

            WindowEvent::MouseInput {device_id: _, state, button } => {
                let mouse = self.mouse_filter(button);        
                let when = match state {
                    ElementState::Pressed => When::Press(mouse)
                    ElementState::Released => When::Release(mouse)
                };
                self.input_pool(On::Mouse(when));
            }
            
            WindowEvent::RedrawRequested => {
                self.draw(event_loop);
            }

            WindowEvent::CloseRequested => {
                println!("The close button was pressed; stopping");
                event_loop.exit();
            }

            WindowEvent::Resized(new_size) => {
                println!("Window resized!");
                if let Some(pixels) = self.pixels.as_mut() {
                    let _ = pixels.resize_surface(new_size.width, new_size.height);
                }
            }

            WindowEvent::Moved(pos) => {
                self.input_pool(On::Window(Win::Move(Vec2::new(pos.x as u32, pos.y as u32))));
            }

            WindowEvent::MouseWheel { device_id: _, delta, phase } => {
                match delta {
                    
                }
                self.input_pool(On::Window(Win::Scroll { line_delta: , pixel_delta: () });

                match phase {
                    
                }
            }

            _ => ()
        }
    }
}

fn draw_rect(frame: &mut [u8], x: u32, y: u32, w: u32, h: u32, color: [u8; 4], screen_width: u32) {
    let len = frame.len();
    const BYTE: u32 = 4;
    for row in y..(y + h) {
        for col in x..(x + w) {
            // Calculate the index in the flat RGBA buffer
            let idx = get_frame_index(row, col, screen_width, BYTE);

            if idx + 3 < len {
                frame[idx..idx + 4].copy_from_slice(&color);
            }
        }
    }
}

fn get_frame_index(y: u32, x: u32, canvas_width: u32, byte: u32) -> usize {
    ((y * canvas_width + x) * byte) as usize
}

fn draw_circle(
    frame: &mut [u8],
    cx: i32,
    cy: i32,
    radius: i32,
    color: [u8; 4],
    screen_width: u32,
    screen_height: u32,
) {
    let len = frame.len();
    const BYTE: u32 = 4;

    // Bounding box of the circle
    let x_min = (cx - radius).max(0);
    let x_max = (cx + radius).min(screen_width as i32 - 1);
    let y_min = (cy - radius).max(0);
    let y_max = (cy + radius).min(screen_height as i32 - 1);

    for y in y_min..y_max {
        for x in x_min..x_max {
            let dx = x - cx;
            let dy = y - cy;

            if dx * dx + dy * dy <= radius * radius {
                let idx = get_frame_index(y as u32, x as u32, screen_width, BYTE);

                if idx + 4 <= len {
                    frame[idx..idx + 4].copy_from_slice(&color);
                }
            }
        }
    }
}

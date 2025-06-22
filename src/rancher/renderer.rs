use std::{
    collections::{HashMap, HashSet},
    mem::{replace, take},
    sync::Arc,
    time::{Duration, Instant},
};

use pixels::{Pixels, SurfaceTexture};
use tiny_skia::{Color, Pixmap};
use winit::{
    application::ApplicationHandler,
    event::{ElementState, MouseButton, MouseScrollDelta, StartCause, TouchPhase, WindowEvent},
    event_loop::{ActiveEventLoop, ControlFlow, EventLoop},
    keyboard::{KeyCode, PhysicalKey},
    window::{Window, WindowAttributes, WindowId},
};

use super::{
    element::{Code, Element, Genus, Island},
    io::{Button, Delta, Input, Key, Mouse, On, Phase, Point, When, Win},
    vector::Vec2,
};

pub fn run<T: ApplicationHandler>(renderer: &mut T) {
    let event_loop = EventLoop::new().unwrap();
    let _ = event_loop.run_app(renderer);
}

struct Renderer<'a> {
    islands: &'a mut Island<'a>,
    messages: HashMap<i8, Code>,
    attr: WindowAttributes,
    window: Option<Arc<Window>>,
    pixels: Option<Pixels<'a>>,
    canvas: Option<Pixmap>,
    input: Input,
    bucket: Option<HashSet<On>>,
    m_pos: Vec2,
}

impl<'a> Renderer<'a> {
    pub fn build(islands: &'a mut Island<'a>, attr: WindowAttributes) -> Self {
        Self {
            islands,
            messages: HashMap::new(),
            attr,
            window: None,
            pixels: None,
            canvas: None,
            input: Input::None,
            bucket: None,
            m_pos: Vec2::new(0, 0),
        }
    }

    fn pos(&mut self, element: &mut Element) {}

    fn draw(&mut self, event_loop: &ActiveEventLoop) {
        if let Some(pixels) = self.pixels.as_mut() {
            let frame = pixels.frame_mut();
            let canvas = self.canvas.as_mut().unwrap();
            canvas.fill(Color::WHITE);
            dim(self.islands, &self.input, &mut self.messages);
            pos(self.islands, canvas);
            frame.copy_from_slice(canvas.data());
            if pixels.render().is_err() {
                eprintln!("Render error");
                event_loop.exit();
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
            _ => (),
        }
    }

    fn check_io(&mut self) {}

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

fn io(island: &mut Island, input: Input, mouse_pos: &Vec2, bucket: &Option<HashSet<On>>) {
    for i in 0..island.member.len() {
        let member = &mut island.member[i];
        if let Some(isle) = &mut member.children.as_mut() {
            io(isle, input, mouse_pos, bucket);
        }
        io_real(member, input, mouse_pos, bucket);
    }
}

fn io_real(
    element: &mut Element,
    input: &mut Input,
    mouse_pos: &Vec2,
    bucket: &Option<HashSet<On>>,
) {
}

fn pos(island: &mut Island, canvas: &mut Pixmap) {
    for i in 0..island.member.len() {
        let member = &mut island.member[i];
        real_pos(member, canvas);
        if let Some(isle) = member.children.as_mut() {
            pos(isle, canvas);
        }
    }
}

fn real_pos(element: &mut Element, canvas: &mut Pixmap) {}

// fn dim(island: &mut Island, input: &mut Input) {
//     let members = take(&mut island.member);
//     for mut member in members {
//         let code = member.listen(input);
//         island.hear(code);
//         if let Some(ref mut isle) = member.children {
//             dim(isle, input);
//         }
//         real_dim(&mut member);
//         island.member.push(member);
//     }
// }
fn dim(island: &mut Island, input: &Input, bus: &mut HashMap<i8, Code>) {
    let len = island.member.len();
    for i in 0..len {
        let mem = island.member.get_mut(i);
        let c: Option<Code> = mem.and_then(|m| m.listen(input));
        if let Some((idx, code)) = island.hear(c) {
            bus.insert(idx, code);
        }
        island.deliver(bus);
        if let Some(member) = island.member.get_mut(i) {
            if let Some(isle) = &mut member.children.as_mut() {
                dim(isle, input, bus);
            }
            real_dim(member);
        }
    }
}

fn real_dim(element: &mut Element) {
    let mut bound = &mut element.bound;
    match &mut element.genus {
        Genus::Box {
            style,
            height,
            width,
            radius,
        } => {}
        Genus::Img { file_name, style } => {}
        Genus::Text {
            text,
            size,
            font_path,
            style,
        } => {}
    }
}

impl<'a> ApplicationHandler for Renderer<'a> {
    fn new_events(&mut self, event_loop: &ActiveEventLoop, cause: StartCause) {
        match cause {
            StartCause::ResumeTimeReached { .. } => {
                self.check_io();
                self.draw(event_loop);
                self.window.as_ref().unwrap().request_redraw();
                event_loop.set_control_flow(ControlFlow::WaitUntil(
                    Instant::now() + Duration::from_millis(16),
                ));
            }
            _ => (),
        }
    }

    fn resumed(&mut self, event_loop: &ActiveEventLoop) {
        let window = Arc::new(event_loop.create_window(self.attr.clone()).unwrap());
        let size = window.inner_size();
        let surface_texture = SurfaceTexture::new(size.width, size.height, window.clone());
        let pixels =
            Pixels::new(size.width, size.height, surface_texture).expect("failed to create Pixels");
        self.window = Some(window);
        self.pixels = Some(pixels);
        self.canvas = Pixmap::new(size.width, size.height);
        event_loop.set_control_flow(ControlFlow::WaitUntil(Instant::now()));
    }

    fn window_event(&mut self, event_loop: &ActiveEventLoop, _id: WindowId, event: WindowEvent) {
        match event {
            WindowEvent::KeyboardInput {
                device_id: _,
                event,
                ..
            } => {
                if let PhysicalKey::Code(code) = event.physical_key {
                    let key = self.key_filter(code);

                    let when = match event.state {
                        ElementState::Pressed => Button::Press(key),
                        ElementState::Released => Button::Release(key),
                    };
                    self.input_pool(On::Key(when));
                }
            }

            WindowEvent::MouseInput {
                device_id: _,
                state,
                button,
            } => {
                let mouse = self.mouse_filter(button);
                let when = match state {
                    ElementState::Pressed => When::Press(mouse),
                    ElementState::Released => When::Release(mouse),
                };
                self.input_pool(On::Mouse(when));
            }

            WindowEvent::CloseRequested => {
                self.input_pool(On::Window(Win::Close));
                self.check_io();
                event_loop.exit();
            }

            WindowEvent::Resized(new_size) => match &mut self.pixels.as_mut() {
                Some(pixels) => {
                    pixels.resize_surface(new_size.width, new_size.height);
                    self.canvas = Pixmap::new(new_size.width, new_size.height);
                    self.input_pool(On::Window(Win::Resize {
                        width: new_size.width,
                        height: new_size.height,
                    }));
                }
                _ => (),
            },

            WindowEvent::Moved(pos) => {
                self.input_pool(On::Window(Win::Move { x: pos.x, y: pos.y }));
            }

            WindowEvent::MouseWheel {
                device_id: _,
                delta,
                phase,
            } => {
                let delt = match delta {
                    MouseScrollDelta::LineDelta(x, y) => Delta::Line {
                        x: x as u32,
                        y: y as u32,
                    },
                    MouseScrollDelta::PixelDelta(size) => Delta::Pixel {
                        x: size.x as u32,
                        y: size.y as u32,
                    },
                };
                let phase = match phase {
                    TouchPhase::Started => Phase::Start,
                    TouchPhase::Moved => Phase::Move,
                    TouchPhase::Ended => Phase::End,
                    TouchPhase::Cancelled => Phase::Cancel,
                };
                self.input_pool(On::Window(Win::Scroll { delta: delt, phase }));
            }

            WindowEvent::CursorEntered { device_id: _ } => {
                self.input_pool(On::Window(Win::Cursor(Point::Enter)));
            }

            WindowEvent::CursorLeft { device_id: _ } => {
                self.input_pool(On::Window(Win::Cursor(Point::Left)));
            }

            WindowEvent::CursorMoved {
                device_id: _,
                position,
            } => {
                self.m_pos.x = position.x as u32;
                self.m_pos.y = position.y as u32;
            }

            WindowEvent::RedrawRequested => {
                self.input_pool(On::Mouse(When::Move {
                    x: self.m_pos.x,
                    y: self.m_pos.y,
                }));
                self.draw(event_loop);
                self.clean_pipe();
            }

            _ => (),
        }
    }
}

fn draw_rect(frame: &mut [u8], x: u32, y: u32, w: u32, h: u32, color: [u8; 4], screen_width: u32) {
    let len = frame.len();
    const BYTE: u32 = 4;
    for row in y..(y + h) {
        for col in x..(x + w) {
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

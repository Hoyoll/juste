mod rancher;
use std::sync::Arc;

use pixels::{Pixels, SurfaceTexture};
use winit::{
    application::ApplicationHandler,
    dpi::{LogicalSize, Size},
    event::WindowEvent,
    event_loop::{ActiveEventLoop, EventLoop},
    window::{Window, WindowAttributes, WindowId},
};

#[derive(Default)]
struct App<'a> {
    pub window: Option<Arc<Window>>,
    pub pixels: Option<Pixels<'a>>,
}

impl<'a> ApplicationHandler for App<'a> {
    fn resumed(&mut self, event_loop: &ActiveEventLoop) {
        // Create window
        let attrs = WindowAttributes::default()
            .with_title("Winit + Pixels")
            .with_resizable(true)
            .with_inner_size(Size::Logical(LogicalSize::new(640.0, 480.0)));

        let window = Arc::new(event_loop.create_window(attrs).unwrap());

        let size = window.inner_size();
        let surface_texture = SurfaceTexture::new(size.width, size.height, window.clone());

        let pixels = Pixels::new(640, 480, surface_texture).expect("Failed to create Pixels");
        self.window = Some(window);
        self.pixels = Some(pixels);

        self.window.as_ref().unwrap().request_redraw();
    }

    fn window_event(&mut self, event_loop: &ActiveEventLoop, _id: WindowId, event: WindowEvent) {
        match event {
            WindowEvent::RedrawRequested => {
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

            _ => (),
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

#[tokio::main]
async fn main() {
    let event_loop = EventLoop::new().unwrap();
    let mut app = App::default();
    let _ = event_loop.run_app(&mut app);
}

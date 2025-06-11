// use winit::application::ApplicationHandler;
// use winit::error::EventLoopError;
// use winit::event::WindowEvent;
// use winit::event_loop::{ActiveEventLoop, EventLoop};
// use winit::window::{Window, WindowId};

// use super::element::Element;

// #[derive(Default)]
// pub struct App {
//     window: Option<Window>,
// }

// impl ApplicationHandler for App {
//     fn resumed(&mut self, event_loop: &ActiveEventLoop) {
//         self.window = Some(
//             event_loop
//                 .create_window(Window::default_attributes())
//                 .unwrap(),
//         );
//     }

//     fn window_event(&mut self, event_loop: &ActiveEventLoop, _id: WindowId, event: WindowEvent) {
//         match event {
//             WindowEvent::CloseRequested => {
//                 println!("The close button was pressed; stopping");
//                 event_loop.exit();
//             }
//             WindowEvent::RedrawRequested => {
//                 // Redraw the application.
//                 //
//                 // It's preferable for applications that do not render continuously to render in
//                 // this event rather than in AboutToWait, since rendering in here allows
//                 // the program to gracefully handle redraws requested by the OS.

//                 // Draw.

//                 // Queue a RedrawRequested event.
//                 //
//                 // You only need to call this if you've determined that you need to redraw in
//                 // applications which do not always need to. Applications that redraw continuously
//                 // can render here instead.
//                 self.window.as_ref().unwrap().request_redraw();
//             }
//             _ => (),
//         }
//     }
// }

// pub struct Wgpu<'a> {
//     ui_element: &'a Element,
//     event_loop: EventLoop<()>,
//     app: App,
// }

// impl<'a> Wgpu<'a> {
//     pub fn new(elements: &'a Element) -> Self {
//         Self {
//             ui_element: elements,
//             event_loop: EventLoop::new().unwrap(),
//             app: App::default(),
//         }
//     }

//     pub fn run(mut self) -> Result<(), EventLoopError> {
//         self.event_loop.run_app(&mut self.app)
//     }
// }

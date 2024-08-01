use tracing::error;
use winit::{
    application::ApplicationHandler,
    event::{ElementState, KeyEvent, WindowEvent},
    event_loop::{ActiveEventLoop, ControlFlow, EventLoop},
    keyboard::{KeyCode, PhysicalKey},
    window::{Window, WindowAttributes, WindowId},
};

mod error;

pub use error::Error;
pub use error::Result;

pub fn run() -> Result<()> {
    let event_loop = EventLoop::new()?;
    event_loop.set_control_flow(ControlFlow::Poll);
    let mut app = App::default();
    event_loop.run_app(&mut app).map_err(Into::into)
}

struct App {
    window: Option<Window>,
    attributes: WindowAttributes,
}

impl App {
    pub fn new() -> Self {
        let attributes = Window::default_attributes().with_title("Game window");
        Self {
            window: None,
            attributes,
        }
    }
}

impl Default for App {
    fn default() -> Self {
        Self::new()
    }
}

impl ApplicationHandler for App {
    fn resumed(&mut self, event_loop: &ActiveEventLoop) {
        let window = match event_loop.create_window(self.attributes.clone()) {
            Ok(window) => window,
            Err(err) => {
                error!("Error creating new window: {err}");
                return;
            }
        };
        self.window = Some(window);
    }

    fn window_event(&mut self, event_loop: &ActiveEventLoop, id: WindowId, event: WindowEvent) {
        if self.window.as_ref().map(Window::id) != Some(id) {
            return;
        }
        match event {
            WindowEvent::CloseRequested => event_loop.exit(),
            WindowEvent::KeyboardInput { event, .. } if escape_pressed(&event) => event_loop.exit(),
            _ => (),
        }
    }
}

fn escape_pressed(event: &KeyEvent) -> bool {
    event.state == ElementState::Pressed && event.physical_key == PhysicalKey::Code(KeyCode::Escape)
}


// callling renderer module
mod renderer;
mod screen;
mod pty;


use std::sync::Arc;
use winit::{
    application::ApplicationHandler,
    event::{ElementState, WindowEvent},
    event_loop::{ActiveEventLoop, EventLoop},
    keyboard::{Key, NamedKey},
    window::{Window, WindowAttributes},
};

struct App {
    window: Option<Arc<Window>>,
    renderer: Option<renderer::Renderer>,
}

impl ApplicationHandler for App {
    fn resumed(&mut self, event_loop: &ActiveEventLoop) {
        let window = Arc::new(event_loop.create_window(WindowAttributes::default().with_title("Terminologyyyya")).unwrap());
        
        let instance = wgpu::Instance::new(wgpu::InstanceDescriptor {
            backends: wgpu::Backends::PRIMARY,
            ..Default::default()
        });
        let renderer = pollster::block_on(renderer::Renderer::new(window.clone(), instance));

        self.renderer = Some(renderer);
        self.window = Some(window);

        self.window.as_ref().unwrap().request_redraw();
    }

    fn window_event(&mut self, event_loop: &ActiveEventLoop, _window_id: winit::window::WindowId, event: WindowEvent) {
        match event {
            WindowEvent::CloseRequested => event_loop.exit(),

            WindowEvent::RedrawRequested => {
                if let Some(renderer) = self.renderer.as_mut() {
                    renderer.render();
                }
            }

            WindowEvent::Resized(size) => {
                if let Some(renderer) = self.renderer.as_mut() {
                    renderer.resize(size.width, size.height);
                }
                if let Some(window) = self.window.as_ref() {
                    window.request_redraw();
                }
            }

            WindowEvent::KeyboardInput { event, .. } => {
                if event.state == ElementState::Pressed {
                    if let Some(renderer) = self.renderer.as_mut() {
                        match &event.logical_key {
                            Key::Character(text) => {
                                for ch in text.chars() {
                                    renderer.input_char(ch);
                                }
                            }
                            Key::Named(NamedKey::Space) => {
                                renderer.input_char(' ');
                            }
                            Key::Named(NamedKey::Backspace) => {
                                renderer.backspace();
                            }
                            Key::Named(NamedKey::Enter) => {
                                renderer.new_line();
                            }
                            _ => {}
                        }
                    }
                    if let Some(window) = self.window.as_ref() {
                        window.request_redraw();
                    }
                }
            }

            _ => (),
        }
    }

    fn about_to_wait(&mut self, _event_loop: &ActiveEventLoop) {
        if let Some(window) = self.window.as_ref() {
            window.request_redraw();
        }
    }
}

fn main() {
    let event_loop = EventLoop::new().unwrap();

    let mut app = App {
        window: None,
        renderer: None,
    };

    event_loop.run_app(&mut app).unwrap();
}


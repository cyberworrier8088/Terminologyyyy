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
        let window = Arc::new(
            event_loop
                .create_window(WindowAttributes::default().with_title("Terminologyyyy Terminal"))
                .unwrap(),
        );

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
                        let mut handled = false;

                        match &event.logical_key {
                            Key::Named(NamedKey::Backspace) => {
                                renderer.write_pty("\x08");
                                handled = true;
                            }
                            Key::Named(NamedKey::Enter) => {
                                renderer.write_pty("\r");
                                handled = true;
                            }
                            Key::Named(NamedKey::Tab) => {
                                renderer.write_pty("\t");
                                handled = true;
                            }
                            Key::Named(NamedKey::Escape) => {
                                renderer.write_pty("\x1b");
                                handled = true;
                            }
                            Key::Named(NamedKey::ArrowUp) => {
                                renderer.write_pty("\x1b[A");
                                handled = true;
                            }
                            Key::Named(NamedKey::ArrowDown) => {
                                renderer.write_pty("\x1b[B");
                                handled = true;
                            }
                            Key::Named(NamedKey::ArrowRight) => {
                                renderer.write_pty("\x1b[C");
                                handled = true;
                            }
                            Key::Named(NamedKey::ArrowLeft) => {
                                renderer.write_pty("\x1b[D");
                                handled = true;
                            }
                            Key::Named(NamedKey::Home) => {
                                renderer.write_pty("\x1b[H");
                                handled = true;
                            }
                            Key::Named(NamedKey::End) => {
                                renderer.write_pty("\x1b[F");
                                handled = true;
                            }
                            Key::Named(NamedKey::Space) => {
                                renderer.write_pty(" ");
                                handled = true;
                            }
                            _ => {}
                        }

                        if !handled {
                            if let Some(text) = &event.text {
                                renderer.write_pty(text);
                            } else if let Key::Character(ch_str) = &event.logical_key {
                                renderer.write_pty(ch_str);
                            }
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

use std::sync::Arc;
use winit::window::Window;

use glyphon::*;

use crate::screen::Screen;
use crate::pty::Pty;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
enum AnsiState {
    Ground,
    Escape,
    Csi,
    Osc,
}

pub struct Renderer {
    window: Arc<Window>,
    instance: wgpu::Instance,
    surface: wgpu::Surface<'static>,
    device: wgpu::Device,
    queue: wgpu::Queue,
    config: wgpu::SurfaceConfiguration,
    font_system: FontSystem,
    swash_cache: SwashCache,
    cache: Cache,

    viewport: Viewport,
    atlas: TextAtlas,

    text_renderer: TextRenderer,

    buffer: Buffer,

    screen: Screen,

    pty: Pty,

    ansi_state: AnsiState,

    csi_buffer: String,
}

impl Renderer {
    pub async fn new(window: Arc<Window>, instance: wgpu::Instance) -> Self {
        println!("Initializing Renderer...");

        let surface = instance.create_surface(window.clone()).unwrap();

        let adapter = instance
            .request_adapter(&wgpu::RequestAdapterOptions {
                compatible_surface: Some(&surface),
                power_preference: wgpu::PowerPreference::HighPerformance,
                force_fallback_adapter: false,
            })
            .await
            .unwrap();

        println!("GPU Found: {:?}", adapter.get_info().name);

        let (device, queue) = adapter.request_device(&wgpu::DeviceDescriptor::default(), None).await.unwrap();

        let size = window.inner_size();
        let config = surface.get_default_config(&adapter, size.width, size.height).unwrap();
        surface.configure(&device, &config);

        let mut font_system = FontSystem::new();
        let swash_cache = SwashCache::new();
        let cache = Cache::new(&device);
        let mut viewport = Viewport::new(&device, &cache);

        viewport.update(
            &queue,
            Resolution {
                width: size.width,
                height: size.height,
            },
        );

        let mut atlas = TextAtlas::new(
            &device,
            &queue,
            &cache,
            config.format,
        );

        let text_renderer = TextRenderer::new(
            &mut atlas,
            &device,
            wgpu::MultisampleState::default(),
            None,
        );

        let mut buffer = Buffer::new(
            &mut font_system,
            Metrics::new(18.0, 24.0),
        );

        buffer.set_size(
            &mut font_system,
            Some(size.width as f32),
            Some(size.height as f32),
        );

        let screen = Screen::new();
        let pty = Pty::new();

        buffer.set_text(
            &mut font_system,
            &screen.lines.join("\n"),
            Attrs::new(),
            Shaping::Advanced,
        );

        Self {
            window,
            instance,
            surface,
            device,
            queue,
            config,
            font_system,
            swash_cache,
            cache,
            viewport,
            atlas,
            text_renderer,
            buffer,
            screen,
            pty,
            ansi_state: AnsiState::Ground,
            csi_buffer: String::new(),
        }
    }

    fn execute_csi(&mut self, command: char) {
        let clean_buffer = self.csi_buffer.trim_start_matches('?');
        let params: Vec<u32> = clean_buffer
            .split(';')
            .filter_map(|s| s.parse::<u32>().ok())
            .collect();

        let first_param = *params.first().unwrap_or(&1);

        match command {
            'A' => self.screen.cursor_up(first_param as usize),
            'B' => self.screen.cursor_down(first_param as usize),
            'C' => self.screen.cursor_right(first_param as usize),
            'D' => self.screen.cursor_left(first_param as usize),
            'K' => {
                let mode = *params.first().unwrap_or(&0);
                self.screen.erase_in_line(mode);
            }
            'J' => {
                let mode = *params.first().unwrap_or(&0);
                if mode == 2 || mode == 3 {
                    self.screen.clear_screen();
                }
            }
            'H' | 'f' => {
                let row = (*params.first().unwrap_or(&1) as usize).saturating_sub(1);
                let col = (*params.get(1).unwrap_or(&1) as usize).saturating_sub(1);
                self.screen.cursor_y = row;
                self.screen.cursor_x = col;
                self.screen.ensure_cursor_valid();
            }
            'n' => {
                if self.csi_buffer == "6" {
                    self.pty.write("\x1b[1;1R");
                }
            }
            _ => {}
        }
    }

    fn refresh_buffer(&mut self) {
        let text = self.screen.lines.join("\n");
        self.buffer.set_text(
            &mut self.font_system,
            &text,
            Attrs::new(),
            Shaping::Advanced,
        );
    }

    pub fn render(&mut self) {
        let output = self.pty.read_output();

        if !output.is_empty() {
            for ch in output.chars() {
                match self.ansi_state {
                    AnsiState::Ground => match ch {
                        '\x1b' => {
                            self.ansi_state = AnsiState::Escape;
                            self.csi_buffer.clear();
                        }
                        '\n' => {
                            self.screen.new_line();
                        }
                        '\r' => {
                            self.screen.carriage_return();
                        }
                        '\x08' => {
                            self.screen.cursor_left(1);
                        }
                        '\x07' => {}
                        '\t' => {
                            let next_tab = (self.screen.cursor_x / 8 + 1) * 8;
                            while self.screen.cursor_x < next_tab {
                                self.screen.push_char(' ');
                            }
                        }
                        _ => {
                            if !ch.is_control() {
                                self.screen.push_char(ch);
                            }
                        }
                    },
                    AnsiState::Escape => match ch {
                        '[' => {
                            self.ansi_state = AnsiState::Csi;
                            self.csi_buffer.clear();
                        }
                        ']' => {
                            self.ansi_state = AnsiState::Osc;
                        }
                        _ => {
                            self.ansi_state = AnsiState::Ground;
                        }
                    },
                    AnsiState::Csi => {
                        if ('\x40'..='~').contains(&ch) {
                            self.execute_csi(ch);
                            self.csi_buffer.clear();
                            self.ansi_state = AnsiState::Ground;
                        } else if ch == '\x1b' {
                            self.ansi_state = AnsiState::Escape;
                            self.csi_buffer.clear();
                        } else {
                            self.csi_buffer.push(ch);
                        }
                    }
                    AnsiState::Osc => {
                        if ch == '\x07' || ch == '\x1b' {
                            self.ansi_state = AnsiState::Ground;
                        }
                    }
                }
            }

            self.refresh_buffer();
        }

        let frame = match self.surface.get_current_texture() {
            Ok(frame) => frame,
            Err(_) => return,
        };

        let view = frame.texture.create_view(&wgpu::TextureViewDescriptor::default());

        let text_area = TextArea {
            buffer: &self.buffer,
            left: 10.0,
            top: 10.0,
            scale: 1.0,
            bounds: TextBounds {
                left: 0,
                top: 0,
                right: 60000,
                bottom: 60000,
            },
            default_color: Color::rgb(255, 255, 255),
            custom_glyphs: &[],
        };
        
        self.text_renderer.prepare(
            &self.device,
            &self.queue,
            &mut self.font_system,
            &mut self.atlas,
            &self.viewport,
            [text_area],
            &mut self.swash_cache,
        ).unwrap();
        
        let mut encoder = self.device.create_command_encoder(
            &wgpu::CommandEncoderDescriptor {
                label: Some("Render Encoder"),
            }
        );

        {
            let mut _pass = encoder.begin_render_pass(&wgpu::RenderPassDescriptor {
                label: Some("Clear Pass"),
                color_attachments: &[Some(wgpu::RenderPassColorAttachment {
                    view: &view,
                    resolve_target: None,
                    ops: wgpu::Operations {
                        load: wgpu::LoadOp::Clear(wgpu::Color {
                            r: 0.05,
                            g: 0.05,
                            b: 0.08,
                            a: 1.0,
                        }),
                        store: wgpu::StoreOp::Store,
                    },
                })],
                depth_stencil_attachment: None,
                occlusion_query_set: None,
                timestamp_writes: None,
            });

            self.text_renderer.render(
                &self.atlas,
                &self.viewport,
                &mut _pass,
            ).unwrap();
        }

        self.queue.submit(std::iter::once(encoder.finish()));
        frame.present();
    }

    pub fn resize(&mut self, width: u32, height: u32) {
        if width == 0 || height == 0 {
            return;
        }
        
        self.config.width = width;
        self.config.height = height;
        self.surface.configure(&self.device, &self.config);
        
        self.viewport.update(
            &self.queue,
            Resolution { width, height },
        );

        self.buffer.set_size(
            &mut self.font_system,
            Some(width as f32),
            Some(height as f32),
        );
    }

    pub fn write_pty(&mut self, text: &str) {
        self.pty.write(text);
    }
}

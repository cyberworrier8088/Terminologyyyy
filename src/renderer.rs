use std::sync::Arc;
use winit::window::Window;

// this for font
use glyphon::*;



use crate::screen::Screen;


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
}

impl Renderer {
    pub async fn new(window: Arc<Window>, instance: wgpu::Instance) -> Self {
        println!("Initializingggg Renderer");

        println!("init surface...");
        let surface = instance.create_surface(window.clone()).unwrap();

        println!("Finding GPU...");
        let adapter = instance
            .request_adapter(&wgpu::RequestAdapterOptions {
                compatible_surface: Some(&surface),
                power_preference: wgpu::PowerPreference::HighPerformance,
                force_fallback_adapter: false,
            })
            .await
            .unwrap();

        println!("GPU Found! Adapter: {:?}", adapter.get_info());

        println!("Creating Device...");
        let (device, queue) = adapter.request_device(&wgpu::DeviceDescriptor::default(), None).await.unwrap();

        println!("Device Ready!");
        let size = window.inner_size();
        println!("Window size: {:?}", size);

        println!("Getting default config...");
        let config = surface.get_default_config(&adapter, size.width, size.height).unwrap();

        println!("Configuring surface...");
        surface.configure(&device, &config);

        println!("Renderer  successcreatedfully!");

        let mut font_system = FontSystem::new();

        let swash_cache = SwashCache::new();

        let cache = Cache::new(&device);

        let mut viewport = Viewport::new(&device, &cache);


        viewport.update(
            &queue,
            Resolution{
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

        let mut screen = Screen::new();

        screen.push_line("Welcome to Terminologyyy");
        screen.push_line(" ");
        screen.push_line("This is Rust Terminal");


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
        }
    }

    pub fn render(&mut self) {
        let frame = self.surface.get_current_texture().unwrap();

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
        
        self.text_renderer.prepare(&self.device,&self.queue,&mut self.font_system,&mut self.atlas,&self.viewport,[text_area],&mut self.swash_cache,).unwrap();
        
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
                            r: 0.1,
                            g: 0.2,
                            b: 0.3,
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
        
        println!("Renderer resized to: {}x{}", width, height);
    }

    pub fn input_char(&mut self, ch: char) {
        self.screen.push_char(ch);


        self.buffer.set_text(
            &mut self.font_system,
            &self.screen.lines.join("\n"),
            Attrs::new(),
            Shaping::Advanced,
        );
    }

    pub fn backspace(&mut self) {
        self.screen.backspace();

        self.buffer.set_text(
            &mut self.font_system,
            &self.screen.lines.join("\n"),
            Attrs::new(),
            Shaping::Advanced,
        );
    }
}


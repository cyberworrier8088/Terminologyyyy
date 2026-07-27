use std::sync::Arc;
use winit::window::Window;


pub struct Renderer {
    window: Arc<Window>,
    instance: wgpu::Instance,
    surface: wgpu::Surface<'static>,
    device: wgpu::Device,
    queue: wgpu::Queue,
    config: wgpu::SurfaceConfiguration,
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
        let (device, queue) = adapter.request_device(&wgpu::DeviceDescriptor::default()).await.unwrap();

        println!("Device Ready!");
        let size = window.inner_size();
        println!("Window size: {:?}", size);

        println!("Getting default config...");
        let config = surface.get_default_config(&adapter, size.width, size.height).unwrap();

        println!("Configuring surface...");
        surface.configure(&device, &config);

        println!("Renderer created successfully!");

        Self {
            window,
            instance,
            surface,
            device,
            queue,
            config,
        }
    }

    pub fn render(&mut self) {
        let frame = self.surface.get_current_texture().unwrap();

        let view = frame.texture.create_view(&wgpu::TextureViewDescriptor::default());

        let mut encoder = self.device.create_command_encoder(
            &wgpu::CommandEncoderDescriptor {
                label: Some("Render Encoder"),
            }
        );

        {
            let _pass = encoder.begin_render_pass(&wgpu::RenderPassDescriptor {
                label: Some("Clear Pass"),
                color_attachments: &[Some(wgpu::RenderPassColorAttachment {
                    view: &view,
                    depth_slice: None,
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
}


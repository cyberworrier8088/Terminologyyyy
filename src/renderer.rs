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
        println!("Rendering frame!")
    }
}
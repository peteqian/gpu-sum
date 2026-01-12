use wgpu::util::DeviceExt;
use wgpu::{Adapter, Instance, PipelineCompilationOptions, ShaderModule, ShaderModuleDescriptor};

fn main() {
    pollster::block_on(run());
}

async fn run() {
    println!("Hello, world!");

    let device: Instance = wgpu::Instance::default(); // get device

    let adapter: Adapter = device
        .request_adapter(&wgpu::RequestAdapterOptions::default())
        .await
        .expect("Failed to find a GPU"); // get adapter - just need to read don't give ownership

    println!("Using: {}", adapter.get_info().name);

    let (device, queue) = adapter
        .request_device(&wgpu::DeviceDescriptor::default())
        .await
        .expect("Failed to create device");

    println!("Got device and a queue");

    let shader: ShaderModule = device.create_shader_module(ShaderModuleDescriptor {
        label: Some("test sum shader"),
        source: wgpu::ShaderSource::Wgsl(include_str!("shader.wgsl").into()),
    });

    let pipeline = device.create_compute_pipeline(&wgpu::ComputePipelineDescriptor {
        label: Some("sum pipeline"),
        layout: None,
        module: &shader,
        entry_point: Some("main"),
        compilation_options: PipelineCompilationOptions::default(),
        cache: None,
    });

    println!("Pipeline Created");

    let input_data: Vec<f32> = vec![1.0, 2.0, 3.0, 4.0, 5.0, 6.0, 7.0, 8.0]; // 8 floats
    let size = (input_data.len() * std::mem::size_of::<f32>()) as u64; // 4 bytes \times 8
    // floats = 32 bytes

    let input_buffer = device.create_buffer_init(&wgpu::util::BufferInitDescriptor {
        label: Some("input buffer"),
        contents: bytemuck::cast_slice(&input_data), // converts Vec<f32> into raw bytes that GPU
        // can read
        usage: wgpu::BufferUsages::STORAGE, // buffer compute shader storage
    });

    let output_buffer = device.create_buffer(&wgpu::BufferDescriptor {
        label: Some("output buffer"),
        size,
        usage: wgpu::BufferUsages::STORAGE | wgpu::BufferUsages::COPY_SRC,
        mapped_at_creation: false,
    });

    println!("Buffers created!");
}

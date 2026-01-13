use wgpu::util::DeviceExt;
use wgpu::{Adapter, Instance, PipelineCompilationOptions, ShaderModule, ShaderModuleDescriptor};

fn main() {
    pollster::block_on(run());
}

async fn run() {
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

    // copy the buffer to gpu
    let input_buffer = device.create_buffer_init(&wgpu::util::BufferInitDescriptor {
        label: Some("input buffer"),
        contents: bytemuck::cast_slice(&input_data), // converts Vec<f32> into raw bytes that GPU
        // can read
        usage: wgpu::BufferUsages::STORAGE, // buffer compute shader storage
    });

    // gpu bufffer
    let output_buffer = device.create_buffer(&wgpu::BufferDescriptor {
        label: Some("output buffer"),
        size,
        usage: wgpu::BufferUsages::STORAGE | wgpu::BufferUsages::COPY_SRC,
        mapped_at_creation: false,
    });

    println!("Buffers created!");

    // Connect shaders to buffer
    let bind_group_layout = pipeline.get_bind_group_layout(0);
    let bind_group = device.create_bind_group(&wgpu::BindGroupDescriptor {
        label: Some("bind group"),
        layout: &bind_group_layout,
        entries: &[
            wgpu::BindGroupEntry {
                binding: 0,
                resource: input_buffer.as_entire_binding(),
            },
            wgpu::BindGroupEntry {
                binding: 1,
                resource: output_buffer.as_entire_binding(),
            },
        ],
    });

    println!("Bind group created!");

    // Create command encoder
    let mut encoder = device.create_command_encoder(&wgpu::CommandEncoderDescriptor {
        label: Some("encoder"),
    });

    // Record commands
    {
        let mut compute_pass = encoder.begin_compute_pass(&wgpu::ComputePassDescriptor {
            label: Some("compute pass"),
            timestamp_writes: None,
        });
        compute_pass.set_pipeline(&pipeline);
        compute_pass.set_bind_group(0, &bind_group, &[]);
        compute_pass.dispatch_workgroups(1, 1, 1);
    }

    // Submit to GPU
    queue.submit(Some(encoder.finish()));

    println!("GPU work submitted!");

    // Create staging buffer to read results back
    let staging_buffer = device.create_buffer(&wgpu::BufferDescriptor {
        label: Some("staging buffer"),
        size,
        usage: wgpu::BufferUsages::MAP_READ | wgpu::BufferUsages::COPY_DST,
        mapped_at_creation: false,
    });

    // Copy output buffer to staging buffer
    let mut encoder = device.create_command_encoder(&wgpu::CommandEncoderDescriptor {
        label: Some("copy encoder"),
    });
    encoder.copy_buffer_to_buffer(&output_buffer, 0, &staging_buffer, 0, size);
    queue.submit(Some(encoder.finish()));

    // Map the staging buffer so CPU can read it
    let slice = staging_buffer.slice(..);
    slice.map_async(wgpu::MapMode::Read, |_| {});

    device
        .poll(wgpu::PollType::wait_indefinitely())
        .expect("Failed to poll device");

    // Read the data
    let data = slice.get_mapped_range();
    let result: Vec<f32> = bytemuck::cast_slice(&data).to_vec();
    drop(data);
    staging_buffer.unmap();

    println!("Input:  {:?}", input_data);
    println!("Output: {:?}", result);
}

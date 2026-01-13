# GPU Sum (Learning WebGPU with Rust)

A tiny project I made to learn how to use the GPU for calculations with Rust.

## What It Does

Takes a list of numbers and doubles each one using the GPU instead of the CPU:

- **Input**: `[1, 2, 3, 4, 5, 6, 7, 8]`
- **Output**: `[2, 4, 6, 8, 10, 12, 14, 16]`

The actual math is super simple (just multiply by 2), but the interesting part is making the GPU do it.

- **GPU Initialization**: Setting up the GPU device and preparing it for work
- **Compute Shaders**: Writing code (WGSL) that runs on the GPU in parallel
- **Data Transfer**: Moving data from CPU to GPU and back
- **Workgroups**: Having multiple threads work on different parts of the data at the same time

## How to Run

```bash
cargo run --release
```

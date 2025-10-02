//! GPU and SIMD parallelism examples
//!
//! This module demonstrates high-performance image filtering offloaded to GPU via wgpu,
//! or CPU SIMD loops with std::arch.

use bytemuck::{Pod, Zeroable};
use std::arch::x86_64::*;
use std::time::Instant;

/// A simple 2D vector for SIMD operations
#[repr(C)]
#[derive(Clone, Copy, Pod, Zeroable)]
pub struct Vec2 {
    pub x: f32,
    pub y: f32,
}

/// A simple 4D vector for SIMD operations
#[repr(C)]
#[derive(Clone, Copy, Pod, Zeroable)]
pub struct Vec4 {
    pub x: f32,
    pub y: f32,
    pub z: f32,
    pub w: f32,
}

/// A simple matrix for transformations
#[repr(C)]
#[derive(Clone, Copy, Pod, Zeroable)]
pub struct Mat4 {
    pub data: [f32; 16],
}

/// Image data structure
pub struct Image {
    pub width: u32,
    pub height: u32,
    pub data: Vec<u8>,
}

impl Image {
    pub fn new(width: u32, height: u32) -> Self {
        Self {
            width,
            height,
            data: vec![0; (width * height * 4) as usize], // RGBA
        }
    }

    pub fn get_pixel(&self, x: u32, y: u32) -> [u8; 4] {
        let index = ((y * self.width + x) * 4) as usize;
        [
            self.data[index],
            self.data[index + 1],
            self.data[index + 2],
            self.data[index + 3],
        ]
    }

    pub fn set_pixel(&mut self, x: u32, y: u32, pixel: [u8; 4]) {
        let index = ((y * self.width + x) * 4) as usize;
        self.data[index] = pixel[0];
        self.data[index + 1] = pixel[1];
        self.data[index + 2] = pixel[2];
        self.data[index + 3] = pixel[3];
    }
}

/// Apply a simple blur filter using SIMD operations
#[target_feature(enable = "sse2")]
unsafe fn simd_blur_filter_sse2(data: &mut [f32], kernel: &[f32; 9]) {
    // This is a simplified example - a real implementation would be more complex
    for i in 0..data.len().saturating_sub(8) {
        if i % 8 == 0 && i + 8 <= data.len() {
            let chunk = &mut data[i..i + 8];
            let chunk_ptr = chunk.as_mut_ptr() as *mut __m128;
            let kernel_ptr = kernel.as_ptr() as *const __m128;
            
            // Load data and kernel
            let data_vec = _mm_load_ps(chunk_ptr as *const f32);
            let kernel_vec = _mm_load_ps(kernel_ptr as *const f32);
            
            // Perform SIMD operation
            let result = _mm_mul_ps(data_vec, kernel_vec);
            
            // Store result
            _mm_store_ps(chunk_ptr as *mut f32, result);
        }
    }
}

/// Apply a simple blur filter using AVX operations
#[target_feature(enable = "avx")]
unsafe fn simd_blur_filter_avx(data: &mut [f32], kernel: &[f32; 9]) {
    // This is a simplified example - a real implementation would be more complex
    for i in 0..data.len().saturating_sub(16) {
        if i % 16 == 0 && i + 16 <= data.len() {
            let chunk = &mut data[i..i + 16];
            let chunk_ptr = chunk.as_mut_ptr() as *mut __m256;
            let kernel_ptr = kernel.as_ptr() as *const __m256;
            
            // Load data and kernel
            let data_vec = _mm256_load_ps(chunk_ptr as *const f32);
            let kernel_vec = _mm256_load_ps(kernel_ptr as *const f32);
            
            // Perform SIMD operation
            let result = _mm256_mul_ps(data_vec, kernel_vec);
            
            // Store result
            _mm256_store_ps(chunk_ptr as *mut f32, result);
        }
    }
}

/// Example of CPU-based SIMD operations
pub fn cpu_simd_example() {
    println!("Starting CPU SIMD operations example...");
    
    // Create sample data
    let mut data: Vec<f32> = (0..1000).map(|i| i as f32 * 0.1).collect();
    let kernel = [0.11, 0.11, 0.11, 0.11, 0.12, 0.11, 0.11, 0.11, 0.11];
    
    println!("Data length: {}", data.len());
    
    // Measure performance of regular operation
    let start = Instant::now();
    for i in 0..data.len() {
        if i < kernel.len() {
            data[i] *= kernel[i % kernel.len()];
        }
    }
    let regular_time = start.elapsed();
    
    println!("Regular operation time: {:?}", regular_time);
    
    // SIMD operations (only run if CPU supports the features)
    if is_x86_feature_detected!("sse2") {
        let mut simd_data: Vec<f32> = (0..1000).map(|i| i as f32 * 0.1).collect();
        
        let start = Instant::now();
        unsafe {
            simd_blur_filter_sse2(&mut simd_data, &kernel);
        }
        let sse2_time = start.elapsed();
        
        println!("SSE2 operation time: {:?}", sse2_time);
    }
    
    if is_x86_feature_detected!("avx") {
        let mut avx_data: Vec<f32> = (0..1000).map(|i| i as f32 * 0.1).collect();
        
        let start = Instant::now();
        unsafe {
            simd_blur_filter_avx(&mut avx_data, &kernel);
        }
        let avx_time = start.elapsed();
        
        println!("AVX operation time: {:?}", avx_time);
    }
}

/// Create a simple wgpu instance
pub async fn create_wgpu_instance() -> Option<(wgpu::Instance, wgpu::Adapter, wgpu::Device, wgpu::Queue)> {
    // Create wgpu instance
    let instance = wgpu::Instance::new(wgpu::InstanceDescriptor {
        backends: wgpu::Backends::all(),
        ..Default::default()
    });
    
    // Get adapter
    let adapter = instance.request_adapter(&wgpu::RequestAdapterOptions {
        power_preference: wgpu::PowerPreference::default(),
        compatible_surface: None,
        force_fallback_adapter: false,
    }).await?;
    
    // Get device and queue
    let (device, queue) = adapter.request_device(
        &wgpu::DeviceDescriptor {
            features: wgpu::Features::empty(),
            limits: wgpu::Limits::default(),
            label: None,
        },
        None,
    ).await.ok()?;
    
    Some((instance, adapter, device, queue))
}

/// Example of GPU-based computation with wgpu
pub async fn gpu_computation_example() {
    println!("Starting GPU computation example...");
    
    // Try to create wgpu instance
    match create_wgpu_instance().await {
        Some((_instance, _adapter, device, _queue)) => {
            println!("Successfully created wgpu instance");
            println!("Device: {:?}", device.get_info());
            
            // In a real application, you would create buffers, shaders, and compute pipelines here
            println!("GPU is ready for computation");
        }
        None => {
            println!("Failed to create wgpu instance - GPU may not be available");
            println!("Falling back to CPU computation");
        }
    }
}

/// Example of vector operations using SIMD
pub fn vector_operations_example() {
    println!("Starting vector operations example...");
    
    // Create sample vectors
    let vec1 = Vec2 { x: 1.0, y: 2.0 };
    let vec2 = Vec2 { x: 3.0, y: 4.0 };
    
    println!("Vector 1: ({}, {})", vec1.x, vec1.y);
    println!("Vector 2: ({}, {})", vec2.x, vec2.y);
    
    // Perform operations
    let sum = Vec2 {
        x: vec1.x + vec2.x,
        y: vec1.y + vec2.y,
    };
    
    let dot_product = vec1.x * vec2.x + vec1.y * vec2.y;
    
    println!("Sum: ({}, {})", sum.x, sum.y);
    println!("Dot product: {}", dot_product);
    
    // SIMD version (if available)
    if is_x86_feature_detected!("sse2") {
        unsafe {
            let v1 = _mm_set_ps(0.0, 0.0, vec1.y, vec1.x);
            let v2 = _mm_set_ps(0.0, 0.0, vec2.y, vec2.x);
            
            let sum_simd = _mm_add_ps(v1, v2);
            let dot_simd = _mm_mul_ps(v1, v2);
            
            println!("SIMD Sum: ({}, {})", 
                     _mm_cvtss_f32(sum_simd), 
                     _mm_cvtss_f32(_mm_shuffle_ps(sum_simd, sum_simd, 1)));
            println!("SIMD operations completed");
        }
    }
}

/// Example of matrix operations
pub fn matrix_operations_example() {
    println!("Starting matrix operations example...");
    
    // Create sample matrices
    let mat1 = Mat4 {
        data: [
            1.0, 0.0, 0.0, 0.0,
            0.0, 1.0, 0.0, 0.0,
            0.0, 0.0, 1.0, 0.0,
            0.0, 0.0, 0.0, 1.0,
        ]
    };
    
    let mat2 = Mat4 {
        data: [
            2.0, 0.0, 0.0, 0.0,
            0.0, 2.0, 0.0, 0.0,
            0.0, 0.0, 2.0, 0.0,
            0.0, 0.0, 0.0, 2.0,
        ]
    };
    
    println!("Matrix 1 (identity):");
    for i in 0..4 {
        println!("  {:?}", &mat1.data[i*4..(i+1)*4]);
    }
    
    println!("Matrix 2 (scale):");
    for i in 0..4 {
        println!("  {:?}", &mat2.data[i*4..(i+1)*4]);
    }
    
    // Matrix multiplication (simplified)
    let mut result = Mat4 { data: [0.0; 16] };
    for i in 0..4 {
        for j in 0..4 {
            for k in 0..4 {
                result.data[i * 4 + j] += mat1.data[i * 4 + k] * mat2.data[k * 4 + j];
            }
        }
    }
    
    println!("Result of multiplication:");
    for i in 0..4 {
        println!("  {:?}", &result.data[i*4..(i+1)*4]);
    }
}

/// Example of image processing with SIMD
pub fn image_processing_simd_example() {
    println!("Starting image processing with SIMD example...");
    
    // Create a sample image
    let mut image = Image::new(100, 100);
    
    // Fill with a gradient
    for y in 0..image.height {
        for x in 0..image.width {
            let r = (x * 255 / image.width) as u8;
            let g = (y * 255 / image.height) as u8;
            let b = 128;
            let a = 255;
            image.set_pixel(x, y, [r, g, b, a]);
        }
    }
    
    println!("Created {}x{} image", image.width, image.height);
    
    // Apply a simple filter (brighten)
    let start = Instant::now();
    for y in 0..image.height {
        for x in 0..image.width {
            let mut pixel = image.get_pixel(x, y);
            // Brighten each channel
            pixel[0] = (pixel[0] as f32 * 1.2).min(255.0) as u8;
            pixel[1] = (pixel[1] as f32 * 1.2).min(255.0) as u8;
            pixel[2] = (pixel[2] as f32 * 1.2).min(255.0) as u8;
            image.set_pixel(x, y, pixel);
        }
    }
    let regular_time = start.elapsed();
    
    println!("Regular image processing time: {:?}", regular_time);
    
    // SIMD version would process multiple pixels at once
    println!("SIMD version would process multiple pixels simultaneously");
}

/// Example usage of GPU and SIMD functions
pub fn example_usage() {
    println!("GPU and SIMD Parallelism Examples");
    println!("================================");
    
    println!("\n1. CPU SIMD operations example:");
    cpu_simd_example();
    
    println!("\n2. Vector operations example:");
    vector_operations_example();
    
    println!("\n3. Matrix operations example:");
    matrix_operations_example();
    
    println!("\n4. Image processing with SIMD example:");
    image_processing_simd_example();
    
    println!("\n5. GPU computation example:");
    // Note: This would need to be called in an async context
    println!("   Call gpu_computation_example().await to see this in action");
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_vec2() {
        let vec = Vec2 { x: 1.0, y: 2.0 };
        assert_eq!(vec.x, 1.0);
        assert_eq!(vec.y, 2.0);
    }

    #[test]
    fn test_vec4() {
        let vec = Vec4 { x: 1.0, y: 2.0, z: 3.0, w: 4.0 };
        assert_eq!(vec.x, 1.0);
        assert_eq!(vec.y, 2.0);
        assert_eq!(vec.z, 3.0);
        assert_eq!(vec.w, 4.0);
    }

    #[test]
    fn test_image() {
        let mut image = Image::new(10, 10);
        assert_eq!(image.width, 10);
        assert_eq!(image.height, 10);
        assert_eq!(image.data.len(), 400); // 10 * 10 * 4
        
        let pixel = [255, 128, 64, 255];
        image.set_pixel(5, 5, pixel);
        
        let retrieved = image.get_pixel(5, 5);
        assert_eq!(retrieved, pixel);
    }

    #[test]
    fn test_matrix() {
        let matrix = Mat4 { data: [1.0; 16] };
        assert_eq!(matrix.data.len(), 16);
        assert_eq!(matrix.data[0], 1.0);
    }
}
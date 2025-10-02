//! Parallel data processing examples using rayon
//!
//! This module demonstrates how to use rayon for parallel data processing,
//! such as image thumbnail generation or other CPU-intensive tasks.

use rayon::prelude::*;
use std::time::Instant;

/// Process a collection of items in parallel using rayon
///
/// This example shows how to use `par_iter()` to process data in parallel
/// across multiple CPU cores.
pub fn parallel_data_processing() {
    // Create a large vector of numbers to process
    let data: Vec<i32> = (1..=1_000_000).collect();
    
    // Process sequentially
    let start = Instant::now();
    let sequential_result: Vec<i32> = data.iter()
        .map(|x| x * x)
        .collect();
    let sequential_time = start.elapsed();
    
    // Process in parallel
    let start = Instant::now();
    let parallel_result: Vec<i32> = data.par_iter()
        .map(|x| x * x)
        .collect();
    let parallel_time = start.elapsed();
    
    println!("Sequential processing time: {:?}", sequential_time);
    println!("Parallel processing time: {:?}", parallel_time);
    println!("Speedup: {:.2}x", sequential_time.as_secs_f64() / parallel_time.as_secs_f64());
    
    // Verify results are the same
    assert_eq!(sequential_result, parallel_result);
    println!("Results verified: both approaches produce identical results");
}

/// Simulate image thumbnail generation using parallel processing
///
/// This example demonstrates how you might process multiple images
/// in parallel, similar to generating thumbnails.
pub fn parallel_image_processing() {
    // Simulate a collection of image processing tasks
    let image_tasks: Vec<String> = (1..=100)
        .map(|i| format!("image_{}.jpg", i))
        .collect();
    
    println!("Processing {} images...", image_tasks.len());
    
    let start = Instant::now();
    
    // Process images in parallel
    let processed_images: Vec<String> = image_tasks
        .par_iter()
        .map(|image_name| {
            // Simulate image processing work
            std::thread::sleep(std::time::Duration::from_millis(10));
            format!("thumbnail_{}", image_name)
        })
        .collect();
    
    let processing_time = start.elapsed();
    
    println!("Processed {} images in {:?}", processed_images.len(), processing_time);
    println!("First few results: {:?}", &processed_images[..5]);
}

/// Parallel reduction example
///
/// This demonstrates how to use parallel reduction to compute
/// aggregate values from a large dataset.
pub fn parallel_reduction() {
    let data: Vec<f64> = (1..=1_000_000)
        .map(|x| x as f64)
        .collect();
    
    // Sequential sum
    let seq_start = Instant::now();
    let seq_sum: f64 = data.iter().sum();
    let seq_time = seq_start.elapsed();
    
    // Parallel sum
    let par_start = Instant::now();
    let par_sum: f64 = data.par_iter().sum();
    let par_time = par_start.elapsed();
    
    println!("Sequential sum: {} (time: {:?})", seq_sum, seq_time);
    println!("Parallel sum: {} (time: {:?})", par_sum, par_time);
    
    // Parallel maximum
    let max_start = Instant::now();
    let max_value = data.par_iter().max_by(|a, b| a.partial_cmp(b).unwrap());
    let max_time = max_start.elapsed();
    
    println!("Parallel max: {:?} (time: {:?})", max_value, max_time);
}

/// Example usage of parallel processing functions
pub fn example_usage() {
    println!("Parallel Data Processing Examples");
    println!("================================");
    
    println!("\n1. Parallel data processing:");
    parallel_data_processing();
    
    println!("\n2. Parallel image processing simulation:");
    parallel_image_processing();
    
    println!("\n3. Parallel reduction:");
    parallel_reduction();
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_parallel_processing() {
        // Test that parallel processing produces correct results
        let data: Vec<i32> = (1..=1000).collect();
        
        let sequential_result: Vec<i32> = data.iter().map(|x| x * 2).collect();
        let parallel_result: Vec<i32> = data.par_iter().map(|x| x * 2).collect();
        
        assert_eq!(sequential_result, parallel_result);
    }

    #[test]
    fn test_parallel_reduction() {
        let data: Vec<i32> = (1..=1000).collect();
        
        let seq_sum: i32 = data.iter().sum();
        let par_sum: i32 = data.par_iter().sum();
        
        assert_eq!(seq_sum, par_sum);
    }
}
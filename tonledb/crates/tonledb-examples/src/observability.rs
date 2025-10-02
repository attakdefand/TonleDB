//! Benchmarking, tracing and observability examples
//!
//! This module demonstrates how to benchmark hotspots with criterion,
//! then add tracing spans exported via opentelemetry.

use criterion::{black_box, criterion_group, criterion_main, Bencher, Criterion};
use opentelemetry::trace::{Span, Tracer, TracerProvider};
use opentelemetry::{global, KeyValue};
use std::time::Duration;
use tracing::{info, span, Level};

/// A simple function to benchmark
fn fibonacci(n: u32) -> u32 {
    match n {
        0 => 1,
        1 => 1,
        _ => fibonacci(n - 1) + fibonacci(n - 2),
    }
}

/// A more efficient fibonacci implementation for benchmarking
fn fibonacci_optimized(n: u32) -> u32 {
    if n <= 1 {
        return 1;
    }
    
    let mut a = 1;
    let mut b = 1;
    
    for _ in 2..=n {
        let temp = a + b;
        a = b;
        b = temp;
    }
    
    b
}

/// Benchmark the recursive fibonacci function
fn bench_fibonacci(c: &mut Criterion) {
    c.bench_function("fibonacci_10", |b| b.iter(|| fibonacci(black_box(10))));
    c.bench_function("fibonacci_20", |b| b.iter(|| fibonacci(black_box(20))));
}

/// Benchmark the optimized fibonacci function
fn bench_fibonacci_optimized(c: &mut Criterion) {
    c.bench_function("fibonacci_optimized_10", |b| b.iter(|| fibonacci_optimized(black_box(10))));
    c.bench_function("fibonacci_optimized_20", |b| b.iter(|| fibonacci_optimized(black_box(20))));
    c.bench_function("fibonacci_optimized_30", |b| b.iter(|| fibonacci_optimized(black_box(30))));
}

/// Compare the two fibonacci implementations
fn bench_fibonacci_comparison(c: &mut Criterion) {
    let mut group = c.benchmark_group("fibonacci_comparison");
    
    group.bench_function("recursive_15", |b| b.iter(|| fibonacci(black_box(15))));
    group.bench_function("optimized_15", |b| b.iter(|| fibonacci_optimized(black_box(15))));
    
    group.finish();
}

/// Initialize OpenTelemetry tracing
pub fn init_tracing() -> anyhow::Result<()> {
    // Initialize tracing subscriber
    tracing_subscriber::fmt::init();
    
    // Initialize OpenTelemetry
    let provider = opentelemetry_stdout::SpanExporterBuilder::default()
        .with_writer(std::io::stdout())
        .build();
    
    let tracer_provider = opentelemetry::sdk::trace::TracerProvider::builder()
        .with_simple_exporter(provider)
        .build();
    
    let tracer = tracer_provider.tracer("tonledb-examples");
    
    // Set the global tracer provider
    global::set_tracer_provider(tracer_provider);
    
    Ok(())
}

/// A function that simulates some work with tracing
#[tracing::instrument]
pub fn simulate_work_with_tracing(id: u32, duration_ms: u64) -> u32 {
    info!("Starting work simulation for task {}", id);
    
    // Create a nested span
    let span = span!(Level::INFO, "work_simulation", task_id = id);
    let _enter = span.enter();
    
    // Simulate some work
    std::thread::sleep(Duration::from_millis(duration_ms));
    
    // Add an event
    tracing::event!(Level::INFO, "Work completed", task_id = id, duration = duration_ms);
    
    info!("Completed work simulation for task {}", id);
    id * 2
}

/// A function that demonstrates tracing with attributes
#[tracing::instrument(fields(task_type = "calculation"))]
pub fn calculation_task(input: f64) -> f64 {
    // Create a span for the calculation
    let span = span!(Level::INFO, "calculation", input = input);
    let _enter = span.enter();
    
    // Simulate a calculation
    let result = input * input + input * 2.0 + 1.0;
    
    // Add attributes to the span
    span.record("result", &result);
    
    result
}

/// A function that demonstrates error tracing
#[tracing::instrument]
pub fn error_prone_task(should_fail: bool) -> Result<String, &'static str> {
    if should_fail {
        tracing::error!("Task failed intentionally");
        Err("Intentional failure")
    } else {
        tracing::info!("Task completed successfully");
        Ok("Success".to_string())
    }
}

/// Example of benchmarking with criterion
pub fn benchmarking_example() {
    println!("Starting benchmarking example...");
    println!("To run benchmarks, use: cargo bench");
    println!("This will execute the benchmark functions and generate reports");
    
    // Show what the benchmark functions do
    println!("\nBenchmark functions:");
    println!("  - fibonacci_10: Benchmark recursive fibonacci(10)");
    println!("  - fibonacci_20: Benchmark recursive fibonacci(20)");
    println!("  - fibonacci_optimized_10: Benchmark optimized fibonacci(10)");
    println!("  - fibonacci_optimized_20: Benchmark optimized fibonacci(20)");
    println!("  - fibonacci_optimized_30: Benchmark optimized fibonacci(30)");
    println!("  - fibonacci_comparison: Compare recursive vs optimized implementations");
}

/// Example of tracing with OpenTelemetry
pub fn tracing_example() {
    println!("Starting tracing example...");
    
    // Initialize tracing
    if let Err(e) = init_tracing() {
        eprintln!("Failed to initialize tracing: {}", e);
        return;
    }
    
    // Create a root span
    let root_span = span!(Level::INFO, "application_root");
    let _enter = root_span.enter();
    
    // Simulate some traced work
    let result1 = simulate_work_with_tracing(1, 100);
    let result2 = simulate_work_with_tracing(2, 150);
    
    println!("Results: {}, {}", result1, result2);
    
    // Perform a calculation task
    let calc_result = calculation_task(5.0);
    println!("Calculation result: {}", calc_result);
    
    // Test error handling
    match error_prone_task(false) {
        Ok(result) => println!("Success result: {}", result),
        Err(e) => println!("Error: {}", e),
    }
    
    match error_prone_task(true) {
        Ok(result) => println!("Success result: {}", result),
        Err(e) => println!("Expected error: {}", e),
    }
    
    // Exit the root span
    drop(_enter);
    drop(root_span);
    
    // Shutdown the tracer provider
    global::shutdown_tracer_provider();
}

/// Example of performance profiling
pub fn profiling_example() {
    println!("Starting performance profiling example...");
    
    // Measure execution time of different approaches
    let start = std::time::Instant::now();
    let result1 = fibonacci(20);
    let duration1 = start.elapsed();
    
    let start = std::time::Instant::now();
    let result2 = fibonacci_optimized(20);
    let duration2 = start.elapsed();
    
    println!("Recursive fibonacci(20): {} (took {:?})", result1, duration1);
    println!("Optimized fibonacci(20): {} (took {:?})", result2, duration2);
    println!("Speedup: {:.2}x", duration1.as_nanos() as f64 / duration2.as_nanos() as f64);
}

/// Example of memory usage tracking
pub fn memory_usage_example() {
    println!("Starting memory usage tracking example...");
    
    // Track memory before allocation
    let mut data: Vec<u32> = Vec::new();
    
    // Allocate memory in chunks
    for i in 0..5 {
        let start = std::time::Instant::now();
        data.extend((0..100000).map(|x| x + i * 100000));
        let duration = start.elapsed();
        
        println!("Allocation {}: {} elements in {:?}", i, data.len(), duration);
    }
    
    // Clear memory
    data.clear();
    println!("Memory cleared, vector length: {}", data.len());
}

/// Example of concurrent tracing
pub async fn concurrent_tracing_example() {
    println!("Starting concurrent tracing example...");
    
    // Initialize tracing
    if let Err(e) = init_tracing() {
        eprintln!("Failed to initialize tracing: {}", e);
        return;
    }
    
    // Spawn multiple async tasks with tracing
    let mut handles = vec![];
    
    for i in 0..5 {
        let handle = tokio::spawn(async move {
            let span = span!(Level::INFO, "async_task", task_id = i);
            let _enter = span.enter();
            
            tracing::info!("Starting async task {}", i);
            tokio::time::sleep(Duration::from_millis(100 * (i + 1))).await;
            tracing::info!("Completed async task {}", i);
            
            i * 10
        });
        handles.push(handle);
    }
    
    // Wait for all tasks to complete
    let mut results = Vec::new();
    for handle in handles {
        if let Ok(result) = handle.await {
            results.push(result);
        }
    }
    
    println!("Task results: {:?}", results);
    
    // Shutdown the tracer provider
    global::shutdown_tracer_provider();
}

/// Example usage of benchmarking, tracing and observability functions
pub fn example_usage() {
    println!("Benchmarking, Tracing and Observability Examples");
    println!("=============================================");
    
    println!("\n1. Benchmarking example:");
    benchmarking_example();
    
    println!("\n2. Tracing example:");
    tracing_example();
    
    println!("\n3. Performance profiling example:");
    profiling_example();
    
    println!("\n4. Memory usage tracking example:");
    memory_usage_example();
    
    println!("\n5. Concurrent tracing example:");
    // Note: This would need to be called in an async context
    println!("   Call concurrent_tracing_example().await to see this in action");
}

// Configure criterion benchmark groups
criterion_group!(
    benches,
    bench_fibonacci,
    bench_fibonacci_optimized,
    bench_fibonacci_comparison
);

criterion_main!(benches);

#[cfg(test)]
mod tests {
    use super::*;
    use criterion::Criterion;

    #[test]
    fn test_fibonacci() {
        assert_eq!(fibonacci(0), 1);
        assert_eq!(fibonacci(1), 1);
        assert_eq!(fibonacci(5), 8);
    }

    #[test]
    fn test_fibonacci_optimized() {
        assert_eq!(fibonacci_optimized(0), 1);
        assert_eq!(fibonacci_optimized(1), 1);
        assert_eq!(fibonacci_optimized(5), 8);
        assert_eq!(fibonacci_optimized(10), 89);
    }

    #[test]
    fn test_benchmark_functions() {
        // Test that benchmark functions compile and run without crashing
        let mut c = Criterion::default();
        bench_fibonacci(&mut c);
        bench_fibonacci_optimized(&mut c);
        bench_fibonacci_comparison(&mut c);
    }

    #[test]
    fn test_simulate_work_with_tracing() {
        let result = simulate_work_with_tracing(1, 1);
        assert_eq!(result, 2);
    }

    #[test]
    fn test_calculation_task() {
        let result = calculation_task(2.0);
        assert_eq!(result, 9.0); // 2*2 + 2*2 + 1 = 9
    }

    #[test]
    fn test_error_prone_task() {
        let success = error_prone_task(false);
        let failure = error_prone_task(true);
        
        assert!(success.is_ok());
        assert!(failure.is_err());
    }
}
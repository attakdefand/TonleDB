//! Benchmark tests for observability examples

use criterion::{black_box, criterion_group, criterion_main, Criterion};
use tonledb_examples::observability::{fibonacci, fibonacci_optimized};

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

// Configure criterion benchmark groups
criterion_group!(
    benches,
    bench_fibonacci,
    bench_fibonacci_optimized,
    bench_fibonacci_comparison
);

criterion_main!(benches);
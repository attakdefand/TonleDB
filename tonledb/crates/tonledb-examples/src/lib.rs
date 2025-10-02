//! TonleDB Examples - A collection of Rust concurrency and parallelism examples
//!
//! This crate contains practical examples of various Rust concepts and tools
//! that can be used with the TonleDB database system.

/// Concurrency examples using Tokio for high-concurrency HTTP server
pub mod concurrency;

/// Parallel data processing examples using rayon
pub mod parallel;

/// Async/await examples with gRPC client implementation
pub mod async_examples;

/// Thread and shared memory examples
pub mod threads;

/// Bulkheads and thread pools examples
pub mod bulkheads;

/// Message-passing and actor examples
pub mod messaging;

/// Channel examples for pipelines
pub mod channels;

/// Event-driven and reactive examples
pub mod reactive;

/// Streams and back-pressure examples
pub mod streams;

/// Coroutines and generators examples
pub mod coroutines;

/// State machines and manual futures examples
pub mod state_machines;

/// Software transactional memory examples
pub mod stm;

/// Dataflow and pipeline examples
pub mod dataflow;

/// GPU and SIMD parallelism examples
pub mod gpu_simd;

/// Distributed systems examples
pub mod distributed;

/// Real-time and embedded examples
pub mod realtime;

/// Benchmarking, tracing and observability examples
pub mod observability;
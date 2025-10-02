//! Main entry point for TonleDB Examples
//!
//! This binary demonstrates various Rust concurrency and parallelism concepts
//! that can be used with the TonleDB database system.

use tonledb_examples::{concurrency, parallel, async_examples, threads, bulkheads, messaging, channels, reactive, streams, coroutines, state_machines, stm, dataflow, gpu_simd, distributed, realtime, observability};

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    println!("TonleDB Examples - Concurrency and Parallelism Demo");
    println!("==================================================");
    
    // Show concurrency example usage information
    println!("\n1. Concurrency Examples:");
    concurrency::example_usage();
    
    // Show parallel processing example usage information
    println!("\n2. Parallel Processing Examples:");
    parallel::example_usage();
    
    // Show async/await example usage information
    println!("\n3. Async/Await Examples:");
    async_examples::example_usage();
    
    // Show thread example usage information
    println!("\n4. Thread and Shared Memory Examples:");
    threads::example_usage();
    
    // Show bulkhead example usage information
    println!("\n5. Bulkhead and Thread Pool Examples:");
    bulkheads::example_usage();
    
    // Show messaging example usage information
    println!("\n6. Message-Passing and Actor Examples:");
    messaging::example_usage();
    
    // Show channel example usage information
    println!("\n7. Channel Examples:");
    channels::example_usage();
    
    // Show reactive example usage information
    println!("\n8. Event-Driven and Reactive Examples:");
    reactive::example_usage();
    
    // Show streams example usage information
    println!("\n9. Streams and Back-Pressure Examples:");
    streams::example_usage();
    
    // Show coroutines example usage information
    println!("\n10. Coroutines and Generators Examples:");
    coroutines::example_usage();
    
    // Show state machines example usage information
    println!("\n11. State Machines and Manual Futures Examples:");
    state_machines::example_usage();
    
    // Show STM example usage information
    println!("\n12. Software Transactional Memory Examples:");
    stm::example_usage();
    
    // Show dataflow example usage information
    println!("\n13. Dataflow and Pipeline Examples:");
    dataflow::example_usage();
    
    // Show GPU and SIMD example usage information
    println!("\n14. GPU and SIMD Parallelism Examples:");
    gpu_simd::example_usage();
    
    // Show distributed systems example usage information
    println!("\n15. Distributed Systems Examples:");
    distributed::example_usage();
    
    // Show real-time example usage information
    println!("\n16. Real-Time and Embedded Examples:");
    realtime::example_usage();
    
    // Show observability example usage information
    println!("\n17. Benchmarking, Tracing and Observability Examples:");
    observability::example_usage();
    
    println!("\nStarting high-concurrency HTTP server...");
    println!("Press Ctrl+C to stop the server");
    
    // Run the high-concurrency server
    concurrency::run_high_concurrency_server().await
}
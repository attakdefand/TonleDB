//! Coroutines and generators examples
//!
//! This module demonstrates how to create live-metrics producers with
//! async_stream::stream! { … yield metric; … } feeding a dashboard.

use async_stream::stream;
use futures::stream::Stream;
use std::time::Duration;
use tokio::time::{sleep, interval};

/// A metric representing system measurements
#[derive(Debug, Clone)]
pub struct Metric {
    pub name: String,
    pub value: f64,
    pub timestamp: std::time::SystemTime,
}

/// Create a stream of CPU metrics
pub fn cpu_metrics_stream() -> impl Stream<Item = Metric> {
    stream! {
        let mut interval = interval(Duration::from_secs(1));
        let mut counter = 0;
        
        loop {
            interval.tick().await;
            counter += 1;
            
            // Simulate CPU usage metric (0-100%)
            let cpu_usage = (counter % 100) as f64;
            
            let metric = Metric {
                name: "cpu_usage".to_string(),
                value: cpu_usage,
                timestamp: std::time::SystemTime::now(),
            };
            
            yield metric;
        }
    }
}

/// Create a stream of memory metrics
pub fn memory_metrics_stream() -> impl Stream<Item = Metric> {
    stream! {
        let mut interval = interval(Duration::from_secs(2));
        let mut counter = 0;
        
        loop {
            interval.tick().await;
            counter += 1;
            
            // Simulate memory usage metric (0-100%)
            let memory_usage = ((counter * 3) % 100) as f64;
            
            let metric = Metric {
                name: "memory_usage".to_string(),
                value: memory_usage,
                timestamp: std::time::SystemTime::now(),
            };
            
            yield metric;
        }
    }
}

/// Create a stream of network metrics
pub fn network_metrics_stream() -> impl Stream<Item = Metric> {
    stream! {
        let mut interval = interval(Duration::from_secs(3));
        let mut bytes_sent = 0u64;
        let mut bytes_received = 0u64;
        
        loop {
            interval.tick().await;
            
            // Simulate network traffic
            let sent = (rand::random::<u64>() % 1000) + 100;
            let received = (rand::random::<u64>() % 1500) + 50;
            
            bytes_sent += sent;
            bytes_received += received;
            
            // Yield bytes sent metric
            let sent_metric = Metric {
                name: "network_bytes_sent".to_string(),
                value: bytes_sent as f64,
                timestamp: std::time::SystemTime::now(),
            };
            yield sent_metric;
            
            // Yield bytes received metric
            let received_metric = Metric {
                name: "network_bytes_received".to_string(),
                value: bytes_received as f64,
                timestamp: std::time::SystemTime::now(),
            };
            yield received_metric;
        }
    }
}

/// Create a stream of database metrics
pub fn database_metrics_stream() -> impl Stream<Item = Metric> {
    stream! {
        let mut interval = interval(Duration::from_secs(5));
        let mut query_count = 0u64;
        let mut connection_count = 10u64;
        
        loop {
            interval.tick().await;
            
            // Simulate database activity
            let new_queries = rand::random::<u64>() % 50;
            query_count += new_queries;
            
            // Simulate connection changes
            if rand::random::<bool>() {
                connection_count = connection_count.saturating_add(1);
            } else {
                connection_count = connection_count.saturating_sub(1).max(1);
            }
            
            // Yield query count metric
            let queries_metric = Metric {
                name: "database_queries_total".to_string(),
                value: query_count as f64,
                timestamp: std::time::SystemTime::now(),
            };
            yield queries_metric;
            
            // Yield connection count metric
            let connections_metric = Metric {
                name: "database_connections".to_string(),
                value: connection_count as f64,
                timestamp: std::time::SystemTime::now(),
            };
            yield connections_metric;
        }
    }
}

/// Create a combined metrics stream
pub fn combined_metrics_stream() -> impl Stream<Item = Metric> {
    stream! {
        let mut interval = interval(Duration::from_millis(500));
        let mut counter = 0;
        
        loop {
            interval.tick().await;
            counter += 1;
            
            // Yield different types of metrics
            let metrics = [
                Metric {
                    name: "combined_counter".to_string(),
                    value: counter as f64,
                    timestamp: std::time::SystemTime::now(),
                },
                Metric {
                    name: "combined_random".to_string(),
                    value: rand::random::<f64>() * 100.0,
                    timestamp: std::time::SystemTime::now(),
                }
            ];
            
            for metric in &metrics {
                yield metric.clone();
            }
        }
    }
}

/// Example of consuming CPU metrics
pub async fn cpu_metrics_example() {
    println!("Starting CPU metrics example...");
    
    let mut stream = cpu_metrics_stream();
    
    // Collect first 5 metrics
    for _ in 0..5 {
        if let Some(metric) = stream.next().await {
            println!("CPU Metric: {} = {:.2}%", metric.name, metric.value);
        }
    }
}

/// Example of consuming memory metrics
pub async fn memory_metrics_example() {
    println!("Starting memory metrics example...");
    
    let mut stream = memory_metrics_stream();
    
    // Collect first 5 metrics
    for _ in 0..5 {
        if let Some(metric) = stream.next().await {
            println!("Memory Metric: {} = {:.2}%", metric.name, metric.value);
        }
    }
}

/// Example of consuming network metrics
pub async fn network_metrics_example() {
    println!("Starting network metrics example...");
    
    let mut stream = network_metrics_stream();
    
    // Collect first 6 metrics (2 metrics per iteration)
    for _ in 0..3 {
        if let Some(metric1) = stream.next().await {
            println!("Network Metric: {} = {:.0}", metric1.name, metric1.value);
        }
        if let Some(metric2) = stream.next().await {
            println!("Network Metric: {} = {:.0}", metric2.name, metric2.value);
        }
    }
}

/// Example of consuming database metrics
pub async fn database_metrics_example() {
    println!("Starting database metrics example...");
    
    let mut stream = database_metrics_stream();
    
    // Collect first 6 metrics (2 metrics per iteration)
    for _ in 0..3 {
        if let Some(metric1) = stream.next().await {
            println!("Database Metric: {} = {:.0}", metric1.name, metric1.value);
        }
        if let Some(metric2) = stream.next().await {
            println!("Database Metric: {} = {:.0}", metric2.name, metric2.value);
        }
    }
}

/// Example of consuming combined metrics
pub async fn combined_metrics_example() {
    println!("Starting combined metrics example...");
    
    let mut stream = combined_metrics_stream();
    
    // Collect first 10 metrics
    for _ in 0..10 {
        if let Some(metric) = stream.next().await {
            println!("Combined Metric: {} = {:.2}", metric.name, metric.value);
        }
    }
}

/// Example of filtering metrics stream
pub async fn filtered_metrics_example() {
    println!("Starting filtered metrics example...");
    
    let stream = combined_metrics_stream();
    
    // Filter metrics with value > 50
    let mut filtered = stream.filter(|metric| async { metric.value > 50.0 });
    
    // Collect first 5 filtered metrics
    for _ in 0..5 {
        if let Some(metric) = filtered.next().await {
            println!("Filtered Metric: {} = {:.2}", metric.name, metric.value);
        }
    }
}

/// Example of transforming metrics stream
pub async fn transformed_metrics_example() {
    println!("Starting transformed metrics example...");
    
    let stream = combined_metrics_stream();
    
    // Transform metrics by doubling their values
    let mut transformed = stream.map(|mut metric| {
        metric.value *= 2.0;
        metric.name = format!("doubled_{}", metric.name);
        metric
    });
    
    // Collect first 5 transformed metrics
    for _ in 0..5 {
        if let Some(metric) = transformed.next().await {
            println!("Transformed Metric: {} = {:.2}", metric.name, metric.value);
        }
    }
}

/// Example usage of coroutines and generators functions
pub fn example_usage() {
    println!("Coroutines and Generators Examples");
    println!("================================");
    
    println!("\n1. CPU metrics example:");
    // Note: This would need to be called in an async context
    println!("   Call cpu_metrics_example().await to see this in action");
    
    println!("\n2. Memory metrics example:");
    println!("   Call memory_metrics_example().await to see this in action");
    
    println!("\n3. Network metrics example:");
    println!("   Call network_metrics_example().await to see this in action");
    
    println!("\n4. Database metrics example:");
    println!("   Call database_metrics_example().await to see this in action");
    
    println!("\n5. Combined metrics example:");
    println!("   Call combined_metrics_example().await to see this in action");
    
    println!("\n6. Filtered metrics example:");
    println!("   Call filtered_metrics_example().await to see this in action");
    
    println!("\n7. Transformed metrics example:");
    println!("   Call transformed_metrics_example().await to see this in action");
}

#[cfg(test)]
mod tests {
    use super::*;
    use futures::StreamExt;

    #[tokio::test]
    async fn test_cpu_metrics_stream() {
        let mut stream = cpu_metrics_stream();
        
        // Get first metric
        let metric = stream.next().await.unwrap();
        assert_eq!(metric.name, "cpu_usage");
        assert!(metric.value >= 0.0 && metric.value <= 100.0);
    }

    #[tokio::test]
    async fn test_memory_metrics_stream() {
        let mut stream = memory_metrics_stream();
        
        // Get first metric
        let metric = stream.next().await.unwrap();
        assert_eq!(metric.name, "memory_usage");
        assert!(metric.value >= 0.0 && metric.value <= 100.0);
    }

    #[tokio::test]
    async fn test_combined_metrics_stream() {
        let mut stream = combined_metrics_stream();
        
        // Get first metric
        let metric = stream.next().await.unwrap();
        assert_eq!(metric.name, "combined_counter");
        assert!(metric.value >= 0.0);
    }

    #[tokio::test]
    async fn test_stream_filtering() {
        let stream = combined_metrics_stream();
        let mut filtered = stream.filter(|metric| async { metric.value > 0.0 });
        
        // Get first filtered metric
        let metric = filtered.next().await.unwrap();
        assert!(metric.value > 0.0);
    }
}
//! Streams and back-pressure examples
//!
//! This module demonstrates how to use futures::Stream to pull chunks
//! and apply back-pressure to avoid overload.

use futures::stream::{self, Stream, StreamExt};
use std::pin::Pin;
use std::task::{Context, Poll};
use std::time::Duration;
use tokio::time::{sleep, Sleep};

/// A simple stream that produces integers
pub struct CounterStream {
    current: i32,
    limit: i32,
}

impl CounterStream {
    pub fn new(limit: i32) -> Self {
        Self { current: 0, limit }
    }
}

impl Stream for CounterStream {
    type Item = i32;

    fn poll_next(mut self: Pin<&mut Self>, _cx: &mut Context<'_>) -> Poll<Option<Self::Item>> {
        if self.current < self.limit {
            self.current += 1;
            Poll::Ready(Some(self.current))
        } else {
            Poll::Ready(None)
        }
    }
}

/// A stream that simulates downloading chunks with back-pressure
pub struct DownloadStream {
    chunks: Vec<Vec<u8>>,
    current_index: usize,
    delay: Pin<Box<Sleep>>,
    delay_finished: bool,
}

impl DownloadStream {
    pub fn new(chunks: Vec<Vec<u8>>) -> Self {
        Self {
            chunks,
            current_index: 0,
            delay: Box::pin(sleep(Duration::from_millis(100))),
            delay_finished: false,
        }
    }
}

impl Stream for DownloadStream {
    type Item = Vec<u8>;

    fn poll_next(mut self: Pin<&mut Self>, cx: &mut Context<'_>) -> Poll<Option<Self::Item>> {
        // If we've sent all chunks, we're done
        if self.current_index >= self.chunks.len() {
            return Poll::Ready(None);
        }

        // If we haven't finished our delay, wait
        if !self.delay_finished {
            match self.delay.as_mut().poll(cx) {
                Poll::Ready(()) => {
                    self.delay_finished = true;
                }
                Poll::Pending => return Poll::Pending,
            }
        }

        // Send the current chunk
        let chunk = self.chunks[self.current_index].clone();
        self.current_index += 1;
        
        // Reset delay for next chunk
        self.delay = Box::pin(sleep(Duration::from_millis(100)));
        self.delay_finished = false;

        Poll::Ready(Some(chunk))
    }
}

/// Example of using streams to process data
pub async fn stream_processing_example() {
    println!("Starting stream processing example...");
    
    // Create a stream of numbers
    let stream = stream::iter(1..=10);
    
    // Process each item in the stream
    let mut processed_stream = stream.map(|x| x * 2);
    
    println!("Processing stream items:");
    while let Some(value) = processed_stream.next().await {
        println!("Processed value: {}", value);
    }
}

/// Example of using a custom stream
pub async fn custom_stream_example() {
    println!("Starting custom stream example...");
    
    // Create a counter stream
    let mut counter_stream = CounterStream::new(5);
    
    println!("Consuming from counter stream:");
    while let Some(value) = counter_stream.next().await {
        println!("Counter value: {}", value);
    }
}

/// Example of stream with back-pressure control
pub async fn backpressure_example() {
    println!("Starting back-pressure example...");
    
    // Create some sample data chunks
    let chunks: Vec<Vec<u8>> = (0..5)
        .map(|i| vec![i; 10]) // Each chunk is 10 bytes of the same value
        .collect();
    
    // Create a download stream
    let mut download_stream = DownloadStream::new(chunks);
    
    println!("Downloading chunks with back-pressure:");
    while let Some(chunk) = download_stream.next().await {
        println!("Received chunk of {} bytes", chunk.len());
        // Simulate processing time
        tokio::time::sleep(Duration::from_millis(50)).await;
    }
}

/// Example of stream filtering and transformation
pub async fn stream_transformation_example() {
    println!("Starting stream transformation example...");
    
    // Create a stream of numbers
    let numbers = stream::iter(1..=20);
    
    // Filter even numbers, square them, and take the first 5
    let transformed: Vec<i32> = numbers
        .filter(|x| async { x % 2 == 0 })
        .map(|x| x * x)
        .take(5)
        .collect()
        .await;
    
    println!("Transformed values: {:?}", transformed);
}

/// Example of merging multiple streams
pub async fn stream_merge_example() {
    println!("Starting stream merge example...");
    
    // Create two streams
    let stream1 = stream::iter(vec![1, 3, 5, 7, 9]);
    let stream2 = stream::iter(vec![2, 4, 6, 8, 10]);
    
    // Merge the streams
    let merged = stream::select(stream1, stream2);
    
    // Collect all values
    let values: Vec<i32> = merged.collect().await;
    
    println!("Merged stream values: {:?}", values);
}

/// Example of stream error handling
pub async fn stream_error_handling_example() {
    println!("Starting stream error handling example...");
    
    // Create a stream that might produce errors
    let stream = stream::iter(vec![Ok(1), Ok(2), Err("Error occurred"), Ok(3), Ok(4)]);
    
    // Handle errors in the stream
    let results: Vec<Result<i32, &str>> = stream.collect().await;
    
    println!("Stream results:");
    for result in results {
        match result {
            Ok(value) => println!("  Value: {}", value),
            Err(error) => println!("  Error: {}", error),
        }
    }
}

/// Example of using streams with channels for back-pressure
pub async fn stream_channel_backpressure_example() {
    println!("Starting stream channel back-pressure example...");
    
    // Create a bounded channel
    let (tx, rx) = tokio::sync::mpsc::channel::<i32>(3);
    
    // Spawn a producer task
    let producer = tokio::spawn(async move {
        for i in 1..=10 {
            println!("Producing item {}", i);
            if tx.send(i).await.is_err() {
                println!("Channel closed, stopping producer");
                break;
            }
            // Simulate work
            tokio::time::sleep(Duration::from_millis(100)).await;
        }
    });
    
    // Create a stream from the receiver
    let mut stream = tokio_stream::wrappers::ReceiverStream::new(rx);
    
    // Consumer that processes items slowly (creating back-pressure)
    println!("Consuming items:");
    while let Some(item) = stream.next().await {
        println!("Consuming item {}", item);
        // Simulate slow processing
        tokio::time::sleep(Duration::from_millis(300)).await;
    }
    
    // Wait for producer to finish
    producer.await.unwrap();
}

/// Example usage of stream functions
pub fn example_usage() {
    println!("Streams and Back-Pressure Examples");
    println!("=================================");
    
    println!("\n1. Stream processing example:");
    // Note: This would need to be called in an async context
    println!("   Call stream_processing_example().await to see this in action");
    
    println!("\n2. Custom stream example:");
    println!("   Call custom_stream_example().await to see this in action");
    
    println!("\n3. Back-pressure example:");
    println!("   Call backpressure_example().await to see this in action");
    
    println!("\n4. Stream transformation example:");
    println!("   Call stream_transformation_example().await to see this in action");
    
    println!("\n5. Stream merge example:");
    println!("   Call stream_merge_example().await to see this in action");
    
    println!("\n6. Stream error handling example:");
    println!("   Call stream_error_handling_example().await to see this in action");
    
    println!("\n7. Stream channel back-pressure example:");
    println!("   Call stream_channel_backpressure_example().await to see this in action");
}

#[cfg(test)]
mod tests {
    use super::*;
    use futures::stream::StreamExt;

    #[tokio::test]
    async fn test_counter_stream() {
        let mut stream = CounterStream::new(3);
        
        assert_eq!(stream.next().await, Some(1));
        assert_eq!(stream.next().await, Some(2));
        assert_eq!(stream.next().await, Some(3));
        assert_eq!(stream.next().await, None);
    }

    #[tokio::test]
    async fn test_stream_processing() {
        let stream = stream::iter(vec![1, 2, 3, 4, 5]);
        let processed: Vec<i32> = stream.map(|x| x * 2).collect().await;
        
        assert_eq!(processed, vec![2, 4, 6, 8, 10]);
    }

    #[tokio::test]
    async fn test_stream_filtering() {
        let stream = stream::iter(vec![1, 2, 3, 4, 5, 6, 7, 8, 9, 10]);
        let filtered: Vec<i32> = stream
            .filter(|x| async { x % 2 == 0 })
            .collect()
            .await;
        
        assert_eq!(filtered, vec![2, 4, 6, 8, 10]);
    }
}
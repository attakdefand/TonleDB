//! Channel examples for image processing pipeline
//!
//! This module demonstrates how to use crossbeam channels to create
//! an image processing pipeline where stages communicate via channels
//! to apply filters in sequence.

use crossbeam::channel::{bounded, unbounded, Receiver, Sender};
use std::thread;
use std::time::Duration;

/// Represents an image in our pipeline
#[derive(Debug, Clone)]
pub struct Image {
    pub id: u32,
    pub data: Vec<u8>,
    pub width: u32,
    pub height: u32,
}

/// A filter that can be applied to an image
pub trait ImageFilter: Send + Sync {
    fn apply(&self, image: Image) -> Image;
    fn name(&self) -> &str;
}

/// A blur filter
pub struct BlurFilter {
    intensity: f32,
}

impl BlurFilter {
    pub fn new(intensity: f32) -> Self {
        Self { intensity }
    }
}

impl ImageFilter for BlurFilter {
    fn apply(&self, mut image: Image) -> Image {
        // Simulate blur processing
        println!("Applying blur filter (intensity: {}) to image {}", 
                 self.intensity, image.id);
        
        // In a real implementation, we would actually blur the image data
        // For this example, we'll just modify some bytes to simulate work
        for i in 0..std::cmp::min(10, image.data.len()) {
            image.data[i] = (image.data[i] as f32 * self.intensity) as u8;
        }
        
        thread::sleep(Duration::from_millis(50)); // Simulate processing time
        image
    }
    
    fn name(&self) -> &str {
        "Blur"
    }
}

/// A sharpen filter
pub struct SharpenFilter {
    strength: f32,
}

impl SharpenFilter {
    pub fn new(strength: f32) -> Self {
        Self { strength }
    }
}

impl ImageFilter for SharpenFilter {
    fn apply(&self, mut image: Image) -> Image {
        // Simulate sharpen processing
        println!("Applying sharpen filter (strength: {}) to image {}", 
                 self.strength, image.id);
        
        // In a real implementation, we would actually sharpen the image data
        // For this example, we'll just modify some bytes to simulate work
        for i in 0..std::cmp::min(10, image.data.len()) {
            image.data[i] = (image.data[i] as f32 * self.strength) as u8;
        }
        
        thread::sleep(Duration::from_millis(75)); // Simulate processing time
        image
    }
    
    fn name(&self) -> &str {
        "Sharpen"
    }
}

/// A brightness filter
pub struct BrightnessFilter {
    factor: f32,
}

impl BrightnessFilter {
    pub fn new(factor: f32) -> Self {
        Self { factor }
    }
}

impl ImageFilter for BrightnessFilter {
    fn apply(&self, mut image: Image) -> Image {
        // Simulate brightness adjustment
        println!("Applying brightness filter (factor: {}) to image {}", 
                 self.factor, image.id);
        
        // In a real implementation, we would actually adjust brightness
        // For this example, we'll just modify some bytes to simulate work
        for i in 0..std::cmp::min(10, image.data.len()) {
            image.data[i] = (image.data[i] as f32 * self.factor) as u8;
        }
        
        thread::sleep(Duration::from_millis(25)); // Simulate processing time
        image
    }
    
    fn name(&self) -> &str {
        "Brightness"
    }
}

/// A processing stage in the pipeline
pub struct ProcessingStage {
    filter: Box<dyn ImageFilter>,
    input: Receiver<Image>,
    output: Sender<Image>,
}

impl ProcessingStage {
    pub fn new<F: ImageFilter + 'static>(
        filter: F,
        input: Receiver<Image>,
        output: Sender<Image>,
    ) -> Self {
        Self {
            filter: Box::new(filter),
            input,
            output,
        }
    }
    
    pub fn run(self) {
        let filter_name = self.filter.name().to_string();
        println!("Starting processing stage: {}", filter_name);
        
        loop {
            match self.input.recv() {
                Ok(image) => {
                    let processed = self.filter.apply(image);
                    if let Err(_) = self.output.send(processed) {
                        println!("Pipeline stage {} shutting down", filter_name);
                        break;
                    }
                }
                Err(_) => {
                    println!("Pipeline stage {} shutting down", filter_name);
                    break;
                }
            }
        }
    }
}

/// An image processing pipeline
pub struct ImagePipeline {
    stages: Vec<thread::JoinHandle<()>>,
    input: Sender<Image>,
    output: Receiver<Image>,
}

impl ImagePipeline {
    pub fn new() -> Self {
        let (input_tx, input_rx) = unbounded();
        let (output_tx, output_rx) = unbounded();
        
        Self {
            stages: Vec::new(),
            input: input_tx,
            output: output_rx,
        }
    }
    
    /// Add a processing stage to the pipeline
    pub fn add_stage<F: ImageFilter + 'static>(&mut self, filter: F) {
        let (next_tx, next_rx) = unbounded();
        
        // For the first stage, use the pipeline's input
        // For subsequent stages, create a new channel
        let (input_rx, output_tx) = if self.stages.is_empty() {
            // First stage: use pipeline input and this stage's output
            (self.input.clone(), next_tx)
        } else {
            // Subsequent stages: we need to recreate the pipeline with new channels
            // This is a simplified approach - in a real implementation, you might
            // want to restructure this differently
            unimplemented!("Adding stages to existing pipeline not implemented in this example")
        };
        
        let stage = ProcessingStage::new(filter, input_rx, output_tx);
        let handle = thread::spawn(move || {
            stage.run();
        });
        
        self.stages.push(handle);
    }
    
    /// Start the pipeline
    pub fn start(&mut self) {
        // In this simplified example, we're just showing the structure
        // A full implementation would manage the channels between stages
        println!("Pipeline started with {} stages", self.stages.len());
    }
    
    /// Send an image through the pipeline
    pub fn process_image(&self, image: Image) -> Result<Image, Box<dyn std::error::Error>> {
        self.input.send(image)?;
        let processed = self.output.recv()?;
        Ok(processed)
    }
    
    /// Shutdown the pipeline
    pub fn shutdown(self) {
        drop(self.input); // Close the input channel
        
        // Wait for all stages to finish
        for stage in self.stages {
            stage.join().unwrap();
        }
    }
}

/// Create a simple linear pipeline with bounded channels
pub fn create_linear_pipeline() -> (Sender<Image>, Receiver<Image>) {
    // Create channels for each stage
    let (input_tx, stage1_rx) = bounded(2);
    let (stage1_tx, stage2_rx) = bounded(2);
    let (stage2_tx, stage3_rx) = bounded(2);
    let (stage3_tx, output_rx) = bounded(2);
    
    // Create filters
    let blur_filter = BlurFilter::new(0.5);
    let sharpen_filter = SharpenFilter::new(1.2);
    let brightness_filter = BrightnessFilter::new(1.1);
    
    // Start stage 1 (Blur)
    let stage1_input = stage1_rx.clone();
    let stage1_output = stage1_tx.clone();
    thread::spawn(move || {
        while let Ok(image) = stage1_input.recv() {
            let processed = blur_filter.apply(image);
            if stage1_output.send(processed).is_err() {
                break;
            }
        }
        println!("Blur stage finished");
    });
    
    // Start stage 2 (Sharpen)
    let stage2_input = stage2_rx.clone();
    let stage2_output = stage2_tx.clone();
    thread::spawn(move || {
        while let Ok(image) = stage2_input.recv() {
            let processed = sharpen_filter.apply(image);
            if stage2_output.send(processed).is_err() {
                break;
            }
        }
        println!("Sharpen stage finished");
    });
    
    // Start stage 3 (Brightness)
    let stage3_input = stage3_rx.clone();
    let stage3_output = stage3_tx.clone();
    thread::spawn(move || {
        while let Ok(image) = stage3_input.recv() {
            let processed = brightness_filter.apply(image);
            if stage3_output.send(processed).is_err() {
                break;
            }
        }
        println!("Brightness stage finished");
    });
    
    (input_tx, output_rx)
}

/// Example of using channels for an image processing pipeline
pub fn image_processing_pipeline_example() {
    println!("Creating image processing pipeline...");
    
    let (input_tx, output_rx) = create_linear_pipeline();
    
    // Spawn a thread to collect results
    let collector = thread::spawn(move || {
        let mut results = Vec::new();
        for _ in 0..5 {
            match output_rx.recv() {
                Ok(image) => {
                    println!("Received processed image {}", image.id);
                    results.push(image);
                }
                Err(e) => {
                    println!("Error receiving image: {}", e);
                    break;
                }
            }
        }
        results
    });
    
    // Send images through the pipeline
    for i in 1..=5 {
        let image = Image {
            id: i,
            data: vec![100; 100], // Simulated image data
            width: 640,
            height: 480,
        };
        
        println!("Sending image {} to pipeline", i);
        if let Err(e) = input_tx.send(image) {
            println!("Error sending image {}: {}", i, e);
            break;
        }
    }
    
    // Wait for results
    let results = collector.join().unwrap();
    println!("Processed {} images", results.len());
}

/// Example of using channels for producer-consumer pattern
pub fn producer_consumer_example() {
    println!("Starting producer-consumer example...");
    
    // Create a bounded channel
    let (tx, rx) = bounded(5);
    
    // Spawn producer threads
    let producers: Vec<_> = (0..3)
        .map(|i| {
            let tx = tx.clone();
            thread::spawn(move || {
                for j in 0..5 {
                    let data = format!("Producer {} - Item {}", i, j);
                    println!("Producing: {}", data);
                    if let Err(_) = tx.send(data) {
                        println!("Producer {} stopped", i);
                        break;
                    }
                    thread::sleep(Duration::from_millis(100));
                }
            })
        })
        .collect();
    
    // Spawn consumer thread
    let consumer = thread::spawn(move || {
        let mut count = 0;
        while let Ok(data) = rx.recv() {
            println!("Consuming: {}", data);
            count += 1;
            if count >= 15 {
                break;
            }
        }
        count
    });
    
    // Wait for producers to finish
    for producer in producers {
        producer.join().unwrap();
    }
    
    // Close the channel by dropping the sender
    drop(tx);
    
    // Wait for consumer to finish
    let consumed = consumer.join().unwrap();
    println!("Consumed {} items", consumed);
}

/// Example of using select! macro with multiple channels
pub fn channel_select_example() {
    println!("Starting channel selection example...");
    
    let (tx1, rx1) = unbounded();
    let (tx2, rx2) = unbounded();
    let (tx3, rx3) = unbounded();
    
    // Spawn threads that send data at different intervals
    let _sender1 = thread::spawn(move || {
        for i in 0..5 {
            thread::sleep(Duration::from_millis(200));
            tx1.send(format!("Channel 1 - Message {}", i)).unwrap();
        }
    });
    
    let _sender2 = thread::spawn(move || {
        for i in 0..3 {
            thread::sleep(Duration::from_millis(300));
            tx2.send(format!("Channel 2 - Message {}", i)).unwrap();
        }
    });
    
    let _sender3 = thread::spawn(move || {
        for i in 0..2 {
            thread::sleep(Duration::from_millis(500));
            tx3.send(format!("Channel 3 - Message {}", i)).unwrap();
        }
    });
    
    // Use select to receive from multiple channels
    let mut received = 0;
    while received < 10 {
        crossbeam::select! {
            recv(rx1) -> msg => {
                if let Ok(data) = msg {
                    println!("Received from channel 1: {}", data);
                    received += 1;
                }
            }
            recv(rx2) -> msg => {
                if let Ok(data) = msg {
                    println!("Received from channel 2: {}", data);
                    received += 1;
                }
            }
            recv(rx3) -> msg => {
                if let Ok(data) = msg {
                    println!("Received from channel 3: {}", data);
                    received += 1;
                }
            }
        }
    }
    
    println!("Channel selection example completed");
}

/// Example usage of channel functions
pub fn example_usage() {
    println!("Channel Examples for Image Processing Pipeline");
    println!("=============================================");
    
    println!("\n1. Image processing pipeline example:");
    image_processing_pipeline_example();
    
    println!("\n2. Producer-consumer example:");
    producer_consumer_example();
    
    println!("\n3. Channel selection example:");
    channel_select_example();
}

#[cfg(test)]
mod tests {
    use super::*;
    use crossbeam::channel::unbounded;

    #[test]
    fn test_image_filters() {
        let image = Image {
            id: 1,
            data: vec![100, 150, 200],
            width: 100,
            height: 100,
        };
        
        let blur_filter = BlurFilter::new(0.5);
        let sharpen_filter = SharpenFilter::new(1.2);
        let brightness_filter = BrightnessFilter::new(1.1);
        
        let blurred = blur_filter.apply(image.clone());
        let sharpened = sharpen_filter.apply(image.clone());
        let brightened = brightness_filter.apply(image.clone());
        
        // Just verify they return images - actual image processing would be more complex
        assert_eq!(blurred.id, 1);
        assert_eq!(sharpened.id, 1);
        assert_eq!(brightened.id, 1);
    }

    #[test]
    fn test_producer_consumer() {
        let (tx, rx) = bounded(5);
        
        // Producer
        let producer = thread::spawn(move || {
            for i in 0..10 {
                tx.send(i).unwrap();
            }
        });
        
        // Consumer
        let consumer = thread::spawn(move || {
            let mut sum = 0;
            for _ in 0..10 {
                sum += rx.recv().unwrap();
            }
            sum
        });
        
        producer.join().unwrap();
        let sum = consumer.join().unwrap();
        
        // Sum of 0..10 is 45
        assert_eq!(sum, 45);
    }
}
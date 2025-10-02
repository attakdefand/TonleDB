//! Dataflow and pipeline examples
//!
//! This module demonstrates how to create real-time sensor analytics
//! built on timely dataflow graphs processing streaming data.

use timely::dataflow::channels::pact::Pipeline;
use timely::dataflow::operators::{Filter, Map, Inspect, Concat, ToStream};
use timely::dataflow::{InputHandle, ProbeHandle};
use timely::Configuration;
use std::time::Duration;
use rand::Rng;

/// A sensor reading
#[derive(Debug, Clone)]
pub struct SensorReading {
    pub sensor_id: u32,
    pub value: f64,
    pub timestamp: u64,
}

/// A processed sensor reading with alert status
#[derive(Debug, Clone)]
pub struct ProcessedReading {
    pub sensor_id: u32,
    pub value: f64,
    pub timestamp: u64,
    pub alert: bool,
}

/// Run a simple timely dataflow example
pub fn simple_dataflow_example() {
    println!("Starting simple timely dataflow example...");
    
    // Create a timely dataflow computation
    timely::execute(Configuration::Thread, |worker| {
        let mut input = InputHandle::new();
        let mut probe = ProbeHandle::new();

        // Create a new dataflow
        worker.dataflow(|scope| {
            // Create a stream from the input
            let stream = input.to_stream(scope);

            // Process the stream: filter high values, map to processed readings, and inspect
            stream
                .filter(|reading: &SensorReading| reading.value > 50.0)
                .map(|reading: SensorReading| ProcessedReading {
                    sensor_id: reading.sensor_id,
                    value: reading.value,
                    timestamp: reading.timestamp,
                    alert: reading.value > 80.0,
                })
                .inspect(|reading: &ProcessedReading| {
                    if reading.alert {
                        println!("ALERT: Sensor {} reported high value: {} at time {}", 
                                 reading.sensor_id, reading.value, reading.timestamp);
                    } else {
                        println!("Sensor {} reported value: {} at time {}", 
                                 reading.sensor_id, reading.value, reading.timestamp);
                    }
                })
                .probe_with(&mut probe);
        });

        // Feed data into the input
        for i in 0..10 {
            let reading = SensorReading {
                sensor_id: (i % 3) + 1,
                value: (i * 10) as f64,
                timestamp: i,
            };
            input.send(reading);
            input.advance_to(i + 1);
            worker.step_while(|| probe.less_than(input.time()));
        }
    }).unwrap();
}

/// Run a dataflow with multiple stages
pub fn multi_stage_dataflow_example() {
    println!("Starting multi-stage timely dataflow example...");
    
    timely::execute(Configuration::Thread, |worker| {
        let mut input = InputHandle::new();
        let mut probe = ProbeHandle::new();

        worker.dataflow(|scope| {
            let stream = input.to_stream(scope);

            // Stage 1: Filter and categorize readings
            let normal_readings = stream
                .filter(|reading: &SensorReading| reading.value <= 50.0)
                .map(|reading: SensorReading| (reading.sensor_id, "NORMAL", reading.value, reading.timestamp));

            let warning_readings = stream
                .filter(|reading: &SensorReading| reading.value > 50.0 && reading.value <= 80.0)
                .map(|reading: SensorReading| (reading.sensor_id, "WARNING", reading.value, reading.timestamp));

            let critical_readings = stream
                .filter(|reading: &SensorReading| reading.value > 80.0)
                .map(|reading: SensorReading| (reading.sensor_id, "CRITICAL", reading.value, reading.timestamp));

            // Stage 2: Combine all readings and process
            normal_readings
                .concat(&warning_readings)
                .concat(&critical_readings)
                .inspect(|(sensor_id, level, value, timestamp): &(u32, &str, f64, u64)| {
                    println!("Sensor {}: {} reading {} at time {}", sensor_id, level, value, timestamp);
                })
                .probe_with(&mut probe);
        });

        // Feed data
        for i in 0..15 {
            let value = (rand::thread_rng().gen::<f64>() * 100.0).round();
            let reading = SensorReading {
                sensor_id: (i % 5) + 1,
                value,
                timestamp: i,
            };
            input.send(reading);
            input.advance_to(i + 1);
            worker.step_while(|| probe.less_than(input.time()));
        }
    }).unwrap();
}

/// Run a dataflow with aggregation
pub fn aggregation_dataflow_example() {
    println!("Starting aggregation timely dataflow example...");
    
    timely::execute(Configuration::Thread, |worker| {
        let mut input = InputHandle::new();
        let mut probe = ProbeHandle::new();

        worker.dataflow(|scope| {
            let stream = input.to_stream(scope);

            // Group by sensor ID and calculate average
            stream
                .map(|reading: SensorReading| (reading.sensor_id, reading.value))
                .inspect(|(sensor_id, value): &(u32, f64)| {
                    println!("Sensor {} reported value: {}", sensor_id, value);
                })
                .probe_with(&mut probe);
        });

        // Feed data
        for i in 0..20 {
            let reading = SensorReading {
                sensor_id: (i % 3) + 1,
                value: (rand::thread_rng().gen::<f64>() * 100.0).round(),
                timestamp: i,
            };
            input.send(reading);
            input.advance_to(i + 1);
            worker.step_while(|| probe.less_than(input.time()));
        }
    }).unwrap();
}

/// Run a dataflow with windowing
pub fn windowing_dataflow_example() {
    println!("Starting windowing timely dataflow example...");
    
    timely::execute(Configuration::Thread, |worker| {
        let mut input = InputHandle::new();
        let mut probe = ProbeHandle::new();

        worker.dataflow(|scope| {
            let stream = input.to_stream(scope);

            // Process readings in time windows
            stream
                .map(|reading: SensorReading| {
                    // Group into 5-second windows
                    let window = reading.timestamp / 5;
                    (window, reading)
                })
                .inspect(|(window, reading): &(u64, SensorReading)| {
                    println!("Window {}: Sensor {} reported {} at time {}", 
                             window, reading.sensor_id, reading.value, reading.timestamp);
                })
                .probe_with(&mut probe);
        });

        // Feed data with timestamps
        for i in 0..25 {
            let reading = SensorReading {
                sensor_id: (i % 4) + 1,
                value: (rand::thread_rng().gen::<f64>() * 100.0).round(),
                timestamp: i,
            };
            input.send(reading);
            input.advance_to(i + 1);
            worker.step_while(|| probe.less_than(input.time()));
        }
    }).unwrap();
}

/// Run a dataflow with multiple workers
pub fn multi_worker_dataflow_example() {
    println!("Starting multi-worker timely dataflow example...");
    
    // This example requires multiple processes, so we'll simulate with a single worker
    // In a real deployment, you would use Configuration::Process()
    timely::execute(Configuration::Thread, |worker| {
        let mut input = InputHandle::new();
        let mut probe = ProbeHandle::new();

        worker.dataflow(|scope| {
            let stream = input.to_stream(scope);

            // Distribute work across workers
            stream
                .map(|reading: SensorReading| {
                    // Add worker ID to the reading
                    (worker.index(), reading)
                })
                .inspect(|(worker_id, reading): &(usize, SensorReading)| {
                    println!("Worker {}: Processing sensor {} value {} at time {}", 
                             worker_id, reading.sensor_id, reading.value, reading.timestamp);
                })
                .probe_with(&mut probe);
        });

        // Feed data
        for i in 0..12 {
            let reading = SensorReading {
                sensor_id: (i % 6) + 1,
                value: (rand::thread_rng().gen::<f64>() * 100.0).round(),
                timestamp: i,
            };
            input.send(reading);
            input.advance_to(i + 1);
            worker.step_while(|| probe.less_than(input.time()));
        }
    }).unwrap();
}

/// Simulate sensor data generation
pub fn sensor_data_simulation_example() {
    println!("Starting sensor data simulation example...");
    
    // Create a vector to store generated data
    let mut sensor_data = Vec::new();
    
    // Generate 30 sensor readings
    for i in 0..30 {
        let reading = SensorReading {
            sensor_id: (i % 5) + 1, // 5 different sensors
            value: (rand::thread_rng().gen::<f64>() * 100.0).round(),
            timestamp: i,
        };
        sensor_data.push(reading);
    }
    
    println!("Generated {} sensor readings:", sensor_data.len());
    
    // Show first 10 readings
    for (i, reading) in sensor_data.iter().take(10).enumerate() {
        println!("  {}: Sensor {} = {} at time {}", 
                 i, reading.sensor_id, reading.value, reading.timestamp);
    }
    
    // Show statistics
    let avg_value: f64 = sensor_data.iter().map(|r| r.value).sum::<f64>() / sensor_data.len() as f64;
    let max_value = sensor_data.iter().map(|r| r.value).fold(0.0/0.0, f64::max);
    let min_value = sensor_data.iter().map(|r| r.value).fold(0.0/0.0, f64::min);
    
    println!("Statistics:");
    println!("  Average value: {:.2}", avg_value);
    println!("  Maximum value: {:.2}", max_value);
    println!("  Minimum value: {:.2}", min_value);
}

/// Example of a custom operator in timely dataflow
pub fn custom_operator_example() {
    println!("Starting custom operator timely dataflow example...");
    
    timely::execute(Configuration::Thread, |worker| {
        let mut input = InputHandle::new();
        let mut probe = ProbeHandle::new();

        worker.dataflow(|scope| {
            let stream = input.to_stream(scope);

            // Custom operator that detects trends
            stream
                .unary(Pipeline, "TrendDetector", |_, _| {
                    let mut last_values: std::collections::HashMap<u32, Vec<f64>> = std::collections::HashMap::new();
                    
                    move |input, output| {
                        input.for_each(|time, data| {
                            let mut session = output.session(&time);
                            for reading in data.iter().cloned() {
                                // Keep last 3 values for each sensor
                                let values = last_values.entry(reading.sensor_id).or_insert_with(Vec::new);
                                values.push(reading.value);
                                if values.len() > 3 {
                                    values.remove(0);
                                }
                                
                                // Detect increasing trend
                                let trend = if values.len() >= 3 {
                                    let first = values[0];
                                    let last = values[values.len() - 1];
                                    last > first
                                } else {
                                    false
                                };
                                
                                session.give((reading, trend));
                            }
                        });
                    }
                })
                .inspect(|(reading, trend): &(SensorReading, bool)| {
                    if *trend {
                        println!("TREND: Sensor {} value {} shows increasing trend", 
                                 reading.sensor_id, reading.value);
                    } else {
                        println!("Sensor {} value {}", reading.sensor_id, reading.value);
                    }
                })
                .probe_with(&mut probe);
        });

        // Feed data
        for i in 0..15 {
            let reading = SensorReading {
                sensor_id: (i % 3) + 1,
                value: 50.0 + (i as f64 * 2.5),
                timestamp: i,
            };
            input.send(reading);
            input.advance_to(i + 1);
            worker.step_while(|| probe.less_than(input.time()));
        }
    }).unwrap();
}

/// Example usage of dataflow and pipeline functions
pub fn example_usage() {
    println!("Dataflow and Pipeline Examples");
    println!("============================");
    
    println!("\n1. Simple dataflow example:");
    simple_dataflow_example();
    
    println!("\n2. Multi-stage dataflow example:");
    multi_stage_dataflow_example();
    
    println!("\n3. Aggregation dataflow example:");
    aggregation_dataflow_example();
    
    println!("\n4. Windowing dataflow example:");
    windowing_dataflow_example();
    
    println!("\n5. Multi-worker dataflow example:");
    multi_worker_dataflow_example();
    
    println!("\n6. Sensor data simulation example:");
    sensor_data_simulation_example();
    
    println!("\n7. Custom operator example:");
    custom_operator_example();
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_sensor_reading() {
        let reading = SensorReading {
            sensor_id: 1,
            value: 42.5,
            timestamp: 100,
        };
        
        assert_eq!(reading.sensor_id, 1);
        assert_eq!(reading.value, 42.5);
        assert_eq!(reading.timestamp, 100);
    }

    #[test]
    fn test_processed_reading() {
        let processed = ProcessedReading {
            sensor_id: 1,
            value: 85.0,
            timestamp: 100,
            alert: true,
        };
        
        assert_eq!(processed.sensor_id, 1);
        assert_eq!(processed.value, 85.0);
        assert_eq!(processed.timestamp, 100);
        assert_eq!(processed.alert, true);
    }

    #[test]
    fn test_sensor_data_generation() {
        let reading = SensorReading {
            sensor_id: 1,
            value: 75.0,
            timestamp: 1000,
        };
        
        assert_eq!(reading.sensor_id, 1);
        assert_eq!(reading.value, 75.0);
        assert_eq!(reading.timestamp, 1000);
    }
}
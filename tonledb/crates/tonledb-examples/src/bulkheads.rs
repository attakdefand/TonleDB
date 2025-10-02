//! Bulkheads and thread pools examples
//!
//! This module demonstrates how to use thread pools to implement bulkheads
//! for capping concurrent jobs per bulkhead.

use std::sync::{Arc, Mutex};
use std::time::Duration;
use threadpool::ThreadPool;

/// A bulkhead that limits the number of concurrent operations
pub struct Bulkhead {
    pool: ThreadPool,
    name: String,
    max_concurrent: usize,
}

impl Bulkhead {
    /// Create a new bulkhead with a specified name and maximum concurrent operations
    pub fn new(name: &str, max_concurrent: usize) -> Self {
        Self {
            pool: ThreadPool::new(max_concurrent),
            name: name.to_string(),
            max_concurrent,
        }
    }

    /// Execute a task within this bulkhead
    pub fn execute<F>(&self, task: F)
    where
        F: FnOnce() + Send + 'static,
    {
        let name = self.name.clone();
        self.pool.execute(move || {
            println!("Task started in bulkhead: {}", name);
            task();
            println!("Task completed in bulkhead: {}", name);
        });
    }

    /// Get the number of active threads in this bulkhead
    pub fn active_count(&self) -> usize {
        self.pool.active_count()
    }

    /// Get the number of pending jobs in this bulkhead
    pub fn queued_count(&self) -> usize {
        self.pool.queued_count()
    }

    /// Wait for all tasks to complete
    pub fn join(&self) {
        self.pool.join();
    }
}

/// A system with multiple bulkheads for different types of work
pub struct BulkheadSystem {
    database_bulkhead: Bulkhead,
    file_bulkhead: Bulkhead,
    network_bulkhead: Bulkhead,
}

impl BulkheadSystem {
    /// Create a new bulkhead system
    pub fn new() -> Self {
        Self {
            database_bulkhead: Bulkhead::new("database", 5),
            file_bulkhead: Bulkhead::new("file", 3),
            network_bulkhead: Bulkhead::new("network", 10),
        }
    }

    /// Execute a database operation within the database bulkhead
    pub fn execute_database_task<F>(&self, task: F)
    where
        F: FnOnce() + Send + 'static,
    {
        self.database_bulkhead.execute(task);
    }

    /// Execute a file operation within the file bulkhead
    pub fn execute_file_task<F>(&self, task: F)
    where
        F: FnOnce() + Send + 'static,
    {
        self.file_bulkhead.execute(task);
    }

    /// Execute a network operation within the network bulkhead
    pub fn execute_network_task<F>(&self, task: F)
    where
        F: FnOnce() + Send + 'static,
    {
        self.network_bulkhead.execute(task);
    }

    /// Get status of all bulkheads
    pub fn status(&self) -> BulkheadStatus {
        BulkheadStatus {
            database: BulkheadInfo {
                name: "database".to_string(),
                active: self.database_bulkhead.active_count(),
                queued: self.database_bulkhead.queued_count(),
                max_concurrent: 5,
            },
            file: BulkheadInfo {
                name: "file".to_string(),
                active: self.file_bulkhead.active_count(),
                queued: self.file_bulkhead.queued_count(),
                max_concurrent: 3,
            },
            network: BulkheadInfo {
                name: "network".to_string(),
                active: self.network_bulkhead.active_count(),
                queued: self.network_bulkhead.queued_count(),
                max_concurrent: 10,
            },
        }
    }

    /// Wait for all tasks to complete
    pub fn join(&self) {
        self.database_bulkhead.join();
        self.file_bulkhead.join();
        self.network_bulkhead.join();
    }
}

/// Information about a bulkhead
#[derive(Debug)]
pub struct BulkheadInfo {
    pub name: String,
    pub active: usize,
    pub queued: usize,
    pub max_concurrent: usize,
}

/// Status of all bulkheads in the system
#[derive(Debug)]
pub struct BulkheadStatus {
    pub database: BulkheadInfo,
    pub file: BulkheadInfo,
    pub network: BulkheadInfo,
}

/// Background task executor with thread pool
pub struct BackgroundTaskExecutor {
    pool: ThreadPool,
    task_counter: Arc<Mutex<usize>>,
}

impl BackgroundTaskExecutor {
    /// Create a new background task executor with a specified number of threads
    pub fn new(num_threads: usize) -> Self {
        Self {
            pool: ThreadPool::new(num_threads),
            task_counter: Arc::new(Mutex::new(0)),
        }
    }

    /// Submit a background task
    pub fn submit_task<F>(&self, task: F) -> usize
    where
        F: FnOnce() + Send + 'static,
    {
        let counter = self.task_counter.clone();
        let task_id = {
            let mut counter = counter.lock().unwrap();
            *counter += 1;
            *counter
        };

        self.pool.execute(move || {
            println!("Background task {} started", task_id);
            task();
            println!("Background task {} completed", task_id);
        });

        task_id
    }

    /// Get the number of active threads
    pub fn active_count(&self) -> usize {
        self.pool.active_count()
    }

    /// Get the number of pending jobs
    pub fn queued_count(&self) -> usize {
        self.pool.queued_count()
    }

    /// Wait for all tasks to complete
    pub fn join(&self) {
        self.pool.join();
    }
}

/// Example of using bulkheads to limit concurrent database operations
pub fn database_bulkhead_example() {
    println!("Creating database bulkhead with max 3 concurrent operations...");
    
    let bulkhead = Bulkhead::new("database", 3);
    
    // Submit more tasks than the bulkhead can handle concurrently
    for i in 1..=10 {
        bulkhead.execute(move || {
            println!("Database task {} started", i);
            // Simulate database work
            std::thread::sleep(Duration::from_millis(500));
            println!("Database task {} completed", i);
        });
    }
    
    println!("Submitted 10 database tasks to bulkhead with max 3 concurrent");
    println!("Active tasks: {}, Queued tasks: {}", 
             bulkhead.active_count(), bulkhead.queued_count());
    
    // Wait for all tasks to complete
    bulkhead.join();
    println!("All database tasks completed");
}

/// Example of using a bulkhead system with different types of work
pub fn bulkhead_system_example() {
    println!("Creating bulkhead system...");
    
    let system = BulkheadSystem::new();
    
    // Submit various types of tasks
    for i in 1..=5 {
        system.execute_database_task(move || {
            println!("Database task {} working...", i);
            std::thread::sleep(Duration::from_millis(300));
        });
    }
    
    for i in 1..=5 {
        system.execute_file_task(move || {
            println!("File task {} working...", i);
            std::thread::sleep(Duration::from_millis(200));
        });
    }
    
    for i in 1..=5 {
        system.execute_network_task(move || {
            println!("Network task {} working...", i);
            std::thread::sleep(Duration::from_millis(100));
        });
    }
    
    // Check status
    let status = system.status();
    println!("Bulkhead status: {:?}", status);
    
    // Wait for all tasks to complete
    system.join();
    println!("All tasks completed");
}

/// Example of using a background task executor
pub fn background_task_executor_example() {
    println!("Creating background task executor with 4 threads...");
    
    let executor = BackgroundTaskExecutor::new(4);
    
    // Submit several background tasks
    for i in 1..=10 {
        executor.submit_task(move || {
            println!("Background task {} started", i);
            // Simulate work
            std::thread::sleep(Duration::from_millis(100 * (i as u64)));
            println!("Background task {} completed", i);
        });
    }
    
    println!("Submitted 10 background tasks");
    println!("Active tasks: {}, Queued tasks: {}", 
             executor.active_count(), executor.queued_count());
    
    // Wait for all tasks to complete
    executor.join();
    println!("All background tasks completed");
}

/// Example usage of bulkhead and thread pool functions
pub fn example_usage() {
    println!("Bulkhead and Thread Pool Examples");
    println!("================================");
    
    println!("\n1. Database bulkhead example:");
    database_bulkhead_example();
    
    println!("\n2. Bulkhead system example:");
    bulkhead_system_example();
    
    println!("\n3. Background task executor example:");
    background_task_executor_example();
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::sync::atomic::{AtomicUsize, Ordering};
    use std::time::Instant;

    #[test]
    fn test_bulkhead_limits_concurrency() {
        let bulkhead = Bulkhead::new("test", 2);
        let counter = Arc::new(AtomicUsize::new(0));
        let max_concurrent = Arc::new(AtomicUsize::new(0));

        // Submit 5 tasks
        for _ in 0..5 {
            let counter = counter.clone();
            let max_concurrent = max_concurrent.clone();
            bulkhead.execute(move || {
                let current = counter.fetch_add(1, Ordering::SeqCst) + 1;
                let prev_max = max_concurrent.fetch_max(current, Ordering::SeqCst);
                if current > prev_max {
                    max_concurrent.store(current, Ordering::SeqCst);
                }
                
                std::thread::sleep(Duration::from_millis(100));
                
                counter.fetch_sub(1, Ordering::SeqCst);
            });
        }

        bulkhead.join();

        // Maximum concurrent should not exceed the bulkhead limit
        assert!(max_concurrent.load(Ordering::SeqCst) <= 2);
    }

    #[test]
    fn test_background_task_executor() {
        let executor = BackgroundTaskExecutor::new(2);
        let counter = Arc::new(AtomicUsize::new(0));

        // Submit 5 tasks
        for _ in 0..5 {
            let counter = counter.clone();
            executor.submit_task(move || {
                counter.fetch_add(1, Ordering::SeqCst);
                std::thread::sleep(Duration::from_millis(50));
            });
        }

        executor.join();

        // All tasks should have completed
        assert_eq!(counter.load(Ordering::SeqCst), 5);
    }
}
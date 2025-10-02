//! Thread and shared memory examples
//!
//! This module demonstrates how to use std::thread::spawn and Mutex for
//! multi-threaded job scheduling with shared state.

use std::sync::{Arc, Mutex};
use std::thread;
use std::time::Duration;

/// A simple job structure
#[derive(Debug, Clone)]
pub struct Job {
    pub id: u32,
    pub name: String,
    pub priority: u8,
    pub status: String,
}

/// A thread-safe job queue
pub struct JobQueue {
    jobs: Arc<Mutex<Vec<Job>>>,
}

impl JobQueue {
    /// Create a new job queue
    pub fn new() -> Self {
        Self {
            jobs: Arc::new(Mutex::new(Vec::new())),
        }
    }

    /// Add a job to the queue
    pub fn add_job(&self, job: Job) {
        let mut jobs = self.jobs.lock().unwrap();
        jobs.push(job);
    }

    /// Get the next job from the queue
    pub fn get_next_job(&self) -> Option<Job> {
        let mut jobs = self.jobs.lock().unwrap();
        jobs.pop()
    }

    /// Get the number of jobs in the queue
    pub fn job_count(&self) -> usize {
        let jobs = self.jobs.lock().unwrap();
        jobs.len()
    }
}

/// Multi-threaded job scheduler
pub struct JobScheduler {
    queue: JobQueue,
    workers: Vec<thread::JoinHandle<()>>,
}

impl JobScheduler {
    /// Create a new job scheduler with the specified number of worker threads
    pub fn new(num_workers: usize) -> Self {
        let queue = JobQueue::new();
        let mut workers = Vec::new();

        // Clone the queue for each worker thread
        let queue_clone = queue.jobs.clone();

        // Spawn worker threads
        for i in 0..num_workers {
            let worker_queue = queue_clone.clone();
            let worker = thread::spawn(move || {
                worker_thread(i, worker_queue);
            });
            workers.push(worker);
        }

        Self { queue, workers }
    }

    /// Add a job to the scheduler
    pub fn add_job(&self, job: Job) {
        self.queue.add_job(job);
    }

    /// Wait for all worker threads to complete
    pub fn wait_for_completion(self) {
        // Drop our reference to the queue to signal workers to stop
        drop(self.queue);

        // Wait for all workers to finish
        for worker in self.workers {
            worker.join().unwrap();
        }
    }
}

/// Worker thread function
fn worker_thread(worker_id: usize, job_queue: Arc<Mutex<Vec<Job>>>) {
    println!("Worker {} started", worker_id);
    
    loop {
        // Try to get a job from the queue
        let job = {
            let mut jobs = job_queue.lock().unwrap();
            if jobs.is_empty() {
                // If no jobs, check if we should stop (queue was dropped)
                if Arc::strong_count(&job_queue) <= 1 {
                    println!("Worker {} stopping (no more jobs)", worker_id);
                    return;
                }
                None
            } else {
                Some(jobs.remove(0))
            }
        };

        // Process the job if we got one
        if let Some(job) = job {
            println!("Worker {} processing job {}: {}", worker_id, job.id, job.name);
            
            // Simulate work
            thread::sleep(Duration::from_millis(100 * (job.priority as u64)));
            
            println!("Worker {} completed job {}", worker_id, job.id);
        } else {
            // No job available, sleep briefly before checking again
            thread::sleep(Duration::from_millis(10));
        }
    }
}

/// Example of basic thread spawning
pub fn basic_thread_example() {
    println!("Spawning basic threads...");
    
    let handles: Vec<thread::JoinHandle<()>> = (0..5)
        .map(|i| {
            thread::spawn(move || {
                println!("Thread {} is running", i);
                thread::sleep(Duration::from_millis(100 * (i as u64 + 1)));
                println!("Thread {} completed", i);
            })
        })
        .collect();

    // Wait for all threads to complete
    for handle in handles {
        handle.join().unwrap();
    }
    
    println!("All threads completed");
}

/// Example of shared memory with Mutex
pub fn shared_memory_example() {
    println!("Demonstrating shared memory with Mutex...");
    
    // Create a shared counter
    let counter = Arc::new(Mutex::new(0));
    let mut handles = vec![];

    // Spawn 10 threads that increment the counter
    for i in 0..10 {
        let counter = Arc::clone(&counter);
        let handle = thread::spawn(move || {
            let mut num = counter.lock().unwrap();
            *num += 1;
            println!("Thread {} incremented counter to {}", i, *num);
        });
        handles.push(handle);
    }

    // Wait for all threads to complete
    for handle in handles {
        handle.join().unwrap();
    }

    println!("Final counter value: {}", *counter.lock().unwrap());
}

/// Example usage of thread and shared memory functions
pub fn example_usage() {
    println!("Thread and Shared Memory Examples");
    println!("================================");
    
    println!("\n1. Basic thread spawning:");
    basic_thread_example();
    
    println!("\n2. Shared memory with Mutex:");
    shared_memory_example();
    
    println!("\n3. Multi-threaded job scheduler:");
    job_scheduler_example();
}

/// Example of using the job scheduler
pub fn job_scheduler_example() {
    println!("Creating job scheduler with 3 worker threads...");
    
    let scheduler = JobScheduler::new(3);
    
    // Add some jobs to the scheduler
    for i in 1..=10 {
        scheduler.add_job(Job {
            id: i,
            name: format!("Job {}", i),
            priority: (i % 3) as u8,
            status: "pending".to_string(),
        });
    }
    
    println!("Added 10 jobs to the scheduler. Processing...");
    
    // In a real application, you would continue with other work
    // For this example, we'll just wait a bit and then stop
    thread::sleep(Duration::from_secs(2));
    
    // Wait for completion (in a real app, you might not want to do this immediately)
    scheduler.wait_for_completion();
    
    println!("Job scheduler completed");
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_job_queue() {
        let queue = JobQueue::new();
        
        assert_eq!(queue.job_count(), 0);
        
        queue.add_job(Job {
            id: 1,
            name: "Test Job".to_string(),
            priority: 1,
            status: "pending".to_string(),
        });
        
        assert_eq!(queue.job_count(), 1);
        
        let job = queue.get_next_job();
        assert!(job.is_some());
        assert_eq!(queue.job_count(), 0);
    }

    #[test]
    fn test_shared_counter() {
        let counter = Arc::new(Mutex::new(0));
        let mut handles = vec![];

        for _ in 0..5 {
            let counter = Arc::clone(&counter);
            let handle = thread::spawn(move || {
                let mut num = counter.lock().unwrap();
                *num += 1;
            });
            handles.push(handle);
        }

        for handle in handles {
            handle.join().unwrap();
        }

        assert_eq!(*counter.lock().unwrap(), 5);
    }
}
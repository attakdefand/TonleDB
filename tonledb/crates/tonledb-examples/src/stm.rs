//! Software Transactional Memory examples
//!
//! This module demonstrates how to create an in-memory key-value cache
//! with atomic transactions via the crossbeam-epoch crate.

use crossbeam_epoch::{pin, Atomic, Guard, Owned, Pointer, Shared};
use crossbeam_utils::thread;
use std::collections::HashMap;
use std::sync::atomic::Ordering;
use std::sync::{Arc, Mutex};

/// A simple key-value cache using software transactional memory
pub struct StmCache<K, V> {
    data: Atomic<HashMap<K, V>>,
}

impl<K, V> StmCache<K, V>
where
    K: Clone + std::hash::Hash + Eq,
    V: Clone,
{
    /// Create a new STM cache
    pub fn new() -> Self {
        let initial_map = HashMap::new();
        Self {
            data: Atomic::new(initial_map),
        }
    }

    /// Get a value from the cache
    pub fn get(&self, key: &K) -> Option<V> {
        let guard = pin();
        let map = self.data.load(Ordering::Acquire, &guard);
        unsafe {
            map.as_ref().and_then(|m| m.get(key).cloned())
        }
    }

    /// Insert a value into the cache
    pub fn insert(&self, key: K, value: V) {
        let guard = pin();
        let current_map = self.data.load(Ordering::Acquire, &guard);
        unsafe {
            let mut new_map = current_map.as_ref().unwrap().clone();
            new_map.insert(key, value);
            let new_map_ptr = Owned::new(new_map).into_shared(&guard);
            self.data.store(new_map_ptr, Ordering::Release);
        }
    }

    /// Remove a value from the cache
    pub fn remove(&self, key: &K) -> Option<V> {
        let guard = pin();
        let current_map = self.data.load(Ordering::Acquire, &guard);
        unsafe {
            let mut new_map = current_map.as_ref().unwrap().clone();
            let removed = new_map.remove(key);
            let new_map_ptr = Owned::new(new_map).into_shared(&guard);
            self.data.store(new_map_ptr, Ordering::Release);
            removed
        }
    }

    /// Get the size of the cache
    pub fn len(&self) -> usize {
        let guard = pin();
        let map = self.data.load(Ordering::Acquire, &guard);
        unsafe {
            map.as_ref().map(|m| m.len()).unwrap_or(0)
        }
    }

    /// Check if the cache is empty
    pub fn is_empty(&self) -> bool {
        self.len() == 0
    }
}

impl<K, V> Drop for StmCache<K, V>
where
    K: Clone + std::hash::Hash + Eq,
    V: Clone,
{
    fn drop(&mut self) {
        let guard = pin();
        let map = self.data.load(Ordering::Acquire, &guard);
        unsafe {
            if !map.is_null() {
                drop(map.into_owned());
            }
        }
    }
}

/// A transactional key-value store
pub struct TransactionalStore<K, V> {
    data: Arc<Mutex<HashMap<K, V>>>,
}

impl<K, V> TransactionalStore<K, V>
where
    K: Clone + std::hash::Hash + Eq,
    V: Clone,
{
    /// Create a new transactional store
    pub fn new() -> Self {
        Self {
            data: Arc::new(Mutex::new(HashMap::new())),
        }
    }

    /// Execute a transaction
    pub fn transaction<F, R>(&self, f: F) -> Result<R, &'static str>
    where
        F: FnOnce(&mut HashMap<K, V>) -> R,
    {
        let mut data = self.data.lock().map_err(|_| "Lock poisoned")?;
        let result = f(&mut data);
        Ok(result)
    }

    /// Get a value from the store
    pub fn get(&self, key: &K) -> Option<V> {
        let data = self.data.lock().unwrap();
        data.get(key).cloned()
    }

    /// Insert a value into the store
    pub fn insert(&self, key: K, value: V) -> Option<V> {
        let mut data = self.data.lock().unwrap();
        data.insert(key, value)
    }

    /// Remove a value from the store
    pub fn remove(&self, key: &K) -> Option<V> {
        let mut data = self.data.lock().unwrap();
        data.remove(key)
    }
}

/// A simple atomic counter using crossbeam-epoch
pub struct AtomicCounter {
    value: Atomic<i32>,
}

impl AtomicCounter {
    /// Create a new atomic counter
    pub fn new(initial: i32) -> Self {
        Self {
            value: Atomic::new(initial),
        }
    }

    /// Increment the counter
    pub fn increment(&self) -> i32 {
        let guard = pin();
        let current = self.value.load(Ordering::Acquire, &guard);
        let new_value = unsafe { current.as_ref().unwrap() + 1 };
        let new_ptr = Owned::new(new_value).into_shared(&guard);
        self.value.store(new_ptr, Ordering::Release);
        new_value
    }

    /// Get the current value
    pub fn get(&self) -> i32 {
        let guard = pin();
        let value = self.value.load(Ordering::Acquire, &guard);
        unsafe { *value.as_ref().unwrap() }
    }

    /// Decrement the counter
    pub fn decrement(&self) -> i32 {
        let guard = pin();
        let current = self.value.load(Ordering::Acquire, &guard);
        let new_value = unsafe { current.as_ref().unwrap() - 1 };
        let new_ptr = Owned::new(new_value).into_shared(&guard);
        self.value.store(new_ptr, Ordering::Release);
        new_value
    }
}

impl Drop for AtomicCounter {
    fn drop(&mut self) {
        let guard = pin();
        let value = self.value.load(Ordering::Acquire, &guard);
        unsafe {
            if !value.is_null() {
                drop(value.into_owned());
            }
        }
    }
}

/// Example of using the STM cache
pub fn stm_cache_example() {
    println!("Starting STM cache example...");
    
    let cache: StmCache<String, i32> = StmCache::new();
    
    // Insert some values
    cache.insert("key1".to_string(), 100);
    cache.insert("key2".to_string(), 200);
    cache.insert("key3".to_string(), 300);
    
    println!("Cache size: {}", cache.len());
    
    // Get values
    if let Some(value) = cache.get(&"key1".to_string()) {
        println!("Value for key1: {}", value);
    }
    
    if let Some(value) = cache.get(&"key2".to_string()) {
        println!("Value for key2: {}", value);
    }
    
    // Remove a value
    if let Some(removed) = cache.remove(&"key2".to_string()) {
        println!("Removed value for key2: {}", removed);
    }
    
    println!("Cache size after removal: {}", cache.len());
}

/// Example of using the transactional store
pub fn transactional_store_example() {
    println!("Starting transactional store example...");
    
    let store: TransactionalStore<String, i32> = TransactionalStore::new();
    
    // Insert some values
    store.insert("a".to_string(), 10);
    store.insert("b".to_string(), 20);
    store.insert("c".to_string(), 30);
    
    // Execute a transaction that modifies multiple values
    let result = store.transaction(|map| {
        let sum: i32 = map.values().sum();
        map.insert("sum".to_string(), sum);
        map.insert("count".to_string(), map.len() as i32);
        sum
    });
    
    match result {
        Ok(sum) => println!("Transaction completed. Sum: {}", sum),
        Err(e) => println!("Transaction failed: {}", e),
    }
    
    // Check the results
    if let Some(sum) = store.get(&"sum".to_string()) {
        println!("Stored sum: {}", sum);
    }
    
    if let Some(count) = store.get(&"count".to_string()) {
        println!("Stored count: {}", count);
    }
}

/// Example of using the atomic counter
pub fn atomic_counter_example() {
    println!("Starting atomic counter example...");
    
    let counter = AtomicCounter::new(0);
    
    println!("Initial counter value: {}", counter.get());
    
    // Increment the counter
    let new_value = counter.increment();
    println!("After increment: {}", new_value);
    
    // Increment again
    let new_value = counter.increment();
    println!("After increment: {}", new_value);
    
    // Decrement
    let new_value = counter.decrement();
    println!("After decrement: {}", new_value);
    
    println!("Final counter value: {}", counter.get());
}

/// Example of concurrent access to STM cache
pub fn concurrent_stm_cache_example() {
    println!("Starting concurrent STM cache example...");
    
    let cache: Arc<StmCache<String, i32>> = Arc::new(StmCache::new());
    
    // Spawn multiple threads that access the cache
    let mut handles = vec![];
    
    for i in 0..5 {
        let cache = cache.clone();
        let handle = std::thread::spawn(move || {
            for j in 0..10 {
                let key = format!("thread{}_item{}", i, j);
                cache.insert(key, (i * 10 + j) as i32);
            }
        });
        handles.push(handle);
    }
    
    // Wait for all threads to complete
    for handle in handles {
        handle.join().unwrap();
    }
    
    println!("Cache size after concurrent inserts: {}", cache.len());
    
    // Verify some values
    if let Some(value) = cache.get(&"thread0_item0".to_string()) {
        println!("Value for thread0_item0: {}", value);
    }
    
    if let Some(value) = cache.get(&"thread4_item9".to_string()) {
        println!("Value for thread4_item9: {}", value);
    }
}

/// Example of concurrent access to atomic counter
pub fn concurrent_atomic_counter_example() {
    println!("Starting concurrent atomic counter example...");
    
    let counter = Arc::new(AtomicCounter::new(0));
    
    // Spawn multiple threads that increment the counter
    let mut handles = vec![];
    
    for _ in 0..10 {
        let counter = counter.clone();
        let handle = std::thread::spawn(move || {
            for _ in 0..100 {
                counter.increment();
            }
        });
        handles.push(handle);
    }
    
    // Wait for all threads to complete
    for handle in handles {
        handle.join().unwrap();
    }
    
    println!("Final counter value after concurrent increments: {}", counter.get());
    // Expected value is 1000 (10 threads * 100 increments each)
}

/// Example usage of software transactional memory functions
pub fn example_usage() {
    println!("Software Transactional Memory Examples");
    println!("====================================");
    
    println!("\n1. STM cache example:");
    stm_cache_example();
    
    println!("\n2. Transactional store example:");
    transactional_store_example();
    
    println!("\n3. Atomic counter example:");
    atomic_counter_example();
    
    println!("\n4. Concurrent STM cache example:");
    concurrent_stm_cache_example();
    
    println!("\n5. Concurrent atomic counter example:");
    concurrent_atomic_counter_example();
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::thread;

    #[test]
    fn test_stm_cache() {
        let cache: StmCache<String, i32> = StmCache::new();
        
        assert!(cache.is_empty());
        
        cache.insert("test".to_string(), 42);
        assert_eq!(cache.len(), 1);
        
        assert_eq!(cache.get(&"test".to_string()), Some(42));
        
        assert_eq!(cache.remove(&"test".to_string()), Some(42));
        assert!(cache.is_empty());
    }

    #[test]
    fn test_transactional_store() {
        let store: TransactionalStore<String, i32> = TransactionalStore::new();
        
        store.insert("a".to_string(), 1);
        store.insert("b".to_string(), 2);
        
        let result = store.transaction(|map| {
            map.get("a").unwrap() + map.get("b").unwrap()
        });
        
        assert_eq!(result, Ok(3));
    }

    #[test]
    fn test_atomic_counter() {
        let counter = AtomicCounter::new(10);
        
        assert_eq!(counter.get(), 10);
        
        assert_eq!(counter.increment(), 11);
        assert_eq!(counter.get(), 11);
        
        assert_eq!(counter.decrement(), 10);
        assert_eq!(counter.get(), 10);
    }

    #[test]
    fn test_concurrent_access() {
        let cache: Arc<StmCache<i32, i32>> = Arc::new(StmCache::new());
        let mut handles = vec![];
        
        for i in 0..10 {
            let cache = cache.clone();
            let handle = thread::spawn(move || {
                cache.insert(i, i * 2);
            });
            handles.push(handle);
        }
        
        for handle in handles {
            handle.join().unwrap();
        }
        
        assert_eq!(cache.len(), 10);
        
        for i in 0..10 {
            assert_eq!(cache.get(&i), Some(i * 2));
        }
    }
}
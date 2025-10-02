//! Event sourcing and changefeed implementation for TonleDB

use std::sync::{Arc, RwLock};
use std::collections::HashMap;
use serde::{Deserialize, Serialize};

/// Represents a change event in the database
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ChangeEvent {
    pub id: String,
    pub timestamp: u64,
    pub operation: Operation,
    pub table: String,
    pub key: Option<Vec<u8>>,
    pub old_value: Option<Vec<u8>>,
    pub new_value: Option<Vec<u8>>,
}

/// Type of operation that caused the change
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub enum Operation {
    Insert,
    Update,
    Delete,
}

/// A changefeed subscription
pub struct ChangeFeed {
    pub id: String,
    pub table_filter: Option<String>,
    pub operation_filter: Option<Vec<Operation>>,
    pub callback: Box<dyn Fn(ChangeEvent) + Send + Sync>,
}

/// Event sourcing manager
pub struct EventSourcingManager {
    feeds: RwLock<HashMap<String, ChangeFeed>>,
}

impl EventSourcingManager {
    pub fn new() -> Self {
        Self {
            feeds: RwLock::new(HashMap::new()),
        }
    }
    
    /// Register a new changefeed
    pub fn register_feed<F>(&self, id: String, table_filter: Option<String>, operation_filter: Option<Vec<Operation>>, callback: F) -> Result<(), String>
    where
        F: Fn(ChangeEvent) + Send + Sync + 'static,
    {
        let feed = ChangeFeed {
            id: id.clone(),
            table_filter,
            operation_filter,
            callback: Box::new(callback),
        };
        
        self.feeds.write().unwrap().insert(id, feed);
        Ok(())
    }
    
    /// Unregister a changefeed
    pub fn unregister_feed(&self, id: &str) -> bool {
        self.feeds.write().unwrap().remove(id).is_some()
    }
    
    /// Publish a change event to all interested feeds
    pub fn publish_event(&self, event: ChangeEvent) {
        let feeds = self.feeds.read().unwrap();
        for feed in feeds.values() {
            // Check table filter
            if let Some(ref table_filter) = feed.table_filter {
                if *table_filter != event.table {
                    continue;
                }
            }
            
            // Check operation filter
            if let Some(ref operation_filter) = feed.operation_filter {
                if !operation_filter.contains(&event.operation) {
                    continue;
                }
            }
            
            // Call the callback
            (feed.callback)(event.clone());
        }
    }
    
    /// Get list of active feed IDs
    pub fn list_feeds(&self) -> Vec<String> {
        self.feeds.read().unwrap().keys().cloned().collect()
    }
}

// Global event sourcing manager instance
lazy_static::lazy_static! {
    pub static ref EVENT_MANAGER: Arc<EventSourcingManager> = Arc::new(EventSourcingManager::new());
}
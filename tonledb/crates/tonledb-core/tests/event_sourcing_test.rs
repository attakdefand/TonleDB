//! Tests for event sourcing functionality

use tonledb_core::{event_sourcing::{EventSourcingManager, Operation}, event_sourcing::EVENT_MANAGER};
use std::sync::{Arc, Mutex};

#[test]
fn test_event_manager_registration() {
    let manager = EventSourcingManager::new();
    
    // Register a feed
    let result = manager.register_feed(
        "test_feed".to_string(),
        None,
        None,
        |_event| { },
    );
    assert!(result.is_ok());
    
    // Check that feed is registered
    let feeds = manager.list_feeds();
    assert!(feeds.contains(&"test_feed".to_string()));
    
    // Unregister the feed
    assert!(manager.unregister_feed("test_feed"));
    
    // Check that feed is unregistered
    let feeds = manager.list_feeds();
    assert!(!feeds.contains(&"test_feed".to_string()));
}

#[test]
fn test_event_publishing() {
    let received_events = Arc::new(Mutex::new(Vec::new()));
    let received_events_clone = received_events.clone();
    
    // Register a feed
    EVENT_MANAGER.register_feed(
        "test_feed".to_string(),
        None,
        None,
        move |event| {
            received_events_clone.lock().unwrap().push(event);
        },
    ).unwrap();
    
    // Publish an event
    let event = tonledb_core::event_sourcing::ChangeEvent {
        id: "test_id".to_string(),
        timestamp: 1234567890,
        operation: Operation::Insert,
        table: "users".to_string(),
        key: Some(b"user:1".to_vec()),
        old_value: None,
        new_value: Some(b"John Doe".to_vec()),
    };
    
    EVENT_MANAGER.publish_event(event.clone());
    
    // Check that event was received
    let events = received_events.lock().unwrap();
    assert_eq!(events.len(), 1);
    assert_eq!(events[0].id, event.id);
    assert_eq!(events[0].operation, event.operation);
    assert_eq!(events[0].table, event.table);
    
    // Unregister the feed
    EVENT_MANAGER.unregister_feed("test_feed");
}
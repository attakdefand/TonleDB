//! Example demonstrating changefeed functionality in TonleDB

use tonledb_core::{Db, Space};
use tonledb_storage::arc_inmem_with_wal;
use tonledb_core::event_sourcing::{Operation, EVENT_MANAGER};

pub fn run_changefeed_example() -> anyhow::Result<()> {
    println!("Starting changefeed example...");
    
    // Create a database instance
    let storage = arc_inmem_with_wal(None, 1000);
    let db = Db::new(storage);
    
    // Register a changefeed
    EVENT_MANAGER.register_feed(
        "example_feed".to_string(),
        Some("users".to_string()), // Only listen to "users" table
        None, // Listen to all operations
        |event| {
            println!("Changefeed event: {:?} on table {} at {:?}", 
                     event.operation, event.table, event.timestamp);
        },
    )?;
    
    // Register another feed with operation filter
    EVENT_MANAGER.register_feed(
        "insert_only_feed".to_string(),
        None, // Listen to all tables
        Some(vec![Operation::Insert]), // Only listen to inserts
        |event| {
            println!("Insert event: key={:?} in table {}", 
                     event.key.as_ref().map(|k| String::from_utf8_lossy(k)), 
                     event.table);
        },
    )?;
    
    // Perform some operations
    println!("Performing database operations...");
    
    // Insert a user
    let user_key = b"user:1";
    let user_value = b"John Doe";
    db.storage.put(&Space("users".to_string()), user_key.to_vec(), user_value.to_vec())?;
    
    // Update the user
    let user_value_updated = b"John Smith";
    db.storage.put(&Space("users".to_string()), user_key.to_vec(), user_value_updated.to_vec())?;
    
    // Insert another user
    let user_key2 = b"user:2";
    let user_value2 = b"Jane Doe";
    db.storage.put(&Space("users".to_string()), user_key2.to_vec(), user_value2.to_vec())?;
    
    // Delete a user
    db.storage.del(&Space("users".to_string()), user_key)?;
    
    // Show active feeds
    println!("Active feeds: {:?}", EVENT_MANAGER.list_feeds());
    
    // Unregister a feed
    EVENT_MANAGER.unregister_feed("insert_only_feed");
    println!("After unregistering 'insert_only_feed': {:?}", EVENT_MANAGER.list_feeds());
    
    Ok(())
}
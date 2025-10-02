//! Tests for TTL functionality in document store

use tonledb_nosql_doc;
use tonledb_storage::arc_inmem_with_wal;
use serde_json::json;

#[test]
fn test_document_insert_with_ttl() {
    let storage = arc_inmem_with_wal(None, 1000);
    
    // Insert a document with TTL
    let doc = json!({"name": "test", "value": 42});
    let result = tonledb_nosql_doc::insert_with_ttl(&*storage, "test_collection", doc, Some(3600));
    assert!(result.is_ok());
    
    let id = result.unwrap();
    
    // Retrieve the document (should not be expired)
    let retrieved = tonledb_nosql_doc::get(&*storage, "test_collection", &id, true).unwrap();
    assert!(retrieved.is_some());
    
    let doc = retrieved.unwrap();
    assert_eq!(doc["name"], "test");
    assert_eq!(doc["value"], 42);
    
    // Check that TTL field was added
    assert!(doc.get("_ttl_epoch_ms").is_some());
}

#[test]
fn test_document_without_ttl() {
    let storage = arc_inmem_with_wal(None, 1000);
    
    // Insert a document without TTL
    let doc = json!({"name": "test", "value": 42});
    let result = tonledb_nosql_doc::insert(&*storage, "test_collection", doc);
    assert!(result.is_ok());
    
    let id = result.unwrap();
    
    // Retrieve the document
    let retrieved = tonledb_nosql_doc::get(&*storage, "test_collection", &id, true).unwrap();
    assert!(retrieved.is_some());
    
    let doc = retrieved.unwrap();
    assert_eq!(doc["name"], "test");
    assert_eq!(doc["value"], 42);
    
    // Check that TTL field was not added
    assert!(doc.get("_ttl_epoch_ms").is_none());
}
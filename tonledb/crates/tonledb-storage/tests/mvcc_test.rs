//! Tests for MVCC functionality

use tonledb_core::{Space, Storage};
use tonledb_storage::InMemoryStore;

#[test]
fn test_mvcc_put_get_versioned() {
    let store = InMemoryStore::new(1000);
    let space = Space("test".to_string());
    let key = b"key".to_vec();
    let value1 = b"value1".to_vec();
    let value2 = b"value2".to_vec();
    
    // Put values with different versions
    assert!(store.put_versioned(&space, key.clone(), value1.clone(), 1).is_ok());
    assert!(store.put_versioned(&space, key.clone(), value2.clone(), 2).is_ok());
    
    // Get specific versions
    let result_v1 = store.get_versioned(&space, &key, 1).unwrap();
    assert_eq!(result_v1, Some(value1));
    
    let result_v2 = store.get_versioned(&space, &key, 2).unwrap();
    assert_eq!(result_v2, Some(value2.clone()));
    
    // Get latest version
    let result_latest = store.get(&space, &key).unwrap();
    assert_eq!(result_latest, Some(value2.clone()));
    
    // Get older version when newer doesn't exist
    let result_v3 = store.get_versioned(&space, &key, 3).unwrap();
    assert_eq!(result_v3, Some(value2)); // Should return latest
}

#[test]
fn test_mvcc_fallback_to_current() {
    let store = InMemoryStore::new(1000);
    let space = Space("test".to_string());
    let key = b"key".to_vec();
    let value = b"value".to_vec();
    
    // Put value using regular put (no version)
    assert!(store.put(&space, key.clone(), value.clone()).is_ok());
    
    // Get versioned should fallback to current value
    let result = store.get_versioned(&space, &key, 1).unwrap();
    assert_eq!(result, Some(value));
}
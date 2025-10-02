//! Tests for transaction functionality

use tonledb_core::{transaction::{Transaction, TransactionState, TXN_MANAGER}, Space, Storage};
use tonledb_storage::InMemoryStore;

#[test]
fn test_transaction_creation() {
    let txn = Transaction::new(1);
    assert_eq!(txn.id, 1);
    assert_eq!(txn.state, TransactionState::Active);
    assert!(txn.read_set.is_empty());
    assert!(txn.write_set.is_empty());
}

#[test]
fn test_transaction_manager() {
    let manager = TXN_MANAGER.clone();
    
    // Begin a transaction
    let txn_id = manager.begin().unwrap();
    assert!(txn_id > 0);
    
    // Get the transaction
    let txn = manager.get_transaction(txn_id);
    assert!(txn.is_some());
    let txn = txn.unwrap();
    assert_eq!(txn.id, txn_id);
    assert_eq!(txn.state, TransactionState::Active);
    
    // Abort the transaction
    assert!(manager.abort(txn_id).is_ok());
    
    // Get the transaction again
    let txn = manager.get_transaction(txn_id);
    assert!(txn.is_some());
    let txn = txn.unwrap();
    assert_eq!(txn.state, TransactionState::Aborted);
}

#[test]
fn test_transaction_put_get() {
    let store = InMemoryStore::new(1000);
    let space = Space("test".to_string());
    let key = b"key".to_vec();
    let value = b"value".to_vec();
    
    let mut txn = Transaction::new(1);
    
    // Put a value in the transaction
    assert!(txn.put(space.clone(), key.clone(), value.clone()).is_ok());
    
    // Get the value from the transaction
    let result = txn.get(&store, &space, &key).unwrap();
    assert_eq!(result, Some(value));
}
//! Example demonstrating transaction functionality in TonleDB

use tonledb_core::{transaction::TXN_MANAGER, Space};
use tonledb_storage::arc_inmem_with_wal;

pub fn run_transaction_example() -> anyhow::Result<()> {
    println!("Starting transaction example...");
    
    // Create a database instance
    let storage = arc_inmem_with_wal(None, 1000);
    
    // Begin a transaction
    let txn_id = TXN_MANAGER.begin()?;
    println!("Started transaction with ID: {}", txn_id);
    
    // Get the transaction
    let mut txn = TXN_MANAGER.get_transaction(txn_id)
        .ok_or_else(|| anyhow::anyhow!("Failed to get transaction"))?;
    
    // Perform some operations within the transaction
    let space = Space("test".to_string());
    let key1 = b"key1".to_vec();
    let value1 = b"value1".to_vec();
    let key2 = b"key2".to_vec();
    let value2 = b"value2".to_vec();
    
    // Put values in the transaction
    txn.put(space.clone(), key1.clone(), value1.clone())?;
    txn.put(space.clone(), key2.clone(), value2.clone())?;
    
    // Read values from the transaction
    let result1 = txn.get(&*storage, &space, &key1)?;
    let result2 = txn.get(&*storage, &space, &key2)?;
    
    println!("Value 1 in transaction: {:?}", result1);
    println!("Value 2 in transaction: {:?}", result2);
    
    // Commit the transaction
    TXN_MANAGER.commit(&*storage, txn_id)?;
    println!("Committed transaction");
    
    // Verify the values were written to storage
    let stored_value1 = storage.get(&space, &key1)?;
    let stored_value2 = storage.get(&space, &key2)?;
    
    println!("Stored value 1: {:?}", stored_value1);
    println!("Stored value 2: {:?}", stored_value2);
    
    // Start another transaction and abort it
    let txn_id2 = TXN_MANAGER.begin()?;
    println!("Started second transaction with ID: {}", txn_id2);
    
    let mut txn2 = TXN_MANAGER.get_transaction(txn_id2)
        .ok_or_else(|| anyhow::anyhow!("Failed to get transaction"))?;
    
    // Put a value in the second transaction
    let key3 = b"key3".to_vec();
    let value3 = b"value3".to_vec();
    txn2.put(space.clone(), key3.clone(), value3.clone())?;
    
    // Read the value from the transaction
    let result3 = txn2.get(&*storage, &space, &key3)?;
    println!("Value 3 in transaction: {:?}", result3);
    
    // Abort the transaction
    TXN_MANAGER.abort(txn_id2)?;
    println!("Aborted second transaction");
    
    // Verify the value was not written to storage
    let stored_value3 = storage.get(&space, &key3)?;
    println!("Stored value 3 after abort: {:?}", stored_value3);
    
    Ok(())
}
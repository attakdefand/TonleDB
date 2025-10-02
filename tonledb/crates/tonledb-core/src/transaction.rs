//! Transaction support for TonleDB

use std::collections::{HashMap, HashSet};
use std::sync::Arc;
use parking_lot::RwLock;
use crate::{DbError, Result, Space, Storage};

/// Transaction state
#[derive(Debug, Clone, PartialEq)]
pub enum TransactionState {
    Active,
    Committed,
    Aborted,
}

/// A database transaction
#[derive(Clone)]
pub struct Transaction {
    pub id: u64,
    pub state: TransactionState,
    // Track read and write operations
    pub read_set: HashSet<(Space, Vec<u8>)>,
    pub write_set: HashMap<(Space, Vec<u8>), Option<Vec<u8>>>,
    // Timestamp for MVCC
    pub timestamp: u64,
}

impl Transaction {
    pub fn new(id: u64) -> Self {
        Self {
            id,
            state: TransactionState::Active,
            read_set: HashSet::new(),
            write_set: HashMap::new(),
            timestamp: Self::current_timestamp(),
        }
    }
    
    /// Get current timestamp in milliseconds
    fn current_timestamp() -> u64 {
        std::time::SystemTime::now()
            .duration_since(std::time::UNIX_EPOCH)
            .map(|d| d.as_millis() as u64)
            .unwrap_or(0)
    }
    
    /// Read a value within the transaction
    pub fn get<S: Storage + ?Sized>(&mut self, storage: &S, space: &Space, key: &[u8]) -> Result<Option<Vec<u8>>> {
        // Check if we have a pending write
        if let Some(value) = self.write_set.get(&(space.clone(), key.to_vec())) {
            return Ok(value.clone());
        }
        
        // Add to read set for conflict detection
        self.read_set.insert((space.clone(), key.to_vec()));
        
        // Read from storage
        storage.get(space, key)
    }
    
    /// Write a value within the transaction
    pub fn put(&mut self, space: Space, key: Vec<u8>, val: Vec<u8>) -> Result<()> {
        if self.state != TransactionState::Active {
            return Err(DbError::Invalid("Transaction is not active".into()));
        }
        
        self.write_set.insert((space, key), Some(val));
        Ok(())
    }
    
    /// Delete a value within the transaction
    pub fn delete(&mut self, space: Space, key: Vec<u8>) -> Result<()> {
        if self.state != TransactionState::Active {
            return Err(DbError::Invalid("Transaction is not active".into()));
        }
        
        self.write_set.insert((space, key), None);
        Ok(())
    }
}

/// Transaction manager
pub struct TransactionManager {
    transactions: RwLock<HashMap<u64, Transaction>>,
    next_txn_id: RwLock<u64>,
}

impl TransactionManager {
    pub fn new() -> Self {
        Self {
            transactions: RwLock::new(HashMap::new()),
            next_txn_id: RwLock::new(1),
        }
    }
    
    /// Begin a new transaction
    pub fn begin(&self) -> Result<u64> {
        let mut next_id = self.next_txn_id.write();
        let txn_id = *next_id;
        *next_id += 1;
        
        let txn = Transaction::new(txn_id);
        self.transactions.write().insert(txn_id, txn);
        
        Ok(txn_id)
    }
    
    /// Commit a transaction
    pub fn commit<S: Storage + ?Sized>(&self, storage: &S, txn_id: u64) -> Result<()> {
        // First, get the transaction and validate it
        {
            let transactions = self.transactions.read();
            let txn = match transactions.get(&txn_id) {
                Some(txn) => txn,
                None => return Err(DbError::NotFound(format!("Transaction {} not found", txn_id))),
            };
            
            if txn.state != TransactionState::Active {
                return Err(DbError::Invalid("Transaction is not active".into()));
            }
            
            // Apply all writes
            for ((space, key), value) in &txn.write_set {
                match value {
                    Some(val) => {
                        storage.put(space, key.clone(), val.clone())?;
                    }
                    None => storage.del(space, key)?,
                }
            }
        }
        
        // Update transaction state
        let mut transactions = self.transactions.write();
        if let Some(mut txn) = transactions.remove(&txn_id) {
            txn.state = TransactionState::Committed;
            transactions.insert(txn_id, txn);
            Ok(())
        } else {
            Err(DbError::NotFound(format!("Transaction {} not found", txn_id)))
        }
    }
    
    /// Abort a transaction
    pub fn abort(&self, txn_id: u64) -> Result<()> {
        let mut transactions = self.transactions.write();
        if let Some(mut txn) = transactions.remove(&txn_id) {
            txn.state = TransactionState::Aborted;
            transactions.insert(txn_id, txn);
            Ok(())
        } else {
            Err(DbError::NotFound(format!("Transaction {} not found", txn_id)))
        }
    }
    
    /// Get a transaction
    pub fn get_transaction(&self, txn_id: u64) -> Option<Transaction> {
        self.transactions.read().get(&txn_id).cloned()
    }
}

// Global transaction manager instance
lazy_static::lazy_static! {
    pub static ref TXN_MANAGER: Arc<TransactionManager> = Arc::new(TransactionManager::new());
}
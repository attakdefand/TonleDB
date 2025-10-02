//! Secondary index implementation for TonleDB storage layer.

use serde::{Deserialize, Serialize};
use tonledb_core::{DbError, Result, Space, Storage};

/// Index entry structure
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct IndexEntry {
    pub key: Vec<u8>,      // Indexed value
    pub row_key: Vec<u8>,  // Reference to the actual row
}

/// Secondary index implementation
pub struct SecondaryIndex {
    pub name: String,
    pub table: String,
    pub column: String,
    pub is_unique: bool,
}

impl SecondaryIndex {
    pub fn new(name: String, table: String, column: String, is_unique: bool) -> Self {
        Self {
            name,
            table,
            column,
            is_unique,
        }
    }
    
    /// Insert an entry into the index
    pub fn insert<S: Storage + ?Sized>(
        &self,
        storage: &S,
        indexed_value: &[u8],
        row_key: &[u8],
    ) -> Result<()> {
        let index_space = Space(format!("index_{}", self.name));
        let index_key = [indexed_value, b"#", row_key].concat();
        
        // For unique indexes, check if the value already exists
        if self.is_unique {
            if let Some(_) = storage.get(&index_space, &index_key)? {
                return Err(DbError::Invalid(format!(
                    "Unique constraint violation for index {} on value {:?}",
                    self.name,
                    String::from_utf8_lossy(indexed_value)
                )));
            }
        }
        
        // Store the index entry
        let value = b"1"; // Simple marker value
        storage.put(&index_space, index_key, value.to_vec())
    }
    
    /// Delete an entry from the index
    pub fn delete<S: Storage + ?Sized>(
        &self,
        storage: &S,
        indexed_value: &[u8],
        row_key: &[u8],
    ) -> Result<()> {
        let index_space = Space(format!("index_{}", self.name));
        let index_key = [indexed_value, b"#", row_key].concat();
        storage.del(&index_space, &index_key)
    }
    
    /// Find all row keys matching a specific indexed value
    pub fn find_rows<S: Storage + ?Sized>(
        &self,
        storage: &S,
        indexed_value: &[u8],
    ) -> Result<Vec<Vec<u8>>> {
        let index_space = Space(format!("index_{}", self.name));
        let prefix = indexed_value.to_vec();
        
        let iter = storage.scan_prefix(&index_space, &prefix)?;
        let mut row_keys = Vec::new();
        
        for (key, _) in iter {
            // Extract row key from the index key (after the # separator)
            if let Some(pos) = key.iter().position(|&x| x == b'#') {
                let row_key = key[pos + 1..].to_vec();
                row_keys.push(row_key);
            }
        }
        
        Ok(row_keys)
    }
    
    /// Find all entries with values in a range
    pub fn find_range<S: Storage + ?Sized>(
        &self,
        storage: &S,
        start: Option<&[u8]>,
        end: Option<&[u8]>,
    ) -> Result<Vec<(Vec<u8>, Vec<u8>)>> {
        let index_space = Space(format!("index_{}", self.name));
        
        // For simplicity, we'll scan all entries and filter
        // In a real implementation, this would use more efficient range queries
        let iter = storage.scan_prefix(&index_space, b"")?;
        let mut results = Vec::new();
        
        for (key, _) in iter {
            // Extract indexed value and row key from the index key
            if let Some(pos) = key.iter().position(|&x| x == b'#') {
                let indexed_value = &key[..pos];
                let row_key = key[pos + 1..].to_vec();
                
                // Check if the value is within range
                let in_range = match (start, end) {
                    (Some(s), Some(e)) => indexed_value >= s && indexed_value <= e,
                    (Some(s), None) => indexed_value >= s,
                    (None, Some(e)) => indexed_value <= e,
                    (None, None) => true,
                };
                
                if in_range {
                    results.push((indexed_value.to_vec(), row_key));
                }
            }
        }
        
        Ok(results)
    }
}
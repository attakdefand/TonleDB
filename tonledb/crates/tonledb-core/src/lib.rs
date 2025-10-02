use std::collections::{BTreeMap, HashMap};
use std::sync::Arc;
use parking_lot::RwLock;
use serde::{Deserialize, Serialize};
use thiserror::Error;
use std::hash::Hash;

pub mod event_sourcing;
pub mod transaction;
pub mod security;

// ---------- Errors ----------
#[derive(Debug, Error)]
pub enum DbError {
#[error("not found: {0}")] NotFound(String),
#[error("invalid: {0}")] Invalid(String),
#[error("storage: {0}")] Storage(String),
}


pub type Result<T> = std::result::Result<T, DbError>;


// ---------- Values ----------
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub enum Value { Null, Bool(bool), I64(i64), F64(f64), Str(String), Bytes(Vec<u8>), Json(serde_json::Value) }


pub type Row = BTreeMap<String, Value>;


#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Serialize, Deserialize, Hash)]
pub struct Space(pub String);


pub trait Storage: Send + Sync {
fn get(&self, space: &Space, key: &[u8]) -> Result<Option<Vec<u8>>>;
fn put(&self, space: &Space, key: Vec<u8>, val: Vec<u8>) -> Result<()>;
fn del(&self, space: &Space, key: &[u8]) -> Result<()>;
fn scan_prefix(&self, space: &Space, prefix: &[u8]) -> Result<Box<dyn Iterator<Item=(Vec<u8>, Vec<u8>)> + Send>>;

// MVCC extensions
fn get_versioned(&self, space: &Space, key: &[u8], _version: u64) -> Result<Option<Vec<u8>>> {
    // Default implementation falls back to regular get
    self.get(space, key)
}

fn put_versioned(&self, space: &Space, key: Vec<u8>, val: Vec<u8>, _version: u64) -> Result<()> {
    // Default implementation falls back to regular put
    self.put(space, key, val)
}
}


// ---------- Catalog ----------
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Column { 
    pub name: String,
    pub data_type: DataType,
    pub constraints: Vec<ColumnConstraint>,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub enum DataType {
    Integer,
    Float,
    Text,
    Boolean,
    Json,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub enum ColumnConstraint {
    NotNull,
    Unique,
    PrimaryKey,
    ForeignKey { table: String, column: String },
    Check(String), // SQL expression
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TableSchema { 
    pub name: String, 
    pub columns: Vec<Column>, 
    pub pk: Option<String>,
    pub constraints: Vec<TableConstraint>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum TableConstraint {
    Unique { columns: Vec<String> },
    ForeignKey { columns: Vec<String>, ref_table: String, ref_columns: Vec<String> },
    Check(String), // SQL expression
}

#[derive(Default)]
pub struct Catalog {
pub tables: BTreeMap<String, TableSchema>,
pub collections: BTreeMap<String, ()>,
pub indexes: HashMap<String, IndexDef>, // key: "tbl.col"
}


#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct IndexDef { 
    pub table: String, 
    pub column: String,
    pub index_type: IndexType,
    pub is_unique: bool,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub enum IndexType {
    BTree,
    Hash,
    // Add more index types as needed
}

impl Default for IndexType {
    fn default() -> Self {
        IndexType::BTree
    }
}

// ---------- Database handle ----------
pub struct Db {
pub storage: Arc<dyn Storage>,
pub catalog: RwLock<Catalog>,
}


impl Db { 
    pub fn new(storage: Arc<dyn Storage>) -> Self { 
        Self { storage, catalog: RwLock::new(Catalog::default()) } 
    }
    
    /// Create a secondary index on a table column
    pub fn create_index(&self, table_name: &str, column_name: &str, index_type: IndexType, is_unique: bool) -> Result<()> {
        let mut catalog = self.catalog.write();
        
        // Verify table exists
        if !catalog.tables.contains_key(table_name) {
            return Err(DbError::NotFound(format!("Table {} not found", table_name)));
        }
        
        // Verify column exists
        let table = &catalog.tables[table_name];
        if !table.columns.iter().any(|c| c.name == column_name) {
            return Err(DbError::NotFound(format!("Column {} not found in table {}", column_name, table_name)));
        }
        
        // Create index definition
        let index_def = IndexDef {
            table: table_name.to_string(),
            column: column_name.to_string(),
            index_type,
            is_unique,
        };
        
        let index_key = format!("{}.{}", table_name, column_name);
        catalog.indexes.insert(index_key, index_def);
        
        Ok(())
    }
    
    /// Drop a secondary index
    pub fn drop_index(&self, table_name: &str, column_name: &str) -> Result<()> {
        let mut catalog = self.catalog.write();
        let index_key = format!("{}.{}", table_name, column_name);
        if catalog.indexes.remove(&index_key).is_none() {
            return Err(DbError::NotFound(format!("Index on {}.{} not found", table_name, column_name)));
        }
        Ok(())
    }
    
    /// Get index information
    pub fn get_index(&self, table_name: &str, column_name: &str) -> Result<Option<IndexDef>> {
        let catalog = self.catalog.read();
        let index_key = format!("{}.{}", table_name, column_name);
        Ok(catalog.indexes.get(&index_key).cloned())
    }
}

// ---------- Secondary index maintenance (in-memory map persisted in catalog space) ----------
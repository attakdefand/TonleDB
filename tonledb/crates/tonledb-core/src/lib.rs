use std::collections::{BTreeMap, HashMap};
use std::sync::Arc;
use parking_lot::RwLock;
use serde::{Deserialize, Serialize};
use thiserror::Error;
use std::hash::Hash;


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
}


// ---------- Catalog ----------
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Column { pub name: String }


#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TableSchema { pub name: String, pub columns: Vec<Column>, pub pk: Option<String> }


#[derive(Default)]
pub struct Catalog {
pub tables: BTreeMap<String, TableSchema>,
pub collections: BTreeMap<String, ()>,
pub indexes: HashMap<String, IndexDef>, // key: "tbl.col"
}


#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct IndexDef { pub table: String, pub column: String }


// ---------- Database handle ----------
pub struct Db {
pub storage: Arc<dyn Storage>,
pub catalog: RwLock<Catalog>,
}


impl Db { pub fn new(storage: Arc<dyn Storage>) -> Self { Self { storage, catalog: RwLock::new(Catalog::default()) } } }


// ---------- Secondary index maintenance (in-memory map persisted in catalog space) ----------
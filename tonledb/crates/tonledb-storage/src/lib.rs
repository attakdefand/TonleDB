use std::collections::BTreeMap;
use std::sync::Arc;
use parking_lot::RwLock;
use clru::CLruCache;
use tonledb_core::{DbError, Result, Space, Storage};

pub mod index;

/// In-memory store with best-effort WAL and an LRU around get/put keys for hot paths.
pub struct InMemoryStore {
inner: RwLock<BTreeMap<(Space, Vec<u8>), Vec<u8>>>,
wal: Option<RwLock<tonledb_wal::Wal>>,
cache: RwLock<CLruCache<(Space, Vec<u8>), Vec<u8>>>,
}

impl InMemoryStore {
pub fn new(cap: usize) -> Self { 
    Self { 
        inner: RwLock::new(BTreeMap::new()), 
        wal: None, 
        cache: RwLock::new(CLruCache::new(cap.try_into().unwrap())),
    } 
}

pub fn with_wal(path: &str, cap: usize) -> anyhow::Result<Self> {
let mut wal = tonledb_wal::Wal::open(path)?;
let mut m = BTreeMap::new();
for rec in wal.replay()? { // record: space\tkey\tval
let mut it = rec.splitn(3, |b| *b==b'\t');
let sp = it.next().unwrap(); let k = it.next().unwrap(); let v = it.next().unwrap();
m.insert((Space(String::from_utf8_lossy(sp).to_string()), k.to_vec()), v.to_vec());
}
Ok(Self { 
    inner: RwLock::new(m), 
    wal: Some(RwLock::new(wal)), 
    cache: RwLock::new(CLruCache::new(cap.try_into().unwrap())),
})
}

}

impl Storage for InMemoryStore {
fn get(&self, space: &Space, key: &[u8]) -> Result<Option<Vec<u8>>> {
if let Some(v) = self.cache.write().get(&(space.clone(), key.to_vec())).cloned() { return Ok(Some(v)); }
let val = self.inner.read().get(&(space.clone(), key.to_vec())).cloned();
if let Some(v) = val.clone() { self.cache.write().put((space.clone(), key.to_vec()), v.clone()); }
Ok(val)
}

fn put(&self, space: &Space, key: Vec<u8>, val: Vec<u8>) -> Result<()> {
if let Some(w) = &self.wal { let rec = [space.0.as_bytes(), b"\t", &key, b"\t", &val].concat(); w.write().append(&rec).map_err(|e| DbError::Storage(e.to_string()))?; }
self.cache.write().put((space.clone(), key.clone()), val.clone());
self.inner.write().insert((space.clone(), key.clone()), val.clone()); Ok(())
}

fn del(&self, space: &Space, key: &[u8]) -> Result<()> { 
    self.cache.write().pop(&(space.clone(), key.to_vec())); 
    self.inner.write().remove(&(space.clone(), key.to_vec())); 
    Ok(()) 
}

fn scan_prefix(&self, space: &Space, prefix: &[u8]) -> Result<Box<dyn Iterator<Item=(Vec<u8>, Vec<u8>)> + Send>> {
let space = space.clone(); let p = prefix.to_vec();
let v: Vec<(Vec<u8>, Vec<u8>)> = self.inner.read().iter().filter(|((s,k),_)| *s==space && k.starts_with(&p)).map(|((_,k),v)|(k.clone(),v.clone())).collect();
Ok(Box::new(v.into_iter()))
}
}

pub fn arc_inmem_with_wal(path: Option<&str>, cache_cap: usize) -> Arc<dyn tonledb_core::Storage> {
match path { Some(p) => Arc::new(InMemoryStore::with_wal(p, cache_cap).unwrap()), None => Arc::new(InMemoryStore::new(cache_cap)) }
}
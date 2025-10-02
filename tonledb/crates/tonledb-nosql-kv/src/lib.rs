//! NoSQL Key/Value API over TonleDB storage.
//!
//! Keys live in the dedicated `Space("kv")`. Values are arbitrary bytes.
//! This module provides simple CRUD and convenience helpers (exists, list,
//! prefix scan, atomic-style set-if-absent).

use tonledb_core::{Result, Space, Storage};

const KV_SPACE: &str = "kv";

/// Get a value by key. Returns `Ok(Some(bytes))` if present.
pub fn get<S: Storage + ?Sized>(storage: &S, key: &[u8]) -> Result<Option<Vec<u8>>> {
    storage.get(&Space(KV_SPACE.into()), key)
}

/// Put (set) a value by key (overwrites any existing value).
pub fn put<S: Storage + ?Sized>(storage: &S, key: Vec<u8>, val: Vec<u8>) -> Result<()> {
    storage.put(&Space(KV_SPACE.into()), key, val)
}

/// Delete a key (no-op if absent).
pub fn del<S: Storage + ?Sized>(storage: &S, key: &[u8]) -> Result<()> {
    storage.del(&Space(KV_SPACE.into()), key)
}

/// Return `true` if the key exists.
pub fn exists<S: Storage + ?Sized>(storage: &S, key: &[u8]) -> Result<bool> {
    Ok(storage.get(&Space(KV_SPACE.into()), key)?.is_some())
}

/// Set a value only if the key does not already exist. Returns `true` if set.
pub fn set_if_absent<S: Storage + ?Sized>(storage: &S, key: Vec<u8>, val: Vec<u8>) -> Result<bool> {
    let space = Space(KV_SPACE.into());
    if storage.get(&space, &key)?.is_some() {
        return Ok(false);
    }
    storage.put(&space, key, val)?;
    Ok(true)
}

/// List all keys having the given prefix. Returns (key, value) pairs.
/// NOTE: For large datasets you may want paging – this returns all matches.
pub fn scan_prefix<S: Storage + ?Sized>(storage: &S, prefix: &[u8]) -> Result<Vec<(Vec<u8>, Vec<u8>)>> {
    let it = storage.scan_prefix(&Space(KV_SPACE.into()), prefix)?;
    Ok(it.collect())
}

/// Convenience helper: list just keys that match a prefix.
pub fn keys_with_prefix<S: Storage + ?Sized>(storage: &S, prefix: &[u8]) -> Result<Vec<Vec<u8>>> {
    let it = storage.scan_prefix(&Space(KV_SPACE.into()), prefix)?;
    Ok(it.map(|(k, _)| k).collect())
}

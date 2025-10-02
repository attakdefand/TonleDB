//! Document Collections over TonleDB storage.
//!
//! Layout:
//! - Catalog entries under `Space("catalog")`: key = `col/<name>`
//! - Documents under `Space("data")`: key = `doc/<collection>/<id>` -> JSON bytes
//!
//! The API below provides basic CRUD, listing, prefix scans, and simple
//! filter queries (client-side predicate). TTL is supported by convention:
//! if a document contains a numeric field `_ttl_epoch_ms`, callers can
//! decide to ignore expired docs (option here).

use tonledb_core::{Result, Space, Storage};
use serde::{Deserialize, Serialize};
use serde_json::Value as Json;

const CATALOG_SPACE: &str = "catalog";
const DATA_SPACE: &str = "data";

/// Create a collection entry in the catalog (idempotent).
pub fn create_collection<S: Storage + ?Sized>(storage: &S, name: &str) -> Result<()> {
    let key = format!("col/{}", name).into_bytes();
    // Minimal metadata for now; can evolve to schema/validation rules.
    let meta = serde_json::json!({ "name": name });
    storage.put(&Space(CATALOG_SPACE.into()), key, serde_json::to_vec(&meta).unwrap())
}

/// Insert a new document and return its generated id (nanoid).
/// If ttl_seconds is provided, the document will expire after that many seconds.
pub fn insert_with_ttl<S: Storage + ?Sized>(
    storage: &S, 
    collection: &str, 
    mut doc: Json, 
    ttl_seconds: Option<u64>
) -> Result<String> {
    // ensure an id field (not required but useful)
    let id = nanoid::nanoid!();
    if let Some(obj) = doc.as_object_mut() {
        obj.entry("_id".to_string()).or_insert(Json::String(id.clone()));
        
        // Add TTL if specified
        if let Some(ttl) = ttl_seconds {
            let now = std::time::SystemTime::now()
                .duration_since(std::time::UNIX_EPOCH)
                .map(|d| d.as_millis() as i64)
                .unwrap_or(0);
            let ttl_epoch_ms = now + (ttl as i64 * 1000);
            obj.insert("_ttl_epoch_ms".to_string(), Json::Number(ttl_epoch_ms.into()));
        }
    }
    let key = doc_key(collection, &id);
    storage.put(&Space(DATA_SPACE.into()), key, serde_json::to_vec(&doc).unwrap())?;
    Ok(id)
}

/// Get a document by id. If `ignore_expired` is true, documents with a
/// numeric `_ttl_epoch_ms` in the past are returned as `Ok(None)`.
pub fn get<S: Storage + ?Sized>(storage: &S, collection: &str, id: &str, ignore_expired: bool) -> Result<Option<Json>> {
    match storage.get(&Space(DATA_SPACE.into()), &doc_key(collection, id))? {
        Some(bytes) => {
            let doc: Json = serde_json::from_slice(&bytes).unwrap_or(Json::Null);
            if ignore_expired && is_expired(&doc) { return Ok(None); }
            Ok(Some(doc))
        }
        None => Ok(None),
    }
}

/// Replace (overwrite) a document by id. Returns `true` if replaced, `false` if missing.
pub fn replace<S: Storage + ?Sized>(storage: &S, collection: &str, id: &str, mut doc: Json) -> Result<bool> {
    let key = doc_key(collection, id);
    let space = Space(DATA_SPACE.into());
    if storage.get(&space, &key)?.is_none() {
        return Ok(false);
    }
    if let Some(obj) = doc.as_object_mut() {
        obj.insert("_id".to_string(), Json::String(id.to_string()));
    }
    storage.put(&space, key, serde_json::to_vec(&doc).unwrap())?;
    Ok(true)
}

/// Update by merging fields with the existing JSON object. Creates document if absent when `upsert=true`.
/// Returns `true` if existing doc updated, `false` if created (with upsert).
pub fn update_merge<S: Storage + ?Sized>(
    storage: &S,
    collection: &str,
    id: &str,
    patch: Json,
    upsert: bool,
) -> Result<bool> {
    let key = doc_key(collection, id);
    let space = Space(DATA_SPACE.into());

    let base = match storage.get(&space, &key)? {
        Some(bytes) => serde_json::from_slice::<Json>(&bytes).unwrap_or(Json::Null),
        None => {
            if !upsert { return Ok(false); }
            Json::Object(Default::default())
        }
    };
    let merged = merge_json(base, patch);

    let mut merged = merged;
    if let Some(obj) = merged.as_object_mut() {
        obj.insert("_id".into(), Json::String(id.to_string()));
    }
    storage.put(&space, key, serde_json::to_vec(&merged).unwrap())?;
    Ok(true)
}

/// Delete a document. Returns `true` if existed.
pub fn delete<S: Storage + ?Sized>(storage: &S, collection: &str, id: &str) -> Result<bool> {
    let key = doc_key(collection, id);
    let space = Space(DATA_SPACE.into());
    let existed = storage.get(&space, &key)?.is_some();
    storage.del(&space, &key)?;
    Ok(existed)
}

/// List documents in a collection. If `ignore_expired` is true, skip docs with TTL in the past.
/// WARNING: Returns the entire collection; for large sets implement paging.
pub fn list_all<S: Storage + ?Sized>(storage: &S, collection: &str, ignore_expired: bool) -> Result<Vec<Json>> {
    let prefix = format!("doc/{}/", collection).into_bytes();
    let it = storage.scan_prefix(&Space(DATA_SPACE.into()), &prefix)?;
    let mut out = Vec::new();
    for (_k, v) in it {
        let doc: Json = serde_json::from_slice(&v).unwrap_or(Json::Null);
        if ignore_expired && is_expired(&doc) { continue; }
        out.push(doc);
    }
    Ok(out)
}

/// Find all documents where `field == value` (simple equality filter).
/// Client-side filter for MVP; later replace with indexed field lookups.
pub fn find_eq<S: Storage + ?Sized>(storage: &S, collection: &str, field: &str, value: &Json, ignore_expired: bool) -> Result<Vec<Json>> {
    let prefix = format!("doc/{}/", collection).into_bytes();
    let it = storage.scan_prefix(&Space(DATA_SPACE.into()), &prefix)?;
    let mut out = Vec::new();
    for (_k, v) in it {
        let doc: Json = serde_json::from_slice(&v).unwrap_or(Json::Null);
        if ignore_expired && is_expired(&doc) { continue; }
        if json_field_eq(&doc, field, value) {
            out.push(doc);
        }
    }
    Ok(out)
}

/// Find with a custom predicate closure.
/// Example:
/// ```ignore
/// let res = find_where(&*db.storage, "todos", |doc| doc["done"] == true.into(), true)?;
/// ```
pub fn find_where<S, F>(storage: &S, collection: &str, mut pred: F, ignore_expired: bool) -> Result<Vec<Json>>
where
    S: Storage + ?Sized,
    F: FnMut(&Json) -> bool,
{
    let prefix = format!("doc/{}/", collection).into_bytes();
    let it = storage.scan_prefix(&Space(DATA_SPACE.into()), &prefix)?;
    let mut out = Vec::new();
    for (_k, v) in it {
        let doc: Json = serde_json::from_slice(&v).unwrap_or(Json::Null);
        if ignore_expired && is_expired(&doc) { continue; }
        if pred(&doc) { out.push(doc); }
    }
    Ok(out)
}

// ---------- helpers ----------

fn doc_key(collection: &str, id: &str) -> Vec<u8> {
    format!("doc/{}/{}", collection, id).into_bytes()
}

fn merge_json(mut base: Json, patch: Json) -> Json {
    match (base, patch) {
        (Json::Object(mut a), Json::Object(b)) => {
            for (k, v) in b {
                a.insert(k, v);
            }
            Json::Object(a)
        }
        (_, p) => p, // if base isn't an object, replace
    }
}

fn json_field_eq(doc: &Json, field: &str, needle: &Json) -> bool {
    match doc {
        Json::Object(map) => map.get(field).map(|v| v == needle).unwrap_or(false),
        _ => false,
    }
}

/// TTL convention: if document contains numeric `_ttl_epoch_ms` and now >= TTL, it is expired.
fn is_expired(doc: &Json) -> bool {
    let now_ms = std::time::SystemTime::now()
        .duration_since(std::time::UNIX_EPOCH)
        .map(|d| d.as_millis() as i128)
        .unwrap_or(i128::MAX);

    doc.get("_ttl_epoch_ms")
        .and_then(|v| v.as_i64().map(|x| x as i128))
        .map(|ttl| now_ms >= ttl)
        .unwrap_or(false)
}

/// Insert a new document with optional TTL and return its generated id (nanoid).
pub fn insert<S: Storage + ?Sized>(storage: &S, collection: &str, doc: Json) -> Result<String> {
    insert_with_ttl(storage, collection, doc, None)
}

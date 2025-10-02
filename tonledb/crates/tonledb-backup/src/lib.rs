//! Backup/restore + logical exports for TonleDB.
//!
//! Features:
//! - Raw snapshot/restore of keyspaces ("catalog", "data", "kv") to JSONL
//!   with optional Zstd compression.
//! - Logical SQL dump (CREATE TABLE + INSERT VALUES).
//! - JSONL export for a document collection.

use anyhow::Result;
use base64::{engine::general_purpose::STANDARD as B64, Engine as _};
use serde::{Deserialize, Serialize};
use serde_json::json;
use std::fs::File;
use std::io::{BufRead, BufReader, BufWriter, Write};
use tonledb_core::{Db, DbError, Space, Storage};

/// A raw record in the snapshot (JSONL)
#[derive(Serialize, Deserialize)]
struct SnapshotRec {
    space: String,
    key_b64: String,
    val_b64: String,
}

/// Take a raw snapshot of all known spaces into a JSONL (optionally .zst) file.
///
/// Known spaces in current engine:
/// - "catalog"  (schemas, indexes, collections)
/// - "data"     (table rows, documents)
/// - "kv"       (key-value store)
///
/// Each line is `{"space": "...", "key_b64": "...", "val_b64": "..."}`.
pub fn snapshot<S: Storage + ?Sized>(storage: &S, out_path: &str, compress: bool) -> Result<()> {
    let file = File::create(out_path)?;
    if compress {
        let mut w = zstd::Encoder::new(file, 7)?.auto_finish();
        write_snapshot(storage, &mut w)
    } else {
        let mut w = BufWriter::new(file);
        write_snapshot(storage, &mut w)
    }
}

fn write_snapshot<W: Write, S: Storage + ?Sized>(storage: &S, mut w: W) -> Result<()> {
    for space in ["catalog", "data", "kv"] {
        let space = Space(space.to_string());
        // Empty prefix => iterate everything in the space
        let iter = storage
            .scan_prefix(&space, b"")
            .map_err(|e| anyhow::anyhow!("scan_prefix: {:?}", e))?;
        for (k, v) in iter {
            let rec = SnapshotRec {
                space: space.0.clone(),
                key_b64: B64.encode(&k),
                val_b64: B64.encode(&v),
            };
            let line = serde_json::to_string(&rec)?;
            writeln!(w, "{line}")?;
        }
    }
    Ok(())
}

/// Restore a snapshot previously created by `snapshot()`.
/// Existing keys are overwritten.
pub fn restore<S: Storage + ?Sized>(storage: &S, path: &str, compressed: bool) -> Result<()> {
    let file = File::open(path)?;
    if compressed {
        let mut rdr = zstd::Decoder::new(file)?;
        let mut buf = String::new();
        let mut br = BufReader::new(&mut rdr);
        while {
            buf.clear();
            br.read_line(&mut buf)? > 0
        } {
            if buf.trim().is_empty() {
                continue;
            }
            let rec: SnapshotRec = serde_json::from_str(buf.trim())?;
            let sp = Space(rec.space);
            let key = B64.decode(rec.key_b64)?;
            let val = B64.decode(rec.val_b64)?;
            storage
                .put(&sp, key, val)
                .map_err(|e| anyhow::anyhow!("restore put: {:?}", e))?;
        }
    } else {
        let f = File::open(path)?;
        let br = BufReader::new(f);
        for line in br.lines() {
            let l = line?;
            if l.trim().is_empty() {
                continue;
            }
            let rec: SnapshotRec = serde_json::from_str(&l)?;
            let sp = Space(rec.space);
            let key = B64.decode(rec.key_b64)?;
            let val = B64.decode(rec.val_b64)?;
            storage
                .put(&sp, key, val)
                .map_err(|e| anyhow::anyhow!("restore put: {:?}", e))?;
        }
    }
    Ok(())
}

/// Dump all SQL tables to a logical `.sql` file.
///
/// Emits:
/// - `CREATE TABLE <name>(col1 TEXT, col2 TEXT, ...)`  (types are opaque in MVP)
/// - `INSERT INTO <name> VALUES (...), (...), ...;`
///
/// Rows are read by scanning `data` space with prefix `tbl/<name>/`.
pub fn dump_sql(db: &Db, out_path: &str) -> Result<()> {
    let mut w = BufWriter::new(File::create(out_path)?);

    let cat = db.catalog.read();

    // 1) CREATE TABLE statements
    for (_, tbl) in cat.tables.iter() {
        let cols = tbl
            .columns
            .iter()
            .map(|c| format!("{} TEXT", ident(&c.name)))
            .collect::<Vec<_>>()
            .join(", ");
        writeln!(
            w,
            "CREATE TABLE {}({});",
            ident(&tbl.name),
            cols
        )?;
    }

    // 2) INSERT statements
    for (_, tbl) in cat.tables.iter() {
        let prefix = format!("tbl/{}/", tbl.name).into_bytes();
        let iter = db
            .storage
            .scan_prefix(&Space("data".into()), &prefix)
            .map_err(|e| anyhow::anyhow!("scan rows: {:?}", e))?;

        // Accumulate values and flush in manageable batches
        let mut batch = Vec::<String>::new();
        for (_k, v) in iter {
            let obj: serde_json::Value =
                serde_json::from_slice(&v).map_err(str_err("row json decode"))?;
            let row_sql = json_row_to_insert_values(&obj, &tbl.columns)?;
            batch.push(row_sql);
            if batch.len() >= 1000 {
                flush_insert(&mut w, &tbl.name, &batch)?;
                batch.clear();
            }
        }
        if !batch.is_empty() {
            flush_insert(&mut w, &tbl.name, &batch)?;
        }
    }

    Ok(())
}

/// Export a document collection as JSON Lines (`.jsonl`).
/// Scans `data` space with prefix `doc/<collection>/`.
pub fn export_collection_jsonl(db: &Db, collection: &str, out_path: &str) -> Result<()> {
    let mut w = BufWriter::new(File::create(out_path)?);
    let prefix = format!("doc/{}/", collection).into_bytes();
    let iter = db
        .storage
        .scan_prefix(&Space("data".into()), &prefix)
        .map_err(|e| anyhow::anyhow!("scan docs: {:?}", e))?;
    for (_k, v) in iter {
        let obj: serde_json::Value =
            serde_json::from_slice(&v).map_err(str_err("doc json decode"))?;
        writeln!(w, "{}", serde_json::to_string(&obj)?)?;
    }
    Ok(())
}

// ---------- helpers ----------

fn ident(name: &str) -> String {
    // Minimal quote for safety
    if name.chars().all(|c| c.is_ascii_alphanumeric() || c == '_') {
        name.to_string()
    } else {
        format!("\"{}\"", name.replace('"', "\"\""))
    }
}

fn sql_quote(s: &str) -> String {
    format!("'{}'", s.replace('\'', "''"))
}

fn json_val_to_sql(v: &serde_json::Value) -> String {
    match v {
        serde_json::Value::Null => "NULL".into(),
        serde_json::Value::Bool(b) => if *b { "TRUE" } else { "FALSE" }.into(),
        serde_json::Value::Number(n) => n.to_string(),
        serde_json::Value::String(s) => sql_quote(s),
        _ => sql_quote(&v.to_string()), // arrays/objects → JSON text
    }
}

fn json_row_to_insert_values(
    row: &serde_json::Value,
    cols: &[tonledb_core::Column],
) -> Result<String> {
    let obj = row
        .as_object()
        .ok_or_else(|| anyhow::anyhow!("row not object"))?;
    let mut parts = Vec::with_capacity(cols.len());
    for c in cols {
        let v = obj.get(&c.name).cloned().unwrap_or(serde_json::Value::Null);
        parts.push(json_val_to_sql(&v));
    }
    Ok(format!("({})", parts.join(", ")))
}

fn flush_insert<W: Write>(w: &mut W, table: &str, rows: &[String]) -> Result<()> {
    writeln!(
        w,
        "INSERT INTO {} VALUES\n  {};\n",
        ident(table),
        rows.join(",\n  ")
    )?;
    Ok(())
}

fn str_err(ctx: &'static str) -> impl FnOnce(serde_json::Error) -> anyhow::Error {
    move |e| anyhow::anyhow!("{}: {}", ctx, e)
}

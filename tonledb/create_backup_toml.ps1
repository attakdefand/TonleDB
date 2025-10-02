Set-Content -Path "crates\tonledb-backup\Cargo.toml" -Value @"
[package]
name = "tonledb-backup"
version = "0.1.0"
edition = "2021"

[dependencies]
tonledb-core = { path = "../tonledb-core" }
tonledb-storage = { path = "../tonledb-storage" }
tonledb-wal = { path = "../tonledb-wal" }
serde = { version = "1", features = ["derive"] }
serde_json = "1"
base64 = "0.22"
anyhow = "1"
zstd = "0.13"
sha2 = "0.10"
hmac = "0.12"
hex = "0.4"
"@
Set-Content -Path "crates\tonledb-wire-pg\Cargo.toml" -Value @"
[package]
name = "tonledb-wire-pg"
version = "0.1.0"
edition = "2021"

[dependencies]
tonledb-core = { path = "../tonledb-core" }
tonledb-sql = { path = "../tonledb-sql" }
tokio = { version = "1", features = ["full"] }
anyhow = "1.0"
serde_json = "1.0"
"@
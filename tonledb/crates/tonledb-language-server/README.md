# TonleDB Language Server

This crate provides a Language Server Protocol (LSP) implementation for TonleDB, enabling rich IDE features for TonleDB SQL.

## Features

- Syntax highlighting
- Code completion
- Hover information
- Error diagnostics
- Connection management to TonleDB instances

## Installation

Add this to your `Cargo.toml`:

```toml
[dependencies]
tonledb-language-server = { path = "../tonledb-language-server" }
```

## Usage

To run the language server:

```bash
cargo run -p tonledb-language-server
```

## Integration with VS Code

The language server is designed to work with the TonleDB VS Code extension, which provides:

- Syntax highlighting for `.tdb` and `.tonlesql` files
- IntelliSense for TonleDB SQL
- Connection management to TonleDB instances

## Connection Management

The language server includes a connection manager that allows connecting to TonleDB instances:

```rust
use tonledb_language_server::{ConnectionManager, ConnectionConfig};

let connection_manager = ConnectionManager::new();
let config = ConnectionConfig {
    host: "localhost".to_string(),
    port: 5432,
    database: "tonledb".to_string(),
    username: "user".to_string(),
    password: None,
};

connection_manager.add_connection("default".to_string(), config).await?;
```
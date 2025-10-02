# TonleDB - Enhanced Database System

TonleDB is a hybrid database that supports both SQL and NoSQL (key-value and document) interfaces over a shared storage and transaction engine.

## Recent Enhancements

This version of TonleDB includes significant enhancements across all roadmap phases:

### Near-term Features (M1-M3)
- **Secondary Indexes**: B-Tree and Hash indexes for improved query performance
- **TTL for Documents**: Automatic expiration of documents after a specified time
- **Richer Query Engine**: Support for complex WHERE clauses, ORDER BY, and LIMIT operations
- **MVCC**: Multi-Version Concurrency Control for better concurrent access
- **Event Sourcing/Changefeeds**: Real-time event system for database changes

### Mid-term Features (M4-M5)
- **Full ACID Transactions**: Complete transaction manager with begin/commit/abort operations
- **Constraint Validation**: Support for NOT NULL, UNIQUE, PRIMARY KEY, FOREIGN KEY, and CHECK constraints

### Longer-term Features (M6-M7)
- **Arrow/Parquet Support**: Industry-standard columnar formats for analytics workloads
- **PostgreSQL Wire Protocol Compatibility**: Integration with PostgreSQL tools and clients
- **Row-Level Security**: Fine-grained access control at the row level
- **Point-In-Time Recovery (PITR)**: Disaster recovery with precise time-based restoration

## Architecture

TonleDB is built with a modular architecture:

- **tonledb-core**: Core data structures and interfaces
- **tonledb-storage**: Storage engine with WAL persistence
- **tonledb-sql**: SQL query parser and execution engine
- **tonledb-nosql-kv**: Key-value NoSQL interface
- **tonledb-nosql-doc**: Document NoSQL interface
- **tonledb-network**: HTTP API and network interface
- **tonledb-wal**: Write-Ahead Logging implementation
- **tonledb-metrics**: Observability and monitoring
- **tonledb-cli**: Command-line interface
- **tonledb-backup**: Backup and recovery functionality
- **tonledb-arrow**: Arrow and Parquet support
- **tonledb-wire-pg**: PostgreSQL wire protocol compatibility

## Getting Started

To run TonleDB:

```bash
cd tonledb
cargo run -p tonledb-network
```

To run examples:

```bash
cd tonledb
cargo run -p tonledb-examples
```

## Testing

To run tests:

```bash
cd tonledb
cargo test
```

## Building Optimized Binaries

To build TonleDB with full optimizations enabled (removes the "unoptimized" message):

```bash
# Build the network server (main database server)
cd tonledb
cargo build -p tonledb-network --release

# Build the CLI tool
cargo build -p tonledb-cli --release

# Build examples
cargo build -p tonledb-examples --release

# Or build all components at once
cargo build --release --workspace
```

For convenience, you can also use the provided build scripts:
- On Linux/macOS: `./build-optimized.sh`
- On Windows: `build-optimized.bat`

The optimized binaries will be located in `target/release/` directory.

Optimization features enabled:
- Full LTO (Link Time Optimization)
- Single codegen unit for maximum optimization
- Abort-on-panic for smaller binary size
- All debug assertions disabled
- Overflow checks disabled

## Running Optimized Binaries

To run the optimized server:
```bash
./target/release/tonledb-network
```

To run the optimized CLI:
```bash
./target/release/tonledb-cli --help
```

## Cross-Platform Support

TonleDB is built with Rust, making it cross-platform compatible. The same codebase works on:

- **Linux** (All major distributions)
- **Windows** (Windows 10, Windows 11, Windows Server)
- **macOS** (macOS 10.15+, Apple Silicon and Intel)

### Windows Usage

On Windows, use PowerShell or Command Prompt with backslashes:

```powershell
# Build optimized binaries
cargo build --release --workspace

# Run optimized CLI
.\target\release\tonledb-cli.exe --help

# Initialize database
.\target\release\tonledb-cli.exe init --wal mydb.wal
```

### macOS Usage

On macOS, the commands are the same as Linux:

```bash
# Build optimized binaries
cargo build --release --workspace

# Run optimized CLI
./target/release/tonledb-cli --help

# Initialize database
./target/release/tonledb-cli init --wal mydb.wal
```

## License

This project is licensed under the MIT License.

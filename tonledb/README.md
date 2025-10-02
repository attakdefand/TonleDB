# TonleDB

A hybrid database system built with Rust, supporting both SQL and NoSQL interfaces over a shared storage and transaction kernel.

## Features

This MVP supports:
- SQL: CREATE TABLE, INSERT, SELECT (WHERE =), CREATE INDEX (single-col equality)
- NoSQL: KV CRUD, Document insert/get
- In-memory storage with Write-Ahead Logging (WAL) for persistence
- LRU cache for performance optimization
- HTTP API for easy access
- Built-in metrics and observability

## Quick Start

### Prerequisites
- Rust and Cargo (latest stable version)

### Building Core Components (Recommended)
```bash
cargo build -p tonledb-core -p tonledb-storage -p tonledb-sql -p tonledb-nosql-kv -p tonledb-nosql-doc -p tonledb-metrics
```

### Running the Database
```bash
RUST_LOG=info cargo run -p tonledb-network
```

Note: This requires CMake and NASM to be installed for the TLS dependencies. See [RUNNING.md](../official-docs/RUNNING.md) for detailed installation instructions.

## Project Structure

TonleDB is organized as a Rust workspace with multiple crates:

- **tonledb-core**: Core database types and traits
- **tonledb-storage**: Storage engine with WAL persistence
- **tonledb-sql**: SQL parser and executor
- **tonledb-nosql-kv**: Key-value NoSQL interface
- **tonledb-nosql-doc**: Document NoSQL interface
- **tonledb-network**: HTTP API server
- **tonledb-metrics**: Metrics and observability
- **tonledb-examples**: Examples demonstrating Rust concurrency patterns

## Installation via Package Managers

Pre-built packages are available for popular Linux distributions:

### Debian/Ubuntu
```bash
# Download the .deb package from releases
sudo dpkg -i tonledb_0.1.0_amd64.deb
sudo apt-get install -f  # Fix dependencies if needed
```

### CentOS/RHEL/Fedora
```bash
# Download the .rpm package from releases
# For CentOS/RHEL
sudo yum install tonledb-0.1.0-1.x86_64.rpm

# For Fedora
sudo dnf install tonledb-0.1.0-1.x86_64.rpm
```

See [packaging/INSTALL.md](packaging/INSTALL.md) for detailed installation instructions.

## Documentation

For detailed instructions on building, running, and using TonleDB, see:
- [RUNNING.md](../official-docs/RUNNING.md) - Complete guide to running the project
- [Architecture Documentation](../official-docs/md.md) - System architecture documentation
- [packaging/README.md](packaging/README.md) - Information about packaging for Linux distributions
- [packaging/INSTALL.md](packaging/INSTALL.md) - User installation instructions

## API Usage

Once the database is running, you can interact with it via HTTP:

```bash
# Health check
curl http://localhost:8080/health

# SQL query
curl -X POST http://localhost:8080/sql \
  -H "Content-Type: application/json" \
  -d '{"sql": "SELECT * FROM users"}'

# Key-Value operations
curl http://localhost:8080/kv/mykey
curl -X POST http://localhost:8080/kv/mykey -d "myvalue"
curl -X DELETE http://localhost:8080/kv/mykey

# Document operations
curl -X POST http://localhost:8080/doc/users \
  -H "Content-Type: application/json" \
  -d '{"name": "John", "age": 30}'
```

## Development

To run tests:
```bash
cargo test --workspace
```

For more detailed development instructions, see [RUNNING.md](../official-docs/RUNNING.md).

## Packaging

This repository includes packaging files for creating distribution packages:

- [packaging/](packaging/) - Contains files for Debian and RPM packaging
- [packaging/README.md](packaging/README.md) - Detailed packaging instructions
- [.github/workflows/package.yml](.github/workflows/package.yml) - GitHub Actions for automated packaging

To build packages locally:
```bash
cd packaging
make
```
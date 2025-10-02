# TonleDB

A hybrid database system built with Rust, supporting both SQL and NoSQL interfaces over a shared storage and transaction kernel.

## Overview

TonleDB is a modern database system that combines the flexibility of NoSQL with the familiarity of SQL. It's designed with the following principles:

- **Hybrid Architecture**: Unified storage engine supporting both SQL and NoSQL operations
- **Performance**: Built with Rust for memory safety and performance
- **Modularity**: Organized as a multi-crate workspace for extensibility
- **Concurrency**: Demonstrates various Rust concurrency patterns in real-world applications

## Project Structure

```
TonleDB/
├── tonledb/                    # Main database implementation
│   ├── crates/
│   │   ├── tonledb-core/       # Core types and traits
│   │   ├── tonledb-storage/    # Storage engine with WAL persistence
│   │   ├── tonledb-sql/        # SQL parser and executor
│   │   ├── tonledb-nosql-kv/   # Key-value NoSQL interface
│   │   ├── tonledb-nosql-doc/  # Document NoSQL interface
│   │   ├── tonledb-network/    # HTTP API server
│   │   ├── tonledb-metrics/    # Metrics and observability
│   │   └── tonledb-examples/   # Examples of Rust concurrency patterns
│   ├── .github/
│   │   └── workflows/          # CI/CD workflows including packaging
│   └── tonledb.toml            # Configuration file
├── packaging/                  # Packaging files for Linux distributions
│   ├── debian/                 # Debian/Ubuntu packaging files
│   ├── rpm/                    # RPM packaging files
│   ├── Makefile                # Build automation
│   ├── build-packages.sh       # Packaging script
│   ├── install.sh              # Installation script
│   ├── install.html            # Installation instructions (HTML)
│   ├── INSTALL.md              # User installation instructions
│   └── README.md               # Packaging documentation
├── repo/                       # Package repository files
│   ├── apt/                    # APT repository configuration
│   ├── yum/                    # YUM repository configuration
│   ├── tonledb.list            # APT repository configuration file
│   ├── tonledb.repo            # YUM repository configuration file
│   └── README.md               # Repository documentation
└── official-docs/              # Documentation
    ├── RUNNING.md              # Instructions for running the project
    ├── PACKAGING.md            # Manual packaging instructions
    ├── PUBLISHING.md           # Instructions for publishing packages
    └── md.md                   # Architecture documentation
```

## Features

### Database Features
- SQL support (CREATE TABLE, INSERT, SELECT with WHERE clause, CREATE INDEX)
- NoSQL support (Key-Value and Document stores)
- In-memory storage with Write-Ahead Logging (WAL) for persistence
- LRU cache for performance optimization
- HTTP API for easy access
- Built-in metrics and observability

### Concurrency Examples
The project includes a comprehensive examples crate demonstrating:
- Async/await with Tokio
- Parallel processing with Rayon
- Threading with std::thread
- Actor model with Actix
- Channel-based communication
- Stream processing
- And many more Rust concurrency patterns

## Quick Start

### Prerequisites
- Rust and Cargo (latest stable version)

### Building Core Components
```bash
cd tonledb
cargo build -p tonledb-core -p tonledb-storage -p tonledb-sql -p tonledb-nosql-kv -p tonledb-nosql-doc -p tonledb-metrics
```

### Running the Database
```bash
cd tonledb
RUST_LOG=info cargo run -p tonledb-network
```

## Installation via Package Managers

TonleDB packages are hosted on GitHub Releases and can be installed via package managers.

### Debian/Ubuntu
```bash
# Add repository
curl -sSL https://attakdefand.github.io/TonleDB/tonledb.list | sudo tee /etc/apt/sources.list.d/tonledb.list
sudo apt update

# Install from GitHub Releases
sudo apt install tonledb
```

### CentOS/RHEL/Fedora
```bash
# Add repository
# For CentOS/RHEL
sudo yum-config-manager --add-repo https://attakdefand.github.io/TonleDB/tonledb.repo

# For Fedora
sudo dnf config-manager --add-repo https://attakdefand.github.io/TonleDB/tonledb.repo

# Install from GitHub Releases
sudo yum install tonledb  # CentOS/RHEL
# or
sudo dnf install tonledb   # Fedora
```

### One-Command Installation
```bash
curl -sSL https://attakdefand.github.io/TonleDB/install.sh | sudo bash
```

### Package Repository
Visit our [GitHub Pages repository](https://attakdefand.github.io/TonleDB/) for installation instructions and repository configuration files.

## Documentation

For detailed instructions on building, running, and using TonleDB, see:
- [RUNNING.md](tonledb/official-docs/RUNNING.md) - Complete guide to running the project
- [PACKAGING.md](tonledb/official-docs/PACKAGING.md) - Manual instructions for packaging for Linux distributions
- [PUBLISHING.md](tonledb/official-docs/PUBLISHING.md) - Instructions for publishing packages to repositories
- [Architecture Documentation](tonledb/official-docs/md.md) - System architecture documentation
- [packaging/README.md](tonledb/packaging/README.md) - Information about automated packaging
- [packaging/INSTALL.md](tonledb/packaging/INSTALL.md) - User installation instructions
- [repo/README.md](tonledb/repo/README.md) - Package repository documentation

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
cd tonledb
cargo test --workspace
```

## License

This project is licensed under the MIT License - see the [LICENSE](tonledb/LICENSE) file for details.

## Contributing

Contributions are welcome! Please feel free to submit a Pull Request.
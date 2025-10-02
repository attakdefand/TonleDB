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
│   │   ├── tonledb-backup/     # Backup and recovery functionality
│   │   ├── tonledb-arrow/      # Arrow/Parquet support for analytics
│   │   ├── tonledb-wire-pg/    # PostgreSQL wire protocol compatibility
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
- **Enhanced Features in v0.2.0:**
  - Secondary indexes (B-Tree and Hash)
  - TTL (Time-To-Live) for automatic document expiration
  - MVCC (Multi-Version Concurrency Control)
  - Full ACID transactions with constraint validation
  - Arrow/Parquet support for analytics workloads
  - Row-level security
  - Point-in-time recovery (PITR) backups
  - PostgreSQL wire protocol compatibility

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

## Optimized Builds

To build TonleDB with full optimizations (removes the "unoptimized" message):

```bash
cd tonledb
# Build all components with optimizations
cargo build --release --workspace

# Or build specific components with optimizations
cargo build -p tonledb-network --release
cargo build -p tonledb-cli --release
```

The optimized binaries will be located in `tonledb/target/release/` directory.

To run the optimized database server:
```bash
cd tonledb
./target/release/tonledb-network
```

To run the optimized CLI:
```bash
cd tonledb
./target/release/tonledb-cli --help
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

## Download and Installation

### Option 1: Pre-built Packages (Recommended)

Download the latest release packages from [GitHub Releases](https://github.com/attakdefand/TonleDB/releases):

1. **For Debian/Ubuntu**:
   ```bash
   # Download the .deb package
   wget https://github.com/attakdefand/TonleDB/releases/download/v0.2.0/tonledb_0.2.0_amd64.deb
   
   # Install the package
   sudo dpkg -i tonledb_0.2.0_amd64.deb
   sudo apt-get install -f  # Fix any dependency issues
   ```

2. **For CentOS/RHEL/Fedora**:
   ```bash
   # Download the .rpm package
   wget https://github.com/attakdefand/TonleDB/releases/download/v0.2.0/tonledb-0.2.0-1.x86_64.rpm
   
   # Install the package
   sudo rpm -i tonledb-0.2.0-1.x86_64.rpm
   # Or on newer systems:
   sudo dnf install tonledb-0.2.0-1.x86_64.rpm
   ```

3. **Start the service**:
   ```bash
   sudo systemctl start tonledb
   sudo systemctl enable tonledb  # Enable auto-start on boot
   ```

### Option 2: Build from Source

1. **Clone the repository**:
   ```bash
   git clone https://github.com/attakdefand/TonleDB.git
   cd TonleDB
   ```

2. **Install build dependencies**:
   ```bash
   # Ubuntu/Debian
   sudo apt update
   sudo apt install cmake nasm protobuf-compiler build-essential
   
   # CentOS/RHEL/Fedora
   sudo yum install cmake nasm protobuf-compiler
   # or for newer versions:
   sudo dnf install cmake nasm protobuf-compiler
   ```

3. **Build the project**:
   ```bash
   cd tonledb
   cargo build --release --workspace
   ```

4. **Run the database**:
   ```bash
   cargo run --release -p tonledb-network
   ```

## Using TonleDB

### Starting the Service

After installation, TonleDB runs as a systemd service:

```bash
# Start the service
sudo systemctl start tonledb

# Check service status
sudo systemctl status tonledb

# Stop the service
sudo systemctl stop tonledb

# View logs
sudo journalctl -u tonledb -f
```

### Configuration

The default configuration file is located at `/etc/tonledb/tonledb.toml`. You can modify this file to change settings like:

- Server binding address and port
- TLS settings
- Rate limiting parameters
- Storage paths
- Authentication settings

### API Usage

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

### New Features Usage

#### Secondary Indexes
```sql
-- Create a secondary index
CREATE INDEX idx_email ON users (email);

-- Query using the index
SELECT * FROM users WHERE email = 'user@example.com';
```

#### TTL for Documents
```bash
# Insert a document with TTL (expires in 3600 seconds)
curl -X POST http://localhost:8080/doc/sessions \
  -H "Content-Type: application/json" \
  -d '{"user_id": "123", "token": "abc", "_ttl": 3600}'
```

#### Transactions
```bash
# Begin a transaction
curl -X POST http://localhost:8080/txn/begin

# Commit a transaction
curl -X POST http://localhost:8080/txn/commit

# Abort a transaction
curl -X POST http://localhost:8080/txn/abort
```

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

## Cross-Platform Support

TonleDB is built with Rust, making it cross-platform compatible. It can be used on:

- **Linux** (Debian/Ubuntu, CentOS/RHEL, Fedora)
- **Windows** (Windows 10, Windows 11, Windows Server)
- **macOS** (macOS 10.15+, Apple Silicon and Intel)

### Windows Installation

#### Option 1: Pre-built Binaries (Coming Soon)
Download pre-built Windows binaries from [GitHub Releases](https://github.com/attakdefand/TonleDB/releases).

#### Option 2: Build from Source
1. Install Rust toolchain from [rust-lang.org](https://www.rust-lang.org/)
2. Install build dependencies:
   - Install [Visual Studio Build Tools](https://visualstudio.microsoft.com/downloads/#build-tools-for-visual-studio-2022)
   - Or install [Visual Studio Community](https://visualstudio.microsoft.com/vs/community/) with C++ development tools
3. Clone and build:
   ```powershell
   git clone https://github.com/attakdefand/TonleDB.git
   cd TonleDB\tonledb
   cargo build --release --workspace
   ```
4. Run the optimized binaries:
   ```powershell
   .\target\release\tonledb-cli.exe --help
   ```

### macOS Installation

#### Option 1: Pre-built Binaries (Coming Soon)
Download pre-built macOS binaries from [GitHub Releases](https://github.com/attakdefand/TonleDB/releases).

#### Option 2: Build from Source
1. Install Rust toolchain:
   ```bash
   curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh
   ```
2. Install build dependencies:
   ```bash
   # Using Homebrew
   brew install cmake nasm protobuf
   
   # Using MacPorts
   sudo port install cmake nasm protobuf3-cpp
   ```
3. Clone and build:
   ```bash
   git clone https://github.com/attakdefand/TonleDB.git
   cd TonleDB/tonledb
   cargo build --release --workspace
   ```
4. Run the optimized binaries:
   ```bash
   ./target/release/tonledb-cli --help
   ```

### Cross-Platform Usage

The same commands work across all platforms:

```bash
# Initialize database
tonledb-cli init --wal mydb.wal

# Run SQL queries
tonledb-cli sql --query "CREATE TABLE users (id INT, name TEXT)"
tonledb-cli sql --query "INSERT INTO users VALUES (1, 'Alice')"
tonledb-cli sql --query "SELECT * FROM users"

# Key-Value operations
tonledb-cli kv put mykey myvalue
tonledb-cli kv get mykey
```

Note: Some advanced security features may require platform-specific configuration.

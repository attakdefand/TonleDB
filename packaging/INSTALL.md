# Installing TonleDB from Packages

This guide explains how to install TonleDB from pre-built packages.

## Debian/Ubuntu Installation

### From Release Packages

1. Download the `.deb` package from the [releases page](https://github.com/attakdefand/TonleDB/releases)
2. Install the package:
   ```bash
   sudo dpkg -i tonledb_0.2.0_amd64.deb
   sudo apt-get install -f  # Fix dependencies if needed
   ```

### From Package Repository (when available)

```bash
# Add the repository (example, adjust for actual repository)
echo "deb [trusted=yes] https://repo.tonledb.org/apt/ /" | sudo tee /etc/apt/sources.list.d/tonledb.list
sudo apt update
sudo apt install tonledb
```

## CentOS/RHEL/Fedora Installation

### From Release Packages

1. Download the `.rpm` package from the [releases page](https://github.com/attakdefand/TonleDB/releases)
2. Install the package:
   ```bash
   # For CentOS/RHEL
   sudo yum install tonledb-0.2.0-1.x86_64.rpm
   
   # For Fedora
   sudo dnf install tonledb-0.2.0-1.x86_64.rpm
   ```

### From Package Repository (when available)

```bash
# For CentOS/RHEL with EPEL
sudo yum install tonledb

# For Fedora with Copr
sudo dnf copr enable your-username/tonledb
sudo dnf install tonledb
```

## Post-Installation

After installation, TonleDB will be available as a system service:

### Starting the Service
```bash
sudo systemctl start tonledb
```

### Enabling Auto-Start on Boot
```bash
sudo systemctl enable tonledb
```

### Checking Service Status
```bash
sudo systemctl status tonledb
```

### Viewing Logs
```bash
sudo journalctl -u tonledb -f
```

## Configuration

The default configuration file is located at `/etc/tonledb/tonledb.toml`. You can modify this file to change settings like:

- Server binding address and port
- TLS settings
- Rate limiting parameters
- Storage paths
- Authentication settings

Example configuration:
```toml
[server]
bind = "0.0.0.0:8080"
require_tls = false
idle_timeout_ms = 30000
request_body_limit_bytes = 2097152
rate_limit_rps = 100
rate_limit_burst = 200

[tls]
enabled = false
cert_path = "/etc/tonledb/cert.pem"
key_path = "/etc/tonledb/key.pem"
require_client_auth = false
ca_path = "/etc/tonledb/ca.pem"

[auth]
mode = "none"
token_file = "/etc/tonledb/tokens.json"

[storage]
encrypt_at_rest = false
kek_env = "TONLEDB_KEK"
wal_path = "/var/lib/tonledb/wal"
```

## Usage

Once the service is running, you can interact with TonleDB via HTTP:

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

## Uninstalling

### Debian/Ubuntu
```bash
sudo apt remove tonledb
```

### CentOS/RHEL/Fedora
```bash
# For CentOS/RHEL
sudo yum remove tonledb

# For Fedora
sudo dnf remove tonledb
```

Note: Configuration files in `/etc/tonledb/` and data in `/var/lib/tonledb/` are typically preserved during uninstallation.

## Cross-Platform Support

TonleDB works on Linux, Windows, and macOS. The same functionality is available across all platforms.

### Windows Build Instructions

1. Install Rust toolchain from [rust-lang.org](https://www.rust-lang.org/)
2. Install Visual Studio Build Tools or Visual Studio Community with C++ development tools
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

Note: Some advanced security features may require additional Windows-specific dependencies.

### macOS Build Instructions

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

## Building Optimized Binaries from Source

If you prefer to build TonleDB from source with full optimizations:

1. Clone the repository:
   ```bash
   git clone https://github.com/attakdefand/TonleDB.git
   cd TonleDB
   ```

2. Install build dependencies:
   ```bash
   # Ubuntu/Debian
   sudo apt update
   sudo apt install cmake nasm protobuf-compiler build-essential
   
   # CentOS/RHEL/Fedora
   sudo yum install cmake nasm protobuf-compiler
   # or for newer versions:
   sudo dnf install cmake nasm protobuf-compiler
   ```

3. Build with optimizations:
   ```bash
   cd tonledb
   # Build all components with full optimizations
   cargo build --release --workspace
   
   # Or build specific components
   cargo build -p tonledb-network --release
   cargo build -p tonledb-cli --release
   ```

4. Run the optimized binaries:
   ```bash
   # Run the optimized server
   ./target/release/tonledb-network
   
   # Run the optimized CLI
   ./target/release/tonledb-cli --help
   ```

The optimized binaries will be located in `tonledb/target/release/` directory and will not show the "unoptimized" message during startup.

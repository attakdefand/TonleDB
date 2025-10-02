# Running TonleDB

This guide provides complete instructions on how to run and use the TonleDB project.

## Prerequisites

First, make sure you have the following installed:
1. Rust and Cargo (latest stable version)
2. Git

## Building the Project

Since some crates have external dependencies that require additional tools, you have a few options:

### Option 1: Build Only Core Components (Recommended to start)

```bash
cd tonledb
cargo build -p tonledb-core -p tonledb-storage -p tonledb-sql -p tonledb-nosql-kv -p tonledb-nosql-doc -p tonledb-metrics
```

This builds all the core database components without the network layer or examples.

### Option 2: Build Everything (Requires Additional Tools)

To build the complete project, you'll need to install:

1. **For tonledb-network**:
   - CMake (for aws-lc-sys dependency)
   - NASM (Netwide Assembler)

2. **For tonledb-examples**:
   - Protocol Buffer Compiler (protoc)

Installation instructions vary by platform:

#### Windows
- CMake: Download from https://cmake.org/download/
- NASM: Download from https://www.nasm.us/
- Protobuf: Download from https://github.com/protocolbuffers/protobuf/releases

#### macOS (using Homebrew)
```bash
brew install cmake nasm protobuf
```

#### Ubuntu/Debian
```bash
sudo apt update
sudo apt install cmake nasm protobuf-compiler
```

#### CentOS/RHEL/Fedora
```bash
sudo yum install cmake nasm protobuf-compiler
# or for newer versions:
sudo dnf install cmake nasm protobuf-compiler
```

After installing these dependencies, run:
```bash
cd tonledb
cargo build --workspace
```

## Running the Database

The main entry point for the database is in the tonledb-network crate. Once you've installed the required dependencies, you can run it with:

```bash
cd tonledb
cargo run -p tonledb-network
```

By default, it will look for a configuration file named `tonledb.toml`. You can find an example configuration in the project root.

## Using the Database

Once the database is running, you can interact with it in several ways:

### 1. HTTP API

The database exposes an HTTP API. You can use any HTTP client like curl:

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

### 2. Using the Core Components Directly

You can also use the core components as a library in your own Rust projects by adding them to your Cargo.toml:

```toml
[dependencies]
tonledb-core = { path = "../path/to/TonleDB/tonledb/crates/tonledb-core" }
tonledb-storage = { path = "../path/to/TonleDB/tonledb/crates/tonledb-storage" }
tonledb-sql = { path = "../path/to/TonleDB/tonledb/crates/tonledb-sql" }
```

Or if you want to use them as external dependencies (once published):
```toml
[dependencies]
tonledb-core = "0.1"
tonledb-storage = "0.1"
tonledb-sql = "0.1"
```

## Running Tests

To run tests for the core components:

```bash
cd tonledb
cargo test -p tonledb-core -p tonledb-storage -p tonledb-sql -p tonledb-nosql-kv -p tonledb-nosql-doc -p tonledb-metrics
```

To run all tests (requires all dependencies installed):
```bash
cd tonledb
cargo test --workspace
```

## Configuration

The database can be configured using a `tonledb.toml` file. Look at the example configuration in the project root to see available options.

Common configuration options include:
- Server binding address and port
- TLS settings
- Rate limiting parameters
- Storage paths
- Authentication settings

## Examples

To run the examples (once you've installed protoc):

```bash
cd tonledb
cargo run -p tonledb-examples
```

The examples demonstrate various Rust concurrency patterns implemented in the database, including:
- Async/await with Tokio
- Parallel processing with Rayon
- Threading with std::thread
- Actor model with Actix
- Channel-based communication
- Stream processing
- And more

## Troubleshooting

### Common Build Issues

1. **Missing cmake/nasm**: If you get errors about missing cmake or nasm when building tonledb-network, make sure you've installed these tools and that they're in your PATH.

2. **Missing protoc**: If you get errors about missing protoc when building tonledb-examples, make sure you've installed the Protocol Buffer compiler.

3. **Permission errors**: On some systems, you might need to run with elevated permissions to bind to certain ports.

### Performance Considerations

- For better performance during example runs, use `cargo run --release -p tonledb-examples`
- For production use of the database, always run with `--release` flag
- The database uses a Write-Ahead Log (WAL) for durability, which may impact performance

## Development Workflow

For daily development, the recommended workflow is:
1. Run `cargo build --workspace` for full build check
2. Execute examples via `cargo run -p tonledb-examples`
3. Run tests with `cargo test -p tonledb-examples`
4. Perform benchmarks using `cargo bench` as needed

## Summary

To get started quickly:
1. Build the core components: `cargo build -p tonledb-core -p tonledb-storage -p tonledb-sql -p tonledb-nosql-kv -p tonledb-nosql-doc -p tonledb-metrics`
2. Install CMake, NASM, and protoc if you want to build everything
3. Run the database: `cargo run -p tonledb-network`
4. Interact with it using HTTP requests

The core functionality is working and ready to use. The network layer and examples require additional tools to be installed but will work once those dependencies are met.
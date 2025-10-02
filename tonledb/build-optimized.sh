#!/bin/bash

# Build script for creating optimized TonleDB binaries
# This script builds the core TonleDB components with full optimizations

set -e  # Exit on any error

echo "Building TonleDB with full optimizations..."

# Check if we're in the right directory
if [ ! -f "Cargo.toml" ]; then
    echo "Error: Cargo.toml not found. Please run this script from the tonledb directory."
    exit 1
fi

# Build core components with optimizations (avoiding crates with complex dependencies)
echo "Building core components with optimizations..."
cargo build -p tonledb-cli --release
cargo build -p tonledb-core --release
cargo build -p tonledb-storage --release
cargo build -p tonledb-sql --release
cargo build -p tonledb-nosql-kv --release
cargo build -p tonledb-nosql-doc --release

echo "Optimized build completed successfully!"
echo "Binaries are located in target/release/"

# List the main binaries
echo "Main optimized binaries:"
ls -lh target/release/tonledb-cli*

echo ""
echo "To run the optimized CLI:"
echo "  ./target/release/tonledb-cli --help"
echo ""
echo "Note: Some advanced features may require additional build dependencies."
echo "For a complete build, install cmake and nasm, then run:"
echo "  cargo build --release --workspace"
#!/bin/bash

# Script to build TonleDB packages for Linux distributions

set -e  # Exit on any error

# Colors for output
RED='\033[0;31m'
GREEN='\033[0;32m'
YELLOW='\033[1;33m'
NC='\033[0m' # No Color

# Function to print colored output
print_status() {
    echo -e "${GREEN}[INFO]${NC} $1"
}

print_warning() {
    echo -e "${YELLOW}[WARNING]${NC} $1"
}

print_error() {
    echo -e "${RED}[ERROR]${NC} $1"
}

# Check if we're in the right directory
if [ ! -f "../Cargo.toml" ]; then
    print_error "This script must be run from the packaging directory"
    exit 1
fi

# Detect OS
if [ -f /etc/os-release ]; then
    . /etc/os-release
    OS=$NAME
    VER=$VERSION_ID
else
    print_error "Cannot detect OS"
    exit 1
fi

print_status "Detected OS: $OS $VER"

# Function to build Debian package
build_deb() {
    print_status "Building Debian package..."
    
    # Check if required tools are installed
    if ! command -v dpkg-deb &> /dev/null; then
        print_error "dpkg-deb is not installed. Please install build-essential package."
        exit 1
    fi
    
    if ! command -v cargo &> /dev/null; then
        print_error "cargo is not installed. Please install Rust."
        exit 1
    fi
    
    # Build the release version with all crates including new features
    print_status "Building TonleDB release version..."
    cd ..
    cargo build --release -p tonledb-core -p tonledb-storage -p tonledb-sql -p tonledb-nosql-kv -p tonledb-nosql-doc -p tonledb-metrics -p tonledb-network -p tonledb-backup -p tonledb-arrow -p tonledb-wire-pg
    cd packaging
    
    # Copy the binary
    print_status "Copying binary..."
    cp ../target/release/tonledb-network debian/tonledb/usr/bin/tonledb
    
    # Copy libraries and configuration
    print_status "Copying libraries and configuration..."
    cp -r ../crates debian/tonledb/usr/lib/tonledb/
    cp ../tonledb.toml debian/tonledb/usr/lib/tonledb/tonledb.toml.example
    
    # Copy documentation
    print_status "Copying documentation..."
    cp ../README.md debian/tonledb/usr/share/doc/tonledb/
    
    # Set permissions
    print_status "Setting permissions..."
    chmod 755 debian/tonledb/usr/bin/tonledb
    find debian/tonledb -type d -exec chmod 755 {} \;
    
    # Build the package
    print_status "Building .deb package..."
    dpkg-deb --build debian/tonledb tonledb_0.2.0_amd64.deb
    
    print_status "Debian package built successfully: tonledb_0.2.0_amd64.deb"
}

# Function to build RPM package
build_rpm() {
    print_status "Building RPM package..."
    
    # Check if required tools are installed
    if ! command -v rpmbuild &> /dev/null; then
        print_error "rpmbuild is not installed. Please install rpm-build package."
        exit 1
    fi
    
    if ! command -v cargo &> /dev/null; then
        print_error "cargo is not installed. Please install Rust."
        exit 1
    fi
    
    # Create source tarball
    print_status "Creating source tarball..."
    cd ..
    tar -czf packaging/tonledb-0.2.0.tar.gz --exclude='target' --exclude='.git' --exclude='packaging/debian' .
    cd packaging
    
    # Build the RPM
    print_status "Building RPM package..."
    rpmbuild -ba rpm/tonledb.spec
    
    print_status "RPM package built successfully"
}

# Main script logic
case "${1:-both}" in
    deb)
        build_deb
        ;;
    rpm)
        build_rpm
        ;;
    both)
        build_deb
        build_rpm
        ;;
    *)
        echo "Usage: $0 [deb|rpm|both]"
        echo "  deb   - Build only Debian package"
        echo "  rpm   - Build only RPM package"
        echo "  both  - Build both packages (default)"
        exit 1
        ;;
esac

print_status "Packaging completed successfully!"
#!/bin/bash

# TonleDB Installation Script
# This script detects the OS and installs TonleDB appropriately

set -e

# Colors
RED='\033[0;31m'
GREEN='\033[0;32m'
YELLOW='\033[1;33m'
NC='\033[0m' # No Color

# Functions
print_status() {
    echo -e "${GREEN}[INFO]${NC} $1"
}

print_warning() {
    echo -e "${YELLOW}[WARNING]${NC} $1"
}

print_error() {
    echo -e "${RED}[ERROR]${NC} $1"
}

# Detect OS
detect_os() {
    if [ -f /etc/os-release ]; then
        . /etc/os-release
        OS=$NAME
        VER=$VERSION_ID
    elif type lsb_release >/dev/null 2>&1; then
        OS=$(lsb_release -si)
        VER=$(lsb_release -sr)
    elif [ -f /etc/lsb-release ]; then
        . /etc/lsb-release
        OS=$DISTRIB_ID
        VER=$DISTRIB_RELEASE
    elif [ -f /etc/debian_version ]; then
        OS=Debian
        VER=$(cat /etc/debian_version)
    elif [ -f /etc/redhat-release ]; then
        OS=$(cat /etc/redhat-release | cut -d ' ' -f 1)
        VER=$(cat /etc/redhat-release | grep -oE '[0-9]+\.[0-9]+' | head -1)
    else
        OS=$(uname -s)
        VER=$(uname -r)
    fi
    
    print_status "Detected OS: $OS $VER"
}

# Install on Debian/Ubuntu
install_deb() {
    print_status "Installing TonleDB on Debian/Ubuntu..."
    
    # Check if running as root
    if [ "$EUID" -ne 0 ]; then
        print_warning "This script needs to run as root. Re-executing with sudo..."
        sudo "$0" "$@"
        exit $?
    fi
    
    # Add repository
    echo "deb [trusted=yes] https://attakdefand.github.io/TonleDB/apt/ /" > /etc/apt/sources.list.d/tonledb.list
    
    # Update package list
    apt update
    
    # Install TonleDB
    apt install -y tonledb
    
    print_status "TonleDB installed successfully!"
}

# Install on CentOS/RHEL/Fedora
install_rpm() {
    print_status "Installing TonleDB on Red Hat based system..."
    
    # Check if running as root
    if [ "$EUID" -ne 0 ]; then
        print_warning "This script needs to run as root. Re-executing with sudo..."
        sudo "$0" "$@"
        exit $?
    fi
    
    # Detect if dnf or yum is available
    if command -v dnf &> /dev/null; then
        PACKAGE_MANAGER="dnf"
    elif command -v yum &> /dev/null; then
        PACKAGE_MANAGER="yum"
    else
        print_error "Neither dnf nor yum package manager found."
        exit 1
    fi
    
    # Add repository
    if [ "$PACKAGE_MANAGER" = "dnf" ]; then
        dnf config-manager --add-repo https://attakdefand.github.io/TonleDB/yum/tonledb.repo
        dnf install -y tonledb
    else
        yum-config-manager --add-repo https://attakdefand.github.io/TonleDB/yum/tonledb.repo
        yum install -y tonledb
    fi
    
    print_status "TonleDB installed successfully!"
}

# Install from GitHub releases
install_from_github() {
    print_status "Installing TonleDB from GitHub releases..."
    
    # Detect architecture
    ARCH=$(uname -m)
    if [ "$ARCH" = "x86_64" ]; then
        PACKAGE_ARCH="amd64"
    else
        print_error "Unsupported architecture: $ARCH"
        exit 1
    fi
    
    # Detect package type
    if command -v dpkg &> /dev/null; then
        PACKAGE_TYPE="deb"
    elif command -v rpm &> /dev/null; then
        PACKAGE_TYPE="rpm"
    else
        print_error "Neither dpkg nor rpm found. Cannot install package."
        exit 1
    fi
    
    # Download package
    PACKAGE_URL="https://github.com/attakdefand/TonleDB/releases/latest/download/tonledb_0.2.0_${PACKAGE_ARCH}.${PACKAGE_TYPE}"
    print_status "Downloading package from $PACKAGE_URL..."
    curl -L -o "/tmp/tonledb.${PACKAGE_TYPE}" "$PACKAGE_URL"
    
    # Install package
    if [ "$PACKAGE_TYPE" = "deb" ]; then
        dpkg -i "/tmp/tonledb.${PACKAGE_TYPE}" || apt-get install -f -y
    else
        rpm -i "/tmp/tonledb.${PACKAGE_TYPE}"
    fi
    
    # Clean up
    rm "/tmp/tonledb.${PACKAGE_TYPE}"
    
    print_status "TonleDB installed successfully!"
}

# Main
main() {
    print_status "TonleDB Installation Script"
    
    # Detect OS
    detect_os
    
    # Install based on OS
    case $OS in
        "Ubuntu"|"Debian"|"Linux Mint"|"elementary OS")
            install_deb
            ;;
        "CentOS"|"Red Hat"|"Fedora"|"Rocky Linux"|"AlmaLinux")
            install_rpm
            ;;
        *)
            print_warning "Unsupported OS: $OS"
            print_status "Attempting to install from GitHub releases..."
            install_from_github
            ;;
    esac
    
    # Start service
    print_status "Starting TonleDB service..."
    systemctl daemon-reload
    systemctl enable tonledb
    systemctl start tonledb
    
    print_status "TonleDB installation completed!"
    echo "Service status: $(systemctl is-active tonledb)"
    echo "Configuration file: /etc/tonledb/tonledb.toml"
    echo "Data directory: /var/lib/tonledb/"
    echo "Logs: journalctl -u tonledb -f"
}

# Run main function
main "$@"
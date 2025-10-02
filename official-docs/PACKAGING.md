# Packaging TonleDB for Linux Distributions

This guide explains how to package TonleDB for Linux distributions and make it available through package managers like `apt` and `yum`.

## Overview

Packaging TonleDB for Linux distributions involves creating distribution-specific packages:
- `.deb` packages for Debian/Ubuntu-based systems
- `.rpm` packages for Red Hat/CentOS/Fedora-based systems

## Prerequisites for Package Building

Before creating packages, you'll need to install the necessary tools:

### For Debian/Ubuntu (.deb packages)
```bash
sudo apt update
sudo apt install build-essential debhelper dh-cargo cargo rustc
```

### For CentOS/RHEL/Fedora (.rpm packages)
```bash
# For CentOS/RHEL
sudo yum install rpm-build cargo rustc

# For Fedora
sudo dnf install rpm-build cargo rustc
```

## Creating a .deb Package for Debian/Ubuntu

### 1. Create the directory structure
```bash
mkdir -p tonledb-debian/DEBIAN
mkdir -p tonledb-debian/usr/bin
mkdir -p tonledb-debian/usr/lib/tonledb
mkdir -p tonledb-debian/etc/tonledb
mkdir -p tonledb-debian/var/lib/tonledb
mkdir -p tonledb-debian/usr/share/doc/tonledb
```

### 2. Create the control file
```bash
cat > tonledb-debian/DEBIAN/control << EOF
Package: tonledb
Version: 0.1.0
Section: database
Priority: optional
Architecture: amd64
Depends: libc6 (>= 2.27)
Maintainer: Your Name <your.email@example.com>
Description: TonleDB - A hybrid SQL/NoSQL database
 A Rust database with SQL + NoSQL over a shared storage+txn kernel.
 Supports SQL queries, Key-Value operations, and Document storage.
EOF
```

### 3. Create pre-installation and post-installation scripts
```bash
# Pre-installation script
cat > tonledb-debian/DEBIAN/preinst << 'EOF'
#!/bin/bash
set -e

# Create tonledb user if it doesn't exist
if ! id tonledb >/dev/null 2>&1; then
    useradd --system --home /var/lib/tonledb --shell /bin/false tonledb
fi

exit 0
EOF

chmod 755 tonledb-debian/DEBIAN/preinst

# Post-installation script
cat > tonledb-debian/DEBIAN/postinst << 'EOF'
#!/bin/bash
set -e

# Set proper ownership
chown -R tonledb:tonledb /var/lib/tonledb
chmod 750 /var/lib/tonledb

# Create default configuration
if [ ! -f /etc/tonledb/tonledb.toml ]; then
    cp /usr/lib/tonledb/tonledb.toml.example /etc/tonledb/tonledb.toml
    chown root:tonledb /etc/tonledb/tonledb.toml
    chmod 640 /etc/tonledb/tonledb.toml
fi

# Enable and start service if systemd is available
if command -v systemctl >/dev/null 2>&1; then
    systemctl daemon-reload
    systemctl enable tonledb.service
fi

exit 0
EOF

chmod 755 tonledb-debian/DEBIAN/postinst

# Post-removal script
cat > tonledb-debian/DEBIAN/prerm << 'EOF'
#!/bin/bash
set -e

# Stop service if systemd is available
if command -v systemctl >/dev/null 2>&1; then
    systemctl stop tonledb.service || true
    systemctl disable tonledb.service || true
fi

exit 0
EOF

chmod 755 tonledb-debian/DEBIAN/prerm
```

### 4. Build TonleDB and copy binaries
```bash
# Build the release version
cd tonledb
cargo build --release -p tonledb-core -p tonledb-storage -p tonledb-sql -p tonledb-nosql-kv -p tonledb-nosql-doc -p tonledb-metrics -p tonledb-network

# Copy the binary
cp target/release/tonledb-network ../tonledb-debian/usr/bin/tonledb

# Copy libraries and configuration
cp -r crates ../tonledb-debian/usr/lib/tonledb/
cp tonledb.toml ../tonledb-debian/usr/lib/tonledb/tonledb.toml.example

# Copy documentation
cp README.md ../tonledb-debian/usr/share/doc/tonledb/
```

### 5. Create systemd service file
```bash
mkdir -p tonledb-debian/usr/lib/systemd/system

cat > tonledb-debian/usr/lib/systemd/system/tonledb.service << 'EOF'
[Unit]
Description=TonleDB Database Server
After=network.target

[Service]
Type=simple
User=tonledb
Group=tonledb
ExecStart=/usr/bin/tonledb
WorkingDirectory=/var/lib/tonledb
Restart=always
RestartSec=10

[Install]
WantedBy=multi-user.target
EOF
```

### 6. Build the package
```bash
# Set permissions
chmod 755 tonledb-debian/usr/bin/tonledb
find tonledb-debian -type d -exec chmod 755 {} \;

# Build the package
dpkg-deb --build tonledb-debian tonledb_0.1.0_amd64.deb
```

### 7. Install and test the package
```bash
sudo dpkg -i tonledb_0.1.0_amd64.deb
sudo systemctl start tonledb
```

## Creating an .rpm Package for Red Hat/CentOS/Fedora

### 1. Create the SPEC file
```bash
cat > tonledb.spec << 'EOF'
Name:           tonledb
Version:        0.1.0
Release:        1%{?dist}
Summary:        TonleDB - A hybrid SQL/NoSQL database

License:        MIT
URL:            https://github.com/your-username/tonledb
Source0:        %{name}-%{version}.tar.gz

BuildRequires:  cargo rustc
BuildArch:      x86_64

Requires:       systemd

%description
TonleDB is a Rust database with SQL + NoSQL over a shared storage+txn kernel.
This MVP supports:
- SQL: CREATE TABLE, INSERT, SELECT (WHERE =), CREATE INDEX (single-col equality)
- NoSQL: KV CRUD, Document insert/get
- In-memory + WAL persistence, LRU cache wrapper
- HTTP API (Axum) + CLI

%prep
%setup -q

%build
cargo build --release -p tonledb-core -p tonledb-storage -p tonledb-sql -p tonledb-nosql-kv -p tonledb-nosql-doc -p tonledb-metrics -p tonledb-network

%install
mkdir -p %{buildroot}/%{_bindir}
mkdir -p %{buildroot}/%{_libdir}/tonledb
mkdir -p %{buildroot}/%{_sysconfdir}/tonledb
mkdir -p %{buildroot}/%{_sharedstatedir}/tonledb
mkdir -p %{buildroot}/%{_unitdir}

install -m 755 target/release/tonledb-network %{buildroot}/%{_bindir}/tonledb
cp -r crates %{buildroot}/%{_libdir}/tonledb/
install -m 644 tonledb.toml %{buildroot}/%{_libdir}/tonledb/tonledb.toml.example

install -m 644 systemd/tonledb.service %{buildroot}/%{_unitdir}/

%pre
getent group tonledb >/dev/null || groupadd -r tonledb
getent passwd tonledb >/dev/null || useradd -r -g tonledb -d %{_sharedstatedir}/tonledb -s /sbin/nologin -c "TonleDB" tonledb

%post
/bin/systemctl daemon-reload >/dev/null 2>&1 || :
/bin/systemctl enable tonledb.service >/dev/null 2>&1 || :

%preun
/bin/systemctl stop tonledb.service >/dev/null 2>&1 || :
/bin/systemctl disable tonledb.service >/dev/null 2>&1 || :

%postun
/bin/systemctl daemon-reload >/dev/null 2>&1 || :

%files
%{_bindir}/tonledb
%{_libdir}/tonledb/*
%{_unitdir}/tonledb.service
%dir %{_sysconfdir}/tonledb
%dir %{_sharedstatedir}/tonledb

%changelog
* Sat Oct 02 2025 Your Name <your.email@example.com> - 0.1.0-1
- Initial package
EOF
```

### 2. Build the RPM package
```bash
# Create source tarball
tar -czf tonledb-0.1.0.tar.gz tonledb

# Build the RPM
rpmbuild -ba tonledb.spec
```

## Publishing to Package Repositories

### For Debian/Ubuntu (PPA)
1. Create a Launchpad account
2. Set up a PPA (Personal Package Archive)
3. Upload your source package:
```bash
debuild -S
dput ppa:your-ppa-name/tonledb ../tonledb_0.1.0-1_source.changes
```

### For Fedora (Copr)
1. Create a Copr account
2. Install copr-cli:
```bash
sudo dnf install copr-cli
```
3. Upload and build:
```bash
copr-cli build your-username/tonledb tonledb-0.1.0.tar.gz
```

### For CentOS/RHEL (EPEL)
Submit your package to the EPEL (Extra Packages for Enterprise Linux) repository through the formal review process.

## Automated Packaging with CI/CD

You can automate the packaging process using GitHub Actions or other CI/CD systems:

### GitHub Actions Example
```yaml
name: Build and Package

on:
  release:
    types: [created]

jobs:
  build-deb:
    runs-on: ubuntu-latest
    steps:
    - uses: actions/checkout@v2
    - name: Install dependencies
      run: |
        sudo apt update
        sudo apt install build-essential debhelper dh-cargo cargo rustc
    - name: Build deb package
      run: |
        # Follow the steps above to create .deb package
    - name: Upload artifact
      uses: actions/upload-artifact@v2
      with:
        name: tonledb-deb-package
        path: tonledb_*.deb

  build-rpm:
    runs-on: ubuntu-latest
    container: fedora:latest
    steps:
    - uses: actions/checkout@v2
    - name: Install dependencies
      run: |
        dnf install -y rpm-build cargo rustc
    - name: Build rpm package
      run: |
        # Follow the steps above to create .rpm package
    - name: Upload artifact
      uses: actions/upload-artifact@v2
      with:
        name: tonledb-rpm-package
        path: *.rpm
```

## User Installation Instructions

Once packaged and published, users can install TonleDB with:

### Debian/Ubuntu
```bash
# If using PPA
sudo add-apt-repository ppa:your-ppa-name/tonledb
sudo apt update
sudo apt install tonledb

# Or with direct package installation
wget https://example.com/tonledb_0.1.0_amd64.deb
sudo dpkg -i tonledb_0.1.0_amd64.deb
sudo apt-get install -f  # Fix dependencies if needed
```

### CentOS/RHEL/Fedora
```bash
# If using Copr
sudo dnf copr enable your-username/tonledb
sudo dnf install tonledb

# Or with direct package installation
sudo rpm -i tonledb-0.1.0-1.x86_64.rpm
```

## Service Management

After installation, TonleDB will be available as a system service:

```bash
# Start the service
sudo systemctl start tonledb

# Enable auto-start on boot
sudo systemctl enable tonledb

# Check status
sudo systemctl status tonledb

# View logs
sudo journalctl -u tonledb -f
```

## Configuration

The default configuration file will be located at `/etc/tonledb/tonledb.toml`. Users can modify this file to change settings like:

- Server binding address and port
- TLS settings
- Rate limiting parameters
- Storage paths
- Authentication settings

## Conclusion

Packaging TonleDB for Linux distributions makes it much easier for users to install and maintain. The process involves:

1. Creating distribution-specific packages (.deb for Debian/Ubuntu, .rpm for Red Hat/CentOS/Fedora)
2. Setting up proper user permissions and service management
3. Providing default configuration files
4. Publishing to package repositories for easy installation

With proper packaging, users can install TonleDB with a simple command like `sudo apt install tonledb` or `sudo dnf install tonledb`.
# TonleDB Packaging

This directory contains files and scripts for packaging TonleDB for Linux distributions.

## Directory Structure

```
packaging/
├── debian/                 # Debian/Ubuntu packaging files
│   └── tonledb/           # Debian package structure
│       ├── DEBIAN/        # Control files
│       ├── etc/tonledb/   # Configuration files
│       ├── usr/bin/       # Binary files
│       ├── usr/lib/tonledb/  # Libraries and data
│       ├── usr/lib/systemd/system/  # Systemd service files
│       └── var/lib/tonledb/  # Data directory
├── rpm/                   # RPM packaging files
│   └── tonledb.spec       # RPM SPEC file
├── Makefile               # Build automation
├── INSTALL.md             # User installation instructions
└── README.md              # This file
```

## Building Packages

### Prerequisites

Before building packages, you'll need to install the necessary tools:

#### For Debian/Ubuntu (.deb packages)
```bash
sudo apt update
sudo apt install build-essential debhelper dh-cargo cargo rustc
```

#### For CentOS/RHEL/Fedora (.rpm packages)
```bash
# For CentOS/RHEL
sudo yum install rpm-build cargo rustc make

# For Fedora
sudo dnf install rpm-build cargo rustc make
```

### Building with Make

The easiest way to build packages is using the provided Makefile:

```bash
# Build both Debian and RPM packages
make

# Build only Debian package
make deb

# Build only RPM package
make rpm

# Install dependencies for Debian packaging
make install-deps-deb

# Install dependencies for RPM packaging
make install-deps-rpm

# Clean build artifacts
make clean
```

### Manual Building

#### Debian Package
```bash
# Build the release version
cd ..
cargo build --release -p tonledb-core -p tonledb-storage -p tonledb-sql -p tonledb-nosql-kv -p tonledb-nosql-doc -p tonledb-metrics -p tonledb-network

# Copy files to package structure
cp target/release/tonledb-network debian/tonledb/usr/bin/tonledb
cp -r crates debian/tonledb/usr/lib/tonledb/
cp tonledb.toml debian/tonledb/usr/lib/tonledb/tonledb.toml.example

# Set permissions
chmod 755 debian/tonledb/usr/bin/tonledb
find debian/tonledb -type d -exec chmod 755 {} \;

# Build the package
dpkg-deb --build debian/tonledb tonledb_0.1.0_amd64.deb
```

#### RPM Package
```bash
# Create source tarball
cd ..
tar -czf packaging/tonledb-0.1.0.tar.gz --exclude='target' --exclude='.git' .

# Build the RPM
rpmbuild -ba packaging/rpm/tonledb.spec
```

## Automated Packaging with CI/CD

This repository includes GitHub Actions workflows for automated packaging:

- [.github/workflows/package.yml](../tonledb/.github/workflows/package.yml) - Builds packages when a release is created

## Package Contents

The packages include:

1. **Binaries**: The tonledb-network executable
2. **Libraries**: Core components as libraries
3. **Configuration**: Default configuration file
4. **Systemd Service**: Service definition for systemd
5. **Documentation**: README and other documentation files

## Service Management

After installation, TonleDB runs as a systemd service:

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

The default configuration file is installed at `/etc/tonledb/tonledb.toml`. 
The service runs under the `tonledb` user and stores data in `/var/lib/tonledb/`.

## Publishing to Repositories

To publish packages to repositories:

### Debian/Ubuntu (PPA)
1. Create a Launchpad account
2. Set up a PPA (Personal Package Archive)
3. Upload your source package:
```bash
debuild -S
dput ppa:your-ppa-name/tonledb ../tonledb_0.1.0-1_source.changes
```

### Fedora (Copr)
1. Create a Copr account
2. Install copr-cli:
```bash
sudo dnf install copr-cli
```
3. Upload and build:
```bash
copr-cli build your-username/tonledb tonledb-0.1.0.tar.gz
```

## Contributing

If you'd like to improve the packaging:

1. Fork the repository
2. Make your changes
3. Test the packages
4. Submit a pull request
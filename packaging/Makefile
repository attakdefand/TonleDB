# Makefile for packaging TonleDB for Linux distributions

VERSION = 0.1.0
PACKAGE_NAME = tonledb

# Directories
DEB_BUILD_DIR = ./debian/$(PACKAGE_NAME)
RPM_SPEC = ./rpm/$(PACKAGE_NAME).spec

# Default target
all: deb rpm

# Build Debian package
deb: build-deb-package
	dpkg-deb --build $(DEB_BUILD_DIR) $(PACKAGE_NAME)_$(VERSION)_amd64.deb

# Build Debian package structure
build-deb-package:
	# Build the release version
	cd .. && cargo build --release -p tonledb-core -p tonledb-storage -p tonledb-sql -p tonledb-nosql-kv -p tonledb-nosql-doc -p tonledb-metrics -p tonledb-network
	
	# Copy the binary
	cp ../target/release/tonledb-network $(DEB_BUILD_DIR)/usr/bin/tonledb
	
	# Copy libraries and configuration
	cp -r ../crates $(DEB_BUILD_DIR)/usr/lib/tonledb/
	cp ../tonledb.toml $(DEB_BUILD_DIR)/usr/lib/tonledb/tonledb.toml.example
	
	# Copy documentation
	cp ../README.md $(DEB_BUILD_DIR)/usr/share/doc/tonledb/
	
	# Set permissions
	chmod 755 $(DEB_BUILD_DIR)/usr/bin/tonledb
	find $(DEB_BUILD_DIR) -type d -exec chmod 755 {} \;

# Build RPM package
rpm:
	# Create source tarball
	cd .. && tar -czf packaging/$(PACKAGE_NAME)-$(VERSION).tar.gz --exclude='target' --exclude='.git' .
	
	# Build the RPM
	rpmbuild -ba $(RPM_SPEC)

# Install dependencies for packaging
install-deps-deb:
	sudo apt update
	sudo apt install build-essential debhelper dh-cargo cargo rustc

install-deps-rpm:
	sudo yum install rpm-build cargo rustc
	# For newer versions:
	# sudo dnf install rpm-build cargo rustc

# Clean build artifacts
clean:
	rm -f *.deb
	rm -f *.rpm
	rm -f *.tar.gz
	rm -rf $(DEB_BUILD_DIR)/usr/bin/tonledb
	rm -rf $(DEB_BUILD_DIR)/usr/lib/tonledb
	rm -rf $(DEB_BUILD_DIR)/etc/tonledb/tonledb.toml

# Install the Debian package locally for testing
install-deb: deb
	sudo dpkg -i $(PACKAGE_NAME)_$(VERSION)_amd64.deb
	sudo apt-get install -f  # Fix dependencies if needed

# Install the RPM package locally for testing
install-rpm: rpm
	sudo rpm -i *.rpm

.PHONY: all deb rpm install-deps-deb install-deps-rpm clean install-deb install-rpm
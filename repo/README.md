# TonleDB Package Repository

This directory contains the files needed to set up a package repository for TonleDB.

## Directory Structure

```
repo/
├── apt/           # APT repository files for Debian/Ubuntu
├── yum/           # YUM repository files for CentOS/RHEL/Fedora
├── tonledb.list   # APT repository configuration
├── tonledb.repo   # YUM repository configuration
└── README.md      # This file
```

## Setting up a Repository

### For HTTP Hosting

To host these files on a web server:

1. Copy the contents of this directory to your web server
2. Ensure the files are accessible via HTTP/HTTPS
3. Users can then add the repository to their systems

### For GitHub Pages

You can also host the repository files using GitHub Pages:

1. Create a `gh-pages` branch in your repository
2. Copy the contents of this directory to the `gh-pages` branch
3. Enable GitHub Pages in your repository settings

## User Installation Instructions

### Debian/Ubuntu

Users can add the repository with:

```bash
curl -sSL https://repo.tonledb.org/tonledb.list | sudo tee /etc/apt/sources.list.d/tonledb.list
sudo apt update
sudo apt install tonledb
```

### CentOS/RHEL/Fedora

Users can add the repository with:

```bash
# For CentOS/RHEL
sudo yum-config-manager --add-repo https://repo.tonledb.org/tonledb.repo
sudo yum install tonledb

# For Fedora
sudo dnf config-manager --add-repo https://repo.tonledb.org/tonledb.repo
sudo dnf install tonledb
```

## Repository Structure

When hosting the actual packages, the repository should have the following structure:

```
repo/
├── apt/
│   ├── dists/
│   │   └── stable/
│   │       ├── main/
│   │       │   ├── binary-amd64/
│   │       │   │   ├── Packages
│   │       │   │   └── Packages.gz
│   │       │   └── source/
│   │       │       ├── Sources
│   │       │       └── Sources.gz
│   │       └── Release
│   └── pool/
│       └── main/
│           └── t/
│               └── tonledb/
│                   ├── tonledb_0.1.0-1_amd64.deb
│                   └── tonledb_0.1.0-1.dsc
├── yum/
│   ├── Packages/
│   │   └── tonledb-0.1.0-1.x86_64.rpm
│   └── repodata/
│       ├── filelists.xml.gz
│       ├── other.xml.gz
│       ├── primary.xml.gz
│       └── repomd.xml
├── tonledb.list
├── tonledb.repo
└── README.md
```

## Publishing New Versions

To publish new versions:

1. Build new packages using the packaging system
2. Update the repository metadata
3. Upload the new packages and metadata to the repository
4. Users will automatically get updates through their package manager

See [../packaging/README.md](../packaging/README.md) for detailed instructions on building packages.
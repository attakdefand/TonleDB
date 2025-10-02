# Publishing TonleDB Packages

This guide explains how to publish TonleDB packages to package repositories for easy installation by users.

## Overview

Publishing TonleDB packages to official repositories makes it easy for users to install and update the database using their system's package manager. This guide covers publishing to:

1. Ubuntu PPA (Personal Package Archive)
2. Fedora Copr
3. EPEL (Extra Packages for Enterprise Linux) for CentOS/RHEL

## Publishing to Ubuntu PPA

### Prerequisites

1. Create a Launchpad account at https://launchpad.net/
2. Set up your GPG key for package signing
3. Create a new PPA in your Launchpad account

### Steps

1. Install required tools:
   ```bash
   sudo apt install devscripts debhelper dh-make
   ```

2. Create a source package:
   ```bash
   # In the packaging directory
   cd debian/tonledb
   dpkg-buildpackage -S -us -uc
   ```

3. Upload to your PPA:
   ```bash
   dput ppa:your-username/your-ppa-name ../tonledb_0.1.0-1_source.changes
   ```

4. Wait for the package to build in Launchpad (this can take some time)

5. Once built, users can install with:
   ```bash
   sudo add-apt-repository ppa:your-username/your-ppa-name
   sudo apt update
   sudo apt install tonledb
   ```

## Publishing to Fedora Copr

### Prerequisites

1. Create a Fedora account at https://accounts.fedoraproject.org/
2. Install copr-cli:
   ```bash
   sudo dnf install copr-cli
   ```

3. Log in to Copr:
   ```bash
   copr-cli login
   ```

### Steps

1. Create a new Copr project:
   ```bash
   copr-cli create --chroot fedora-35-x86_64 --chroot fedora-36-x86_64 tonledb
   ```

2. Build the package:
   ```bash
   # Create source tarball first
   cd ../..
   tar -czf tonledb-0.1.0.tar.gz --exclude='target' --exclude='.git' .
   
   # Upload and build
   copr-cli build your-username/tonledb tonledb-0.1.0.tar.gz
   ```

3. Once built, users can install with:
   ```bash
   sudo dnf copr enable your-username/tonledb
   sudo dnf install tonledb
   ```

## Publishing to EPEL (CentOS/RHEL)

### Prerequisites

1. Join the EPEL team by following the instructions at https://fedoraproject.org/wiki/EPEL
2. Understand the EPEL package review process

### Steps

1. Submit a package review request at https://bugzilla.redhat.com/
2. Prepare your package according to EPEL guidelines
3. Go through the community review process
4. Once approved, the package will be available in EPEL repositories

## Automated Publishing with GitHub Actions

You can automate the publishing process using GitHub Actions. Here's an example workflow:

```yaml
name: Publish Packages

on:
  release:
    types: [published]

jobs:
  publish-deb:
    runs-on: ubuntu-latest
    steps:
    - uses: actions/checkout@v2
    
    - name: Install dependencies
      run: |
        sudo apt update
        sudo apt install devscripts debhelper dh-make dput
        
    - name: Build and publish to PPA
      run: |
        # Set up GPG key from secrets
        echo "${{ secrets.GPG_KEY }}" | gpg --import
        
        # Build source package
        cd packaging/debian/tonledb
        dpkg-buildpackage -S -sa
        
        # Upload to PPA
        dput ppa:your-username/your-ppa-name ../../tonledb_*.changes
      env:
        DEBEMAIL: your-email@example.com
        DEBFULLNAME: Your Name

  publish-rpm:
    runs-on: ubuntu-latest
    container: fedora:latest
    steps:
    - uses: actions/checkout@v2
    
    - name: Install dependencies
      run: |
        dnf install -y copr-cli rpm-build cargo rustc
        
    - name: Build and publish to Copr
      run: |
        # Set up Copr configuration from secrets
        mkdir -p ~/.config
        echo "${{ secrets.COPR_CONFIG }}" > ~/.config/copr
        
        # Create source tarball
        cd packaging
        tar -czf tonledb-0.1.0.tar.gz --exclude='target' --exclude='.git' --exclude='packaging/debian' ..
        
        # Build and upload to Copr
        copr-cli build your-username/tonledb tonledb-0.1.0.tar.gz
```

## Repository Signing

All official repositories should sign packages with GPG keys to ensure authenticity and security.

### Generating GPG Keys

```bash
gpg --gen-key
# Follow the prompts to create your key
```

### Exporting Public Key

```bash
gpg --export --armor your-email@example.com > public-key.asc
```

### Signing Packages

For Debian packages:
```bash
debsign tonledb_0.1.0-1_source.changes
```

For RPM packages:
```bash
rpmsign --addsign tonledb-0.1.0-1.x86_64.rpm
```

## Repository Metadata

### APT Repository (Debian/Ubuntu)

Create a proper APT repository structure:

```
repository/
├── dists/
│   └── stable/
│       ├── main/
│       │   ├── binary-amd64/
│       │   │   ├── Packages
│       │   │   └── Packages.gz
│       │   └── source/
│       │       ├── Sources
│       │       └── Sources.gz
│       └── Release
└── pool/
    └── main/
        └── t/
            └── tonledb/
                ├── tonledb_0.1.0-1_amd64.deb
                └── tonledb_0.1.0-1.dsc
```

Generate repository metadata:
```bash
# Generate Packages file
dpkg-scanpackages pool/ /dev/null | gzip -9c > dists/stable/main/binary-amd64/Packages.gz

# Generate Sources file (for source packages)
dpkg-source --quiet -b tonledb_0.1.0-1.dsc
dpkg-scansources pool/ /dev/null | gzip -9c > dists/stable/main/source/Sources.gz

# Generate Release file
apt-ftparchive release dists/stable > dists/stable/Release

# Sign Release file
gpg --clearsign -o dists/stable/Release.gpg dists/stable/Release
```

### YUM Repository (CentOS/RHEL/Fedora)

Create a proper YUM repository structure:

```
repository/
├── Packages/
│   └── tonledb-0.1.0-1.x86_64.rpm
├── repodata/
│   ├── filelists.xml.gz
│   ├── other.xml.gz
│   ├── primary.xml.gz
│   └── repomd.xml
└── comps.xml
```

Generate repository metadata:
```bash
# Create repository
createrepo --database --simple-md-filenames repository/

# Sign repository metadata
gpg --detach-sign --armor repository/repodata/repomd.xml
```

## Best Practices

1. **Versioning**: Follow semantic versioning (MAJOR.MINOR.PATCH)
2. **Changelogs**: Maintain detailed changelogs for each release
3. **Dependencies**: Clearly specify all runtime dependencies
4. **Security**: Sign all packages and repository metadata
5. **Testing**: Test packages on clean systems before publishing
6. **Documentation**: Provide clear installation and usage instructions
7. **Backward Compatibility**: Maintain backward compatibility within major versions
8. **Licensing**: Include proper licensing information in packages

## Monitoring and Maintenance

1. **Monitor Downloads**: Track package downloads and user feedback
2. **Security Updates**: Provide timely security updates
3. **Bug Fixes**: Respond to bug reports and provide updated packages
4. **Compatibility**: Ensure compatibility with new OS versions
5. **Deprecation**: Handle deprecated features gracefully

## Conclusion

Publishing TonleDB packages to official repositories significantly improves the user experience by allowing simple installation with standard package managers. The process involves:

1. Creating properly structured packages
2. Signing packages for security
3. Uploading to package repositories
4. Maintaining packages with updates and security fixes

With proper publishing, users can install TonleDB with simple commands like `sudo apt install tonledb` or `sudo dnf install tonledb`.
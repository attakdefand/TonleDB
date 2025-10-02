# GitHub Pages Setup for TonleDB

This document explains how to set up GitHub Pages for hosting the TonleDB package repository.

## Prerequisites

All the necessary files have been committed to your repository:
- GitHub Actions workflows for deployment
- Repository configuration files (tonledb.list, tonledb.repo)
- Installation scripts (install.sh, install.html)
- Documentation files

## Setup Instructions

### 1. Enable GitHub Pages

1. Go to your repository at https://github.com/attakdefand/TonleDB
2. Click on the "Settings" tab
3. In the left sidebar, scroll down and click on "Pages"
4. Under "Source", select "GitHub Actions"
5. Click "Save"

### 2. Trigger the Deployment Workflow

You can trigger the deployment in two ways:

#### Option A: Manually run the workflow
1. Go to the "Actions" tab in your repository
2. Find the "Deploy GitHub Pages" workflow
3. Click "Run workflow" and confirm

#### Option B: Make a commit to the main branch
Any commit to the main branch will automatically trigger the deployment workflow.

### 3. Verify Deployment

After the workflow completes successfully, your GitHub Pages site will be available at:
https://attakdefand.github.io/TonleDB/

You can verify that the following files are accessible:
- https://attakdefand.github.io/TonleDB/tonledb.list
- https://attakdefand.github.io/TonleDB/tonledb.repo
- https://attakdefand.github.io/TonleDB/install.sh

### 4. Create a Release (Optional but Recommended)

To make pre-built packages available:

1. Go to the "Releases" section in your repository
2. Click "Create a new release"
3. Enter a tag name (e.g., v0.1.0)
4. Enter a release title (e.g., "Initial Release")
5. Click "Publish release"

This will trigger the "Build and Release Packages" workflow that builds and uploads .deb and .rpm packages.

## Usage Instructions for Users

Once GitHub Pages is deployed, users can install TonleDB using the following methods:

### Method 1: One-Command Installation
```bash
curl -sSL https://attakdefand.github.io/TonleDB/install.sh | sudo bash
```

### Method 2: Package Manager Installation

#### For Debian/Ubuntu:
```bash
curl -sSL https://attakdefand.github.io/TonleDB/tonledb.list | sudo tee /etc/apt/sources.list.d/tonledb.list
sudo apt update
sudo apt install tonledb
```

#### For CentOS/RHEL/Fedora:
```bash
# For CentOS/RHEL
sudo yum-config-manager --add-repo https://attakdefand.github.io/TonleDB/tonledb.repo
sudo yum install tonledb

# For Fedora
sudo dnf config-manager --add-repo https://attakdefand.github.io/TonleDB/tonledb.repo
sudo dnf install tonledb
```

## Troubleshooting

### If GitHub Pages is not deploying:
1. Check that GitHub Pages is enabled in Settings > Pages
2. Verify that the "Deploy GitHub Pages" workflow ran successfully in the Actions tab
3. Ensure there are no errors in the workflow logs

### If repository files are not accessible:
1. Check that the workflow completed without errors
2. Verify that the files exist in the repository
3. Make sure the workflow is correctly copying files to the _site directory

### If package installation fails:
1. Ensure packages have been built and uploaded to a release
2. Check that the repository configuration files point to the correct locations
3. Verify that the system meets all dependencies

## Next Steps

1. Enable GitHub Pages as described above
2. Trigger the deployment workflow
3. Create a release to build and upload packages
4. Test the installation process
5. Update documentation with the live URLs

Once everything is set up, your users will be able to easily install TonleDB using standard package managers.
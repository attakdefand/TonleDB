#!/bin/bash

# Script to deploy TonleDB packages to various repositories

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
if [ ! -f "./Makefile" ]; then
    print_error "This script must be run from the packaging directory"
    exit 1
fi

# Function to deploy to GitHub Releases
deploy_github() {
    print_status "Deploying to GitHub Releases..."
    
    # Check if GitHub CLI is installed
    if ! command -v gh &> /dev/null; then
        print_error "GitHub CLI (gh) is not installed. Please install it first."
        print_warning "Installation instructions: https://github.com/cli/cli#installation"
        exit 1
    fi
    
    # Check if we're in a git repository
    if [ ! -d "../.git" ]; then
        print_error "Not in a git repository. Please run this from within the git repository."
        exit 1
    fi
    
    # Get the latest tag
    TAG=$(git describe --tags `git rev-list --tags --max-count=1` 2>/dev/null || echo "v0.1.0")
    
    print_status "Latest tag: $TAG"
    
    # Check if release exists
    if gh release view "$TAG" &>/dev/null; then
        print_status "Release $TAG already exists. Updating assets..."
        gh release upload "$TAG" *.deb *.rpm --clobber
    else
        print_status "Creating new release $TAG..."
        gh release create "$TAG" *.deb *.rpm --title "Release $TAG" --notes "TonleDB release $TAG"
    fi
    
    print_status "Packages deployed to GitHub Releases successfully!"
    echo "Users can download packages from: https://github.com/your-username/tonledb/releases/tag/$TAG"
}

# Function to deploy to a simple HTTP repository
deploy_http() {
    print_status "Deploying to HTTP repository..."
    
    # Check if rsync is installed
    if ! command -v rsync &> /dev/null; then
        print_error "rsync is not installed."
        exit 1
    fi
    
    # Check if destination is provided
    if [ -z "$DEPLOY_HOST" ] || [ -z "$DEPLOY_PATH" ]; then
        print_error "DEPLOY_HOST and DEPLOY_PATH environment variables must be set"
        print_error "Example: DEPLOY_HOST=user@server.com DEPLOY_PATH=/var/www/repo ./deploy.sh http"
        exit 1
    fi
    
    print_status "Deploying to $DEPLOY_HOST:$DEPLOY_PATH"
    
    # Create repository structure
    ssh "$DEPLOY_HOST" "mkdir -p $DEPLOY_PATH/deb $DEPLOY_PATH/rpm"
    
    # Deploy packages
    rsync -av *.deb "$DEPLOY_HOST:$DEPLOY_PATH/deb/"
    rsync -av *.rpm "$DEPLOY_HOST:$DEPLOY_PATH/rpm/"
    
    print_status "Packages deployed to HTTP repository successfully!"
    echo "Users can add the repository with:"
    echo "  # For Debian/Ubuntu:"
    echo "  echo \"deb http://$DEPLOY_HOST\$DEPLOY_PATH/deb/ /\" | sudo tee /etc/apt/sources.list.d/tonledb.list"
    echo "  sudo apt update && sudo apt install tonledb"
    echo ""
    echo "  # For CentOS/RHEL/Fedora:"
    echo "  sudo dnf config-manager --add-repo http://$DEPLOY_HOST\$DEPLOY_PATH/rpm/"
    echo "  sudo dnf install tonledb"
}

# Function to deploy to package repositories (PPA, Copr, etc.)
deploy_repos() {
    print_status "Deploying to package repositories..."
    
    # This is a placeholder - actual implementation would depend on which repositories
    # you want to publish to and would require proper authentication setup
    
    print_warning "This feature requires manual setup for each repository:"
    echo "1. Ubuntu PPA: Requires Launchpad account and GPG key"
    echo "2. Fedora Copr: Requires Fedora account and Copr CLI"
    echo "3. EPEL: Requires package review process"
    echo ""
    echo "See official-docs/PUBLISHING.md for detailed instructions."
}

# Main script logic
case "${1:-github}" in
    github)
        deploy_github
        ;;
    http)
        deploy_http
        ;;
    repos)
        deploy_repos
        ;;
    all)
        deploy_github
        deploy_http
        deploy_repos
        ;;
    *)
        echo "Usage: $0 [github|http|repos|all]"
        echo "  github - Deploy to GitHub Releases (default)"
        echo "  http   - Deploy to HTTP repository (requires DEPLOY_HOST and DEPLOY_PATH)"
        echo "  repos  - Instructions for deploying to package repositories"
        echo "  all    - Deploy to all available targets"
        exit 1
        ;;
esac
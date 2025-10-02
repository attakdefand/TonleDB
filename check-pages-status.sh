#!/bin/bash

# Script to check GitHub Pages deployment status for TonleDB

echo "Checking GitHub Pages deployment status for TonleDB..."

# Check if GitHub Pages is enabled by trying to access the site
echo "Testing GitHub Pages accessibility..."

# Test if the main page is accessible
if curl -s -f -o /dev/null https://attakdefand.github.io/TonleDB/; then
    echo "✓ GitHub Pages is accessible"
else
    echo "⚠ GitHub Pages may not be deployed yet or there's an issue"
fi

# Test if repository configuration files are accessible
echo "Testing repository configuration files..."

if curl -s -f -o /dev/null https://attakdefand.github.io/TonleDB/tonledb.list; then
    echo "✓ tonledb.list is accessible"
else
    echo "⚠ tonledb.list may not be deployed yet"
fi

if curl -s -f -o /dev/null https://attakdefand.github.io/TonleDB/tonledb.repo; then
    echo "✓ tonledb.repo is accessible"
else
    echo "⚠ tonledb.repo may not be deployed yet"
fi

if curl -s -f -o /dev/null https://attakdefand.github.io/TonleDB/install.sh; then
    echo "✓ install.sh is accessible"
else
    echo "⚠ install.sh may not be deployed yet"
fi

echo ""
echo "GitHub Pages status check complete."
echo ""
echo "If files are not accessible yet, please check:"
echo "1. GitHub Actions workflows in the 'Actions' tab"
echo "2. GitHub Pages settings in 'Settings' > 'Pages'"
echo "3. Ensure the 'Deploy GitHub Pages' workflow has run successfully"
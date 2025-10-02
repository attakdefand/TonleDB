#!/bin/bash

# Script to verify GitHub Pages setup for TonleDB

echo "Verifying GitHub Pages setup for TonleDB..."

# Check if required files exist
echo "Checking for required files..."

if [ -f "repo/tonledb.list" ]; then
    echo "✓ tonledb.list found"
else
    echo "✗ tonledb.list not found"
fi

if [ -f "repo/tonledb.repo" ]; then
    echo "✓ tonledb.repo found"
else
    echo "✗ tonledb.repo not found"
fi

if [ -f "packaging/install.sh" ]; then
    echo "✓ install.sh found"
else
    echo "✗ install.sh not found"
fi

if [ -f "tonledb/.github/workflows/pages.yml" ]; then
    echo "✓ pages.yml workflow found"
else
    echo "✗ pages.yml workflow not found"
fi

if [ -f "tonledb/.github/workflows/release.yml" ]; then
    echo "✓ release.yml workflow found"
else
    echo "✗ release.yml workflow not found"
fi

echo ""
echo "GitHub Pages setup verification complete."
echo ""
echo "To manually verify GitHub Pages deployment:"
echo "1. Go to https://github.com/attakdefand/TonleDB"
echo "2. Click on Settings > Pages"
echo "3. Ensure source is set to GitHub Actions"
echo "4. Check Actions tab for workflow status"
echo ""
echo "Once deployed, users can access:"
echo "- https://attakdefand.github.io/TonleDB/tonledb.list"
echo "- https://attakdefand.github.io/TonleDB/tonledb.repo"
echo "- https://attakdefand.github.io/TonleDB/install.sh"
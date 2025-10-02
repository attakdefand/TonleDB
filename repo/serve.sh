#!/bin/bash

# Simple script to serve the repository locally for testing

# Check if Python is available
if command -v python3 &> /dev/null; then
    PYTHON_CMD="python3"
elif command -v python &> /dev/null; then
    PYTHON_CMD="python"
else
    echo "Python is not installed. Please install Python to serve the repository."
    exit 1
fi

# Serve the repository
echo "Serving repository at http://localhost:8000"
echo "Press Ctrl+C to stop"
cd ..  # Go to the root directory to serve all files
$PYTHON_CMD -m http.server 8000
#!/bin/bash

# Setup script for TonleDB VS Code Extension

echo "Setting up TonleDB VS Code Extension..."

# Check if node is installed
if ! command -v node &> /dev/null
then
    echo "Node.js is not installed. Please install Node.js first."
    exit 1
fi

# Check if npm is installed
if ! command -v npm &> /dev/null
then
    echo "npm is not installed. Please install npm first."
    exit 1
fi

echo "Installing dependencies..."
npm install

if [ $? -eq 0 ]; then
    echo "Dependencies installed successfully."
else
    echo "Failed to install dependencies."
    exit 1
fi

echo "Compiling TypeScript..."
npm run compile

if [ $? -eq 0 ]; then
    echo "TypeScript compiled successfully."
else
    echo "Failed to compile TypeScript."
    exit 1
fi

echo "Setup complete! You can now run the extension by pressing F5 in VS Code."
#!/bin/bash

echo "Diagnosing TonleDB VS Code Extension Environment..."
echo "==================================================="

echo "1. Checking Node.js installation..."
if command -v node &> /dev/null
then
    echo "   Node.js version: $(node --version)"
else
    echo "   ERROR: Node.js is not installed or not in PATH"
    echo "   Please install Node.js from https://nodejs.org/"
fi

echo ""
echo "2. Checking npm installation..."
if command -v npm &> /dev/null
then
    echo "   npm version: $(npm --version)"
else
    echo "   ERROR: npm is not installed or not in PATH"
    echo "   Please install Node.js (which includes npm) from https://nodejs.org/"
fi

echo ""
echo "3. Checking current directory..."
echo "   Current directory: $(pwd)"
echo "   Directory contents:"
ls -la

echo ""
echo "4. Checking for package.json..."
if [ -f "package.json" ]; then
    echo "   package.json found"
    echo "   Checking for compile script..."
    if grep -q "\"compile\"" package.json; then
        echo "   Compile script found in package.json"
    else
        echo "   ERROR: Compile script not found in package.json"
    fi
else
    echo "   ERROR: package.json not found"
    echo "   Please navigate to the directory containing package.json"
fi

echo ""
echo "5. Checking for node_modules..."
if [ -d "node_modules" ]; then
    echo "   node_modules directory found"
    echo "   Dependency count: $(ls node_modules | wc -l)"
else
    echo "   node_modules directory not found"
    echo "   Run 'npm install' to install dependencies"
fi

echo ""
echo "6. Checking for TypeScript config..."
if [ -f "tsconfig.json" ]; then
    echo "   tsconfig.json found"
else
    echo "   tsconfig.json not found"
fi

echo ""
echo "==================================================="
echo "Diagnostic complete."
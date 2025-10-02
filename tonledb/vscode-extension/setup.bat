@echo off
REM Setup script for TonleDB VS Code Extension on Windows

echo Setting up TonleDB VS Code Extension...

REM Check if node is installed
node --version >nul 2>&1
if %errorlevel% neq 0 (
    echo Node.js is not installed. Please install Node.js first.
    exit /b 1
)

REM Check if npm is installed
npm --version >nul 2>&1
if %errorlevel% neq 0 (
    echo npm is not installed. Please install npm first.
    exit /b 1
)

echo Installing dependencies...
npm install

if %errorlevel% equ 0 (
    echo Dependencies installed successfully.
) else (
    echo Failed to install dependencies.
    exit /b 1
)

echo Compiling TypeScript...
npm run compile

if %errorlevel% equ 0 (
    echo TypeScript compiled successfully.
) else (
    echo Failed to compile TypeScript.
    exit /b 1
)

echo Setup complete! You can now run the extension by pressing F5 in VS Code.
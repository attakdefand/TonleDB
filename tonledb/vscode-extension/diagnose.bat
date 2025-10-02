@echo off
echo Diagnosing TonleDB VS Code Extension Environment...
echo ===================================================

echo 1. Checking Node.js installation...
node --version >nul 2>&1
if %errorlevel% equ 0 (
    for /f "tokens=*" %%i in ('node --version') do set node_version=%%i
    echo    Node.js version: %node_version%
) else (
    echo    ERROR: Node.js is not installed or not in PATH
    echo    Please install Node.js from https://nodejs.org/
)

echo.
echo 2. Checking npm installation...
npm --version >nul 2>&1
if %errorlevel% equ 0 (
    for /f "tokens=*" %%i in ('npm --version') do set npm_version=%%i
    echo    npm version: %npm_version%
) else (
    echo    ERROR: npm is not installed or not in PATH
    echo    Please install Node.js (which includes npm) from https://nodejs.org/
)

echo.
echo 3. Checking current directory...
echo    Current directory: %cd%
echo    Directory contents:
dir

echo.
echo 4. Checking for package.json...
if exist package.json (
    echo    package.json found
    echo    Checking for compile script...
    findstr /C:"\"compile\"" package.json >nul
    if %errorlevel% equ 0 (
        echo    Compile script found in package.json
    ) else (
        echo    ERROR: Compile script not found in package.json
    )
) else (
    echo    ERROR: package.json not found
    echo    Please navigate to the directory containing package.json
)

echo.
echo 5. Checking for node_modules...
if exist node_modules (
    echo    node_modules directory found
    echo    Dependency count: 
    dir node_modules /b | find /c /v ""
) else (
    echo    node_modules directory not found
    echo    Run 'npm install' to install dependencies
)

echo.
echo 6. Checking for TypeScript config...
if exist tsconfig.json (
    echo    tsconfig.json found
) else (
    echo    tsconfig.json not found
)

echo.
echo ===================================================
echo Diagnostic complete.
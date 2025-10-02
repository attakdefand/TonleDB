@echo off
REM Build script for creating optimized TonleDB binaries on Windows
REM This script builds the core TonleDB components with full optimizations

echo Building TonleDB with full optimizations...

REM Check if we're in the right directory
if not exist "Cargo.toml" (
    echo Error: Cargo.toml not found. Please run this script from the tonledb directory.
    exit /b 1
)

REM Build core components with optimizations (avoiding crates with complex dependencies)
echo Building core components with optimizations...
cargo build -p tonledb-cli --release
cargo build -p tonledb-core --release
cargo build -p tonledb-storage --release
cargo build -p tonledb-sql --release
cargo build -p tonledb-nosql-kv --release
cargo build -p tonledb-nosql-doc --release

echo Optimized build completed successfully!
echo Binaries are located in target\release\

REM List the main binaries
echo Main optimized binaries:
dir target\release\tonledb-cli.exe

echo.
echo To run the optimized CLI:
echo   target\release\tonledb-cli.exe --help
echo.
echo Note: Some advanced features may require additional build dependencies.
echo For a complete build, install cmake and nasm, then run:
echo   cargo build --release --workspace
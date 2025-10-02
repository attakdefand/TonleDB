# TonleDB VS Code Extension

This extension provides language support for TonleDB SQL in Visual Studio Code.

## Prerequisites

Before setting up the extension, ensure you have the following installed:
- Node.js (version 12 or higher) - [Download Node.js](https://nodejs.org/)
- npm (comes with Node.js)
- Visual Studio Code

### Installing Dependencies on Windows

On Windows, you might encounter PATH issues even after installing Node.js. If you get "command not found" errors:

1. Find your Node.js installation directory (typically `C:\Program Files\nodejs\`)
2. Add this directory to your system PATH environment variable:
   - Press `Win + X` and select "System"
   - Click "Advanced system settings"
   - Click "Environment Variables"
   - Under "System Variables", find and select "Path", then click "Edit"
   - Click "New" and add `C:\Program Files\nodejs\`
   - Click "OK" to save changes
3. Restart your command prompt/PowerShell

Alternatively, you can run commands directly with the full path:
```cmd
"C:\Program Files\nodejs\npm.cmd" install
"C:\Program Files\nodejs\npm.cmd" run compile
```

If you still encounter issues, try:
```cmd
$env:PATH += ";C:\Program Files\nodejs"; npm install
$env:PATH += ";C:\Program Files\nodejs"; npm run compile
```

## Setup Instructions

### 1. Install Dependencies

Navigate to the vscode-extension directory and install the required npm packages:

```bash
cd path/to/tonledb/vscode-extension
npm install
```

This will install all dependencies listed in package.json, including:
- @types/vscode
- @types/node
- typescript
- vscode-languageclient

If you encounter any issues during installation, try:
```bash
npm install --no-optional
```

### 2. Compile the Extension

Compile the TypeScript code to JavaScript:

```bash
npm run compile
```

Or for development, you can watch for changes:

```bash
npm run watch
```

If you get a "Missing script: compile" error, verify that:
1. You're in the correct directory (where package.json is located)
2. The package.json file contains the "compile" script
3. All dependencies are properly installed

### 3. Install the Extension Locally

1. Open Visual Studio Code
2. Press `Ctrl+Shift+P` to open the command palette
3. Type "Extensions: Install from VSIX" and select it
4. Navigate to your `vscode-extension` directory
5. Select the generated `.vsix` file (if you've packaged it) or:
   - Press `F5` to launch a new VS Code window with the extension loaded for testing

### 4. Alternative Installation Method

You can also install the extension in development mode:

1. Open the `vscode-extension` folder in VS Code
2. Press `F5` to launch the extension in a new Extension Development Host window

### 5. Build the Language Server

The extension requires the TonleDB language server to be built:

```bash
cd path/to/tonledb
cargo build -p tonledb-language-server
```

The extension expects the language server binary to be located at `../target/debug/tonledb-language-server` relative to the extension directory.

## Features

- Syntax highlighting for TonleDB SQL (.tdb and .tonlesql files)
- Code completion (planned)
- Error diagnostics (planned)
- Hover information (planned)
- Connection management to TonleDB instances

## Supported File Extensions

- `.tdb` - TonleDB SQL files
- `.tonlesql` - TonleDB SQL files

## Commands

- `TonleDB: Connect to Instance` - Connect to a TonleDB instance

## Troubleshooting

### Node.js Not Found

If you get "command not found" or similar errors for `node` or `npm`:

1. Ensure Node.js is installed (see installation instructions above)
2. Restart your terminal/command prompt
3. Check that Node.js is in your system PATH

On Windows, you might need to add Node.js to your PATH:
1. Find your Node.js installation directory (typically `C:\Program Files\nodejs\`)
2. Add this directory to your system PATH environment variable
3. Restart your command prompt

Alternatively, you can run commands directly with the full path:
```cmd
"C:\Program Files\nodejs\npm.cmd" install
"C:\Program Files\nodejs\npm.cmd" run compile
```

Or temporarily add Node.js to your PATH in the current session:
```powershell
$env:PATH += ";C:\Program Files\nodejs"; npm install
```

### Missing Dependencies

If you see errors about missing modules like 'vscode' or 'vscode-languageclient', make sure you've run:

```bash
npm install
```

If you still encounter issues, try:
```bash
npm install --no-optional
```

### Language Server Not Starting

If the language server doesn't start:

1. Ensure you've built the language server:
   ```bash
   cargo build -p tonledb-language-server
   ```

2. Check that the binary exists at `../target/debug/tonledb-language-server`

3. Verify the path in `extension.ts` matches your build output

### TypeScript Compilation Errors

If you see TypeScript errors:

1. Make sure you're using a compatible TypeScript version
2. Check that all dependencies are installed
3. Restart the TypeScript server in VS Code (Ctrl+Shift+P, then "TypeScript: Restart TS server")

### "Missing script: compile" Error

If you get this error:

1. Verify you're in the correct directory (where package.json is located)
2. Check that package.json contains the "compile" script
3. Try reinstalling dependencies:
   ```bash
   rm -rf node_modules package-lock.json
   npm install
   ```

### npm Audit Issues

If you see security audit warnings:

```bash
npm audit
```

To fix vulnerabilities (use with caution):
```bash
npm audit fix
```

For breaking changes:
```bash
npm audit fix --force
```

Note that audit fixes may update dependencies to versions that could break compatibility.

## Development

To modify the extension:

1. Make changes to the TypeScript files in the `src` directory
2. Run `npm run compile` to compile the changes
3. Press `F5` to test the extension

## Packaging for Distribution

To create a VSIX package for distribution:

```bash
npm install -g vsce
vsce package
```

This creates a `.vsix` file that can be installed in VS Code.
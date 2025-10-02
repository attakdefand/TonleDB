# TonleDB VS Code Extension

This extension provides language support for TonleDB SQL in Visual Studio Code.

## Features

- Syntax highlighting for TonleDB SQL
- Code completion
- Hover information
- Connection management to TonleDB instances

## Installation

1. Install the extension from the VS Code Marketplace
2. Or, build and install from source:
   ```bash
   npm install
   npm run compile
   ```

## Supported File Extensions

- `.tdb` - TonleDB SQL files
- `.tonlesql` - TonleDB SQL files

## Commands

- `TonleDB: Connect to Instance` - Connect to a TonleDB instance

## Configuration

The extension can be configured through VS Code settings:

```json
{
  "tonledb.connectionTimeout": 30000,
  "tonledb.enableDiagnostics": true
}
```

## Requirements

- VS Code version 1.60.0 or higher
- TonleDB Language Server (automatically installed with the extension)

## Development

To develop the extension:

1. Clone the repository
2. Run `npm install` to install dependencies
3. Open the project in VS Code
4. Press F5 to launch the extension in a new VS Code window

## Building

To build the extension for distribution:

```bash
npm install
npm run compile
vsce package
```
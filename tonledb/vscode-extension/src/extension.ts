import * as vscode from 'vscode';
import * as path from 'path';
import { LanguageClient, LanguageClientOptions, ServerOptions, TransportKind } from 'vscode-languageclient/node';

let client: LanguageClient | undefined;

function startLanguageClient(context: vscode.ExtensionContext) {
    try {
        // Server module path
        const serverModule = context.asAbsolutePath(
            path.join('..', 'target', 'debug', 'tonledb-language-server')
        );
        
        // Server options
        const serverOptions: ServerOptions = {
            run: { module: serverModule, transport: TransportKind.stdio },
            debug: {
                module: serverModule,
                transport: TransportKind.stdio,
            }
        };
        
        // Client options
        const clientOptions: LanguageClientOptions = {
            documentSelector: [{ scheme: 'file', language: 'tonledb-sql' }],
            synchronize: {
                fileEvents: vscode.workspace.createFileSystemWatcher('**/*.tdb', false, false, false)
            }
        };
        
        // Create the language client and start the client
        client = new LanguageClient(
            'tonledbLanguageServer',
            'TonleDB Language Server',
            serverOptions,
            clientOptions
        );
        
        // Start the client
        client.start();
    } catch (error) {
        vscode.window.showErrorMessage(`Failed to start TonleDB Language Server: ${error}`);
    }
}

export function activate(context: vscode.ExtensionContext) {
    // Register the language server
    startLanguageClient(context);
    
    // Register command for connecting to TonleDB instances
    const connectCommand = vscode.commands.registerCommand('tonledb.connect', () => {
        vscode.window.showInputBox({
            prompt: 'Enter TonleDB connection string',
            placeHolder: 'e.g., localhost:5432'
        }).then((connectionString: string | undefined) => {
            if (connectionString) {
                // Here we would implement the actual connection logic
                vscode.window.showInformationMessage(`Connecting to TonleDB at ${connectionString}`);
            }
        });
    });
    
    context.subscriptions.push(connectCommand);
}

export function deactivate(): Promise<void> | undefined {
    if (!client) {
        return undefined;
    }
    return client.stop();
}
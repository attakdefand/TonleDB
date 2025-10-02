use tonledb_language_server::{LanguageServerManager, ConnectionManager, ConnectionConfig};
use tower_lsp::jsonrpc::Result;

#[tokio::main]
async fn main() -> Result<()> {
    // Example of how to use the connection manager
    let connection_manager = ConnectionManager::new();
    
    let config = ConnectionConfig {
        host: "localhost".to_string(),
        port: 5432,
        database: "tonledb".to_string(),
        username: "user".to_string(),
        password: None,
    };
    
    connection_manager.add_connection("default".to_string(), config).await.unwrap();
    
    // Run the language server
    LanguageServerManager::run().await
}
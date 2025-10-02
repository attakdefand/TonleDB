use tower_lsp::{Client, LanguageServer, LspService, Server};
use tower_lsp::jsonrpc::Result;
use tower_lsp::lsp_types::*;
use std::sync::Arc;
use tokio::io::{stdin, stdout};

mod connection;
pub use connection::{ConnectionManager, ConnectionConfig};

#[derive(Debug)]
struct Backend {
    client: Client,
    connection_manager: Arc<ConnectionManager>,
}

#[tower_lsp::async_trait]
impl LanguageServer for Backend {
    async fn initialize(&self, _: InitializeParams) -> Result<InitializeResult> {
        Ok(InitializeResult {
            server_info: None,
            capabilities: ServerCapabilities {
                text_document_sync: Some(TextDocumentSyncCapability::Kind(
                    TextDocumentSyncKind::FULL,
                )),
                completion_provider: Some(CompletionOptions {
                    resolve_provider: Some(false),
                    trigger_characters: Some(vec![".".to_string()]),
                    work_done_progress_options: Default::default(),
                    all_commit_characters: None,
                    completion_item: None,
                }),
                hover_provider: Some(HoverProviderCapability::Simple(true)),
                ..ServerCapabilities::default()
            },
        })
    }

    async fn initialized(&self, _: InitializedParams) {
        self.client
            .log_message(MessageType::INFO, "TonleDB Language Server initialized")
            .await;
    }

    async fn shutdown(&self) -> Result<()> {
        Ok(())
    }

    async fn did_change(&self, _: DidChangeTextDocumentParams) {
        // Handle document changes for syntax checking and diagnostics
    }

    async fn completion(&self, _: CompletionParams) -> Result<Option<CompletionResponse>> {
        // Provide completion suggestions
        Ok(None)
    }

    async fn hover(&self, _: HoverParams) -> Result<Option<Hover>> {
        // Provide hover information
        Ok(None)
    }
}

pub struct LanguageServerManager;

impl LanguageServerManager {
    pub async fn run() -> Result<()> {
        let connection_manager = Arc::new(ConnectionManager::new());
        let (service, socket) = LspService::new(|client| Backend { 
            client, 
            connection_manager 
        });
        Server::new(stdin(), stdout(), socket).serve(service).await;
        Ok(())
    }
}
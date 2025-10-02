//! Postgres wire protocol compatibility for TonleDB

use tokio::net::TcpListener;
use tokio::io::{AsyncReadExt, AsyncWriteExt};
use tonledb_core::Db;
use std::sync::Arc;

/// PostgreSQL wire protocol message types
#[derive(Debug)]
pub enum PgMessage {
    StartupMessage {
        version: i32,
        parameters: Vec<(String, String)>,
    },
    Query {
        query: String,
    },
    Terminate,
}

/// Parse a PostgreSQL wire protocol message
pub async fn parse_pg_message(stream: &mut tokio::net::TcpStream) -> Result<PgMessage, anyhow::Error> {
    // Read message type
    let mut type_byte = [0u8; 1];
    stream.read_exact(&mut type_byte).await?;
    
    match type_byte[0] {
        b'Q' => {
            // Query message
            let mut len_bytes = [0u8; 4];
            stream.read_exact(&mut len_bytes).await?;
            let len = i32::from_be_bytes(len_bytes) as usize;
            
            let mut query_bytes = vec![0u8; len - 4];
            stream.read_exact(&mut query_bytes).await?;
            
            let query = String::from_utf8(query_bytes)?;
            Ok(PgMessage::Query { query })
        }
        b'X' => {
            // Terminate message
            let mut len_bytes = [0u8; 4];
            stream.read_exact(&mut len_bytes).await?;
            Ok(PgMessage::Terminate)
        }
        _ => {
            // For other messages, just read the length and skip
            let mut len_bytes = [0u8; 4];
            stream.read_exact(&mut len_bytes).await?;
            let len = i32::from_be_bytes(len_bytes) as usize;
            
            let mut skip_bytes = vec![0u8; len - 4];
            stream.read_exact(&mut skip_bytes).await?;
            
            // Recursively parse the next message
            parse_pg_message(stream).await
        }
    }
}

/// Send a PostgreSQL response message
pub async fn send_pg_response(stream: &mut tokio::net::TcpStream, message: &str) -> Result<(), anyhow::Error> {
    // Send a simple response
    let response = format!("{}\0", message);
    let len = (response.len() + 4) as i32;
    
    stream.write_all(&[b'T']).await?; // RowDescription message type
    stream.write_all(&len.to_be_bytes()).await?;
    stream.write_all(response.as_bytes()).await?;
    
    Ok(())
}

/// Handle a PostgreSQL client connection
pub async fn handle_pg_connection(mut stream: tokio::net::TcpStream, db: Arc<Db>) -> Result<(), anyhow::Error> {
    loop {
        match parse_pg_message(&mut stream).await {
            Ok(PgMessage::Query { query }) => {
                // Execute the query using TonleDB's SQL engine
                match tonledb_sql::execute_sql(&db, &query) {
                    Ok(result) => {
                        let result_str = serde_json::to_string(&result)?;
                        send_pg_response(&mut stream, &result_str).await?;
                    }
                    Err(e) => {
                        send_pg_response(&mut stream, &format!("Error: {}", e)).await?;
                    }
                }
            }
            Ok(PgMessage::Terminate) => {
                break;
            }
            Err(e) => {
                eprintln!("Error parsing PG message: {}", e);
                break;
            }
        }
    }
    
    Ok(())
}

/// Start a PostgreSQL wire protocol server
pub async fn start_pg_server(db: Arc<Db>, bind_addr: &str) -> Result<(), anyhow::Error> {
    let listener = TcpListener::bind(bind_addr).await?;
    println!("PostgreSQL wire protocol server listening on {}", bind_addr);
    
    loop {
        let (stream, addr) = listener.accept().await?;
        println!("New PostgreSQL client connected from {}", addr);
        
        let db_clone = db.clone();
        tokio::spawn(async move {
            if let Err(e) = handle_pg_connection(stream, db_clone).await {
                eprintln!("Error handling PostgreSQL connection: {}", e);
            }
        });
    }
}
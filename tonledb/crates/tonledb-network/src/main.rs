use std::{net::SocketAddr, sync::Arc};
use axum::{routing::{get, post}, Router, extract::State, Json};
use serde::Deserialize;
use tonledb_core::Db;
use base64::{Engine as _, engine::general_purpose};
use figment::providers::Format;

mod auth;
mod audit;

#[derive(Clone)]
struct AppState { db: Arc<Db>, auth: auth::AppAuth }

#[derive(Deserialize)]
struct ConfServer { bind:String }
#[derive(Deserialize)]
struct ConfAuth { mode:String, token_file:String }
#[derive(Deserialize)]
struct ConfStorage { wal_path:String }
#[derive(Deserialize)]
struct Conf { server:ConfServer, auth:ConfAuth, storage:ConfStorage }

#[derive(Deserialize)]
struct SqlBody { sql: String }

#[tokio::main]
async fn main()->anyhow::Result<()>{
    tonledb_metrics::init_tracing_and_metrics("info");
    let cfg: Conf = figment::Figment::new()
        .merge(figment::providers::Toml::file("tonledb.toml"))
        .merge(figment::providers::Env::prefixed("TLDB_"))
        .extract()?;

    // Storage base (existing in-mem+WAL)
    let base = tonledb_storage::arc_inmem_with_wal(Some(&cfg.storage.wal_path), 100_000);
    let storage: Arc<dyn tonledb_core::Storage> = base;

    let db = Arc::new(tonledb_core::Db::new(storage));
    let tokens = auth::TokenStore::from_file(&cfg.auth.token_file).unwrap_or_else(|_| auth::TokenStore::default());
    let mode = match cfg.auth.mode.as_str(){ "token"=>auth::AuthMode::Token, _=>auth::AuthMode::None };
    let app_auth = auth::AppAuth{ tokens, mode };

    let app = Router::new()
        .route("/health", get(|| async {"ok"}))
        .route("/metrics", get(tonledb_metrics::axum_handler::metrics))
        .route("/sql", post(sql_handler))
        .route("/kv/:key", get(kv_get).post(kv_put))
        .route("/doc/:col", post(doc_insert))
        .with_state(AppState{ db, auth: app_auth.clone() });

    let addr: SocketAddr = cfg.server.bind.parse()?;
    tracing::warn!("TLS disabled (dev only).");
    let listener = tokio::net::TcpListener::bind(addr).await?;
    tracing::info!(%addr, "TonleDB listening (HTTP)");
    axum::serve(listener, app).await?;
    Ok(())
}

async fn sql_handler(State(app):State<AppState>, user:auth::User, Json(p):Json<SqlBody>)->Json<serde_json::Value>{
    if !auth::require(auth::Role::ReadWrite, &user.0.role) { return Json(serde_json::json!({"error":"forbidden"})); }
    let t = tonledb_metrics::QueryTimer::start("sql");
    let res = tonledb_sql::execute_sql(&app.db, &p.sql).map_err(|e| serde_json::json!({"error":e.to_string()})).unwrap_or_else(|e|e);
    t.stop();
    audit::log(&audit::AuditEvent{ ts: &chrono::Utc::now().to_rfc3339(), who: &user.0.name, action:"SQL", resource:"/sql", result:"ok" });
    Json(res)
}

use axum::extract::Path;
async fn kv_get(State(app):State<AppState>, user:auth::User, Path(key):Path<String>)->Json<serde_json::Value>{
    if !auth::require(auth::Role::ReadOnly, &user.0.role){ return Json(serde_json::json!({"error":"forbidden"})); }
    let v = tonledb_nosql_kv::get(&*app.db.storage, key.as_bytes()).unwrap();
    Json(match v { Some(b)=>serde_json::json!({"value": general_purpose::STANDARD.encode(b)}), None=>serde_json::json!({"value":null}) })
}
async fn kv_put(State(app):State<AppState>, user:auth::User, Path(key):Path<String>, body:String)->Json<serde_json::Value>{
    if !auth::require(auth::Role::ReadWrite, &user.0.role){ return Json(serde_json::json!({"error":"forbidden"})); }
    tonledb_nosql_kv::put(&*app.db.storage, key.into_bytes(), body.into_bytes()).unwrap();
    Json(serde_json::json!({"ok":true}))
}
async fn doc_insert(State(app):State<AppState>, user:auth::User, Path(col):Path<String>, Json(doc):Json<serde_json::Value>)->Json<serde_json::Value>{
    if !auth::require(auth::Role::ReadWrite, &user.0.role){ return Json(serde_json::json!({"error":"forbidden"})); }
    let id = tonledb_nosql_doc::insert(&*app.db.storage, &col, doc).unwrap();
    Json(serde_json::json!({"id":id}))
}
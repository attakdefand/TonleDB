use std::{net::SocketAddr, sync::Arc, time::Duration};
use axum::{routing::{get, post, delete}, Router, extract::State, Json};
use serde::Deserialize;
use tower::ServiceBuilder;
use tower_governor::{GovernorLayer, governor::{GovernorConfigBuilder, PeerIpKeyExtractor}};
use tower_http::{cors::CorsLayer, limit::RequestBodyLimitLayer, timeout::TimeoutLayer};
use tonledb_core::Db;

mod tls;
mod auth;
mod audit;

#[derive(Clone)]
struct AppState { db: Arc<Db>, auth: auth::AppAuth }

#[derive(Deserialize)]
struct ConfServer { bind:String, require_tls:bool, idle_timeout_ms:u64, request_body_limit_bytes:usize, rate_limit_rps:u32, rate_limit_burst:u32 }
#[derive(Deserialize)]
struct ConfTLS { enabled:bool, cert_path:String, key_path:String, require_client_auth:bool, ca_path:Option<String> }
#[derive(Deserialize)]
struct ConfAuth { mode:String, token_file:String }
#[derive(Deserialize)]
struct ConfStorage { encrypt_at_rest:bool, kek_env:String, wal_path:String }
#[derive(Deserialize)]
struct Conf { server:ConfServer, tls:ConfTLS, auth:ConfAuth, storage:ConfStorage }

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
    // Optional encryption wrapper
    let storage: Arc<dyn tonledb_core::Storage> = if cfg.storage.encrypt_at_rest {
        let kek = std::env::var(&cfg.storage.kek_env).expect("KEK env not set");
        let inner = tonledb_storage::arc_inmem_with_wal(Some(&cfg.storage.wal_path), 100_000);
        let crypt = tonledb_storage::crypto::CryptoStorage::new((*inner).clone(), &kek).expect("crypto init");
        Arc::new(crypt)
    } else { base };

    let db = Arc::new(tonledb_core::Db::new(storage));
    let tokens = auth::TokenStore::from_file(&cfg.auth.token_file).unwrap_or_else(|_| auth::TokenStore{ map: Default::default() });
    let mode = match cfg.auth.mode.as_str(){ "token"=>auth::AuthMode::Token, _=>auth::AuthMode::None };
    let app_auth = auth::AppAuth{ tokens, mode };

    let cors = CorsLayer::permissive(); // tighten for prod
    let limit = RequestBodyLimitLayer::new(cfg.server.request_body_limit_bytes as u64);
    let timeout = TimeoutLayer::new(Duration::from_millis(cfg.server.idle_timeout_ms));
    
    // Rate limiting configuration
    let governor_conf = Box::new(
        GovernorConfigBuilder::default()
            .per_second(cfg.server.rate_limit_rps)
            .burst_size(cfg.server.rate_limit_burst)
            .key_extractor(PeerIpKeyExtractor)
            .finish()
            .unwrap()
    );
    let governor = GovernorLayer {
        config: governor_conf,
    };

    let svc_stack = ServiceBuilder::new()
        .layer(governor)
        .layer(cors)
        .layer(limit)
        .layer(timeout);

    let app = Router::new()
        .route("/health", get(|| async {"ok"}))
        .route("/metrics", get(tonledb_metrics::axum_handler::metrics))
        .route("/sql", post(sql_handler))
        .route("/kv/:key", get(kv_get).post(kv_put).delete(kv_del))
        .route("/doc/:col", post(doc_insert))
        .with_state(AppState{ db, auth: app_auth.clone() })
        .layer(axum::middleware::from_fn(move |mut req, next| {
            let authc = app_auth.clone();
            async move { req.extensions_mut().insert(authc); Ok::<_, axum::http::StatusCode>(next.run(req).await) }
        }))
        .layer(svc_stack);

    let addr: SocketAddr = cfg.server.bind.parse()?;
    if cfg.tls.enabled {
        let tls_cfg = tls::tls_config(&cfg.tls.cert_path, &cfg.tls.key_path, cfg.tls.ca_path.as_deref(), cfg.tls.require_client_auth)?;
        let listener = tokio::net::TcpListener::bind(addr).await?;
        tracing::info!(%addr, "TonleDB listening (HTTPS)");
        loop {
            let (stream, _) = listener.accept().await?;
            let tls_cfg = tls_cfg.clone();
            let app = app.clone();
            tokio::spawn(async move {
                let acceptor = tokio_rustls::TlsAcceptor::from(tls_cfg);
                if let Ok(tls_stream) = acceptor.accept(stream).await {
                    if let Err(e)=hyper_util::server::conn::auto::Builder::new(hyper_util::rt::TokioExecutor::new())
                        .serve_connection_with_upgrades(hyper_util::rt::TokioIo::new(tls_stream), app.into_make_service())
                        .await { tracing::error!(?e, "serve_connection"); }
                }
            });
        }
    } else {
        tracing::warn!("TLS disabled (dev only).");
        axum::Server::bind(&addr).serve(app.into_make_service()).await?;
    }
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
    Json(match v { Some(b)=>serde_json::json!({"value": base64::encode(b)}), None=>serde_json::json!({"value":null}) })
}
async fn kv_put(State(app):State<AppState>, user:auth::User, Path(key):Path<String>, body:String)->Json<serde_json::Value>{
    if !auth::require(auth::Role::ReadWrite, &user.0.role){ return Json(serde_json::json!({"error":"forbidden"})); }
    tonledb_nosql_kv::put(&*app.db.storage, key.into_bytes(), body.into_bytes()).unwrap();
    Json(serde_json::json!({"ok":true}))
}
async fn kv_del(State(app):State<AppState>, user:auth::User, Path(key):Path<String>)->Json<serde_json::Value>{
    if !auth::require(auth::Role::ReadWrite, &user.0.role){ return Json(serde_json::json!({"error":"forbidden"})); }
    tonledb_nosql_kv::del(&*app.db.storage, key.as_bytes()).unwrap();
    Json(serde_json::json!({"ok":true}))
}
async fn doc_insert(State(app):State<AppState>, user:auth::User, Path(col):Path<String>, Json(doc):Json<serde_json::Value>)->Json<serde_json::Value>{
    if !auth::require(auth::Role::ReadWrite, &user.0.role){ return Json(serde_json::json!({"error":"forbidden"})); }
    let id = tonledb_nosql_doc::insert(&*app.db.storage, &col, doc).unwrap();
    Json(serde_json::json!({"id":id}))
}

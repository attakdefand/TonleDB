use once_cell::sync::Lazy;
use prometheus::{
    Encoder, HistogramOpts, HistogramVec, IntCounterVec, Opts, Registry, TextEncoder,
};
use std::time::Duration;
use tracing_subscriber::{fmt, EnvFilter};

/// Global Prometheus registry
static REGISTRY: Lazy<Registry> = Lazy::new(Registry::new);

/// Common DB metrics
static HTTP_REQS: Lazy<IntCounterVec> = Lazy::new(|| {
    IntCounterVec::new(
        Opts::new("tonledb_http_requests_total", "HTTP requests by method, path, status"),
        &["method", "path", "status"],
    )
    .unwrap()
});

static WAL_APPENDS: Lazy<IntCounterVec> = Lazy::new(|| {
    IntCounterVec::new(
        Opts::new("tonledb_wal_appends_total", "WAL append operations"),
        &["result"], // "ok" | "err"
    )
    .unwrap()
});

static QUERY_LATENCY: Lazy<HistogramVec> = Lazy::new(|| {
    HistogramVec::new(
        HistogramOpts::new(
            "tonledb_query_latency_seconds",
            "SQL query latency histogram",
        )
        // Prometheus default-ish buckets for short requests
        .buckets(vec![
            0.001, 0.002, 0.005, 0.01, 0.02, 0.05,
            0.1, 0.2, 0.5, 1.0, 2.0,
        ]),
        &["type"], // "select" | "insert" | "ddl" | "other"
    )
    .unwrap()
});

/// Initialize tracing and register metrics. Idempotent.
pub fn init_tracing_and_metrics(default_level: &str) {
    // Tracing
    let _ = fmt()
        .with_env_filter(EnvFilter::try_from_default_env().unwrap_or_else(|_| {
            EnvFilter::new(default_level)
        }))
        .try_init();

    // Metrics registration (ignore AlreadyReg errors)
    let _ = REGISTRY.register(Box::new(HTTP_REQS.clone()));
    let _ = REGISTRY.register(Box::new(WAL_APPENDS.clone()));
    let _ = REGISTRY.register(Box::new(QUERY_LATENCY.clone()));
}

/// Observe one HTTP request
pub fn observe_http_request(method: &str, path: &str, status: u16) {
    HTTP_REQS
        .with_label_values(&[method, path, &status.to_string()])
        .inc();
}

/// Observe WAL append result ("ok" | "err")
pub fn observe_wal_append(result: &str) {
    WAL_APPENDS.with_label_values(&[result]).inc();
}

/// Time a query and record latency under `kind`
pub struct QueryTimer {
    start: std::time::Instant,
    kind: String,
}
impl QueryTimer {
    pub fn start(kind: &str) -> Self {
        Self {
            start: std::time::Instant::now(),
            kind: kind.to_string(),
        }
    }
    pub fn stop(self) {
        let e = self.start.elapsed();
        let secs = duration_to_secs(e);
        QUERY_LATENCY
            .with_label_values(&[&self.kind])
            .observe(secs);
    }
}
fn duration_to_secs(d: Duration) -> f64 {
    d.as_secs() as f64 + f64::from(d.subsec_nanos()) / 1_000_000_000.0
}

/// Encode all metrics in Prometheus text format
pub fn gather_prometheus() -> String {
    let mf = REGISTRY.gather();
    let mut buf = Vec::new();
    TextEncoder::new()
        .encode(&mf, &mut buf)
        .expect("encode metrics");
    String::from_utf8(buf).unwrap_or_default()
}

#[cfg(feature = "axum")]
pub mod axum_handler {
    use super::gather_prometheus;
    use axum::{response::IntoResponse, response::Response};
    use axum::http::header;

    /// `GET /metrics` — returns Prometheus text format
    pub async fn metrics() -> Response {
        let body = gather_prometheus();
        (
            [(header::CONTENT_TYPE, "text/plain; version=0.0.4; charset=utf-8")],
            body,
        )
            .into_response()
    }
}

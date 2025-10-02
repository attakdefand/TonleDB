#!/usr/bin/env bash
set -euo pipefail
echo "== Format & Clippy =="
cargo fmt --all -- --check
RUSTFLAGS="-D warnings" cargo clippy --all-targets --all-features -- -D warnings
echo "== Build & Test =="
cargo build --release
cargo test --all --release
echo "== Audit/License/Vet =="
cargo audit || true
cargo deny check || true
cargo vet || true
echo "== SBOM/Scan (dir) =="
command -v syft >/dev/null && syft packages dir:. -o cyclonedx-json > sbom.json || true
command -v grype >/dev/null && grype dir:. --fail-on medium || true
echo "== Done =="

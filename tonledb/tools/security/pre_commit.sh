#!/usr/bin/env bash
set -euo pipefail
echo "[pre-commit] fmt + clippy + deny warnings"
cargo fmt --all -- --check
RUSTFLAGS="-D warnings" cargo clippy --all-targets --all-features -- -D warnings

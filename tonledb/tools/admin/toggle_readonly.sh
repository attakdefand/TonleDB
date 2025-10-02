#!/usr/bin/env bash
set -euo pipefail
BIN=${BIN:-./target/release/tonledb}
MODE=${1:-on} # on|off
if [ "$MODE" = "on" ]; then
  $BIN config set server.readonly true
else
  $BIN config set server.readonly false
fi

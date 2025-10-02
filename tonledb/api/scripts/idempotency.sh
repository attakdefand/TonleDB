#!/usr/bin/env bash
# Usage: ./api/scripts/idempotency.sh <key>
curl -isk -H "Idempotency-Key: ${1:?missing}" -H "Content-Type: application/json" \
  -d '{"key":"alpha","value":"beta"}' https://localhost:8444/kv

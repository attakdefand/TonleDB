#!/usr/bin/env bash
set -euo pipefail
docker run --rm -t -u zap -v "$PWD:/zap/wrk" ghcr.io/zaproxy/zaproxy:stable \
  zap-baseline.py -t http://host.docker.internal:8444 -r zap_report.html || true
echo "Report: ./zap_report.html"

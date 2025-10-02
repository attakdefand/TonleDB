#!/usr/bin/env bash
set -euo pipefail
# Usage: ./scripts/s3-object-lock.sh <bucket> <local-file> <days-retain>
bucket="$1"; file="$2"; days="${3:-7}"
key="$(basename "$file")"
aws s3api put-object \
  --bucket "$bucket" \
  --key "$key" \
  --body "$file" \
  --object-lock-mode GOVERNANCE \
  --object-lock-retain-until-date "$(date -u -d "+$days days" +"%Y-%m-%dT%H:%M:%SZ")"
echo "Uploaded with retention. Bucket=$bucket Key=$key Days=$days"

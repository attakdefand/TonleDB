#!/usr/bin/env bash
# Append your signature/approval to a pending operation file
set -euo pipefail
OP_FILE=${1:?usage: approve_op.sh <op.json>}
sig="$(whoami)-$(date +%s)"
jq --arg sig "$sig" '.context.approvals += 1 | .context.signers += [$sig]' "$OP_FILE" | sponge "$OP_FILE"
jq '.context' "$OP_FILE"

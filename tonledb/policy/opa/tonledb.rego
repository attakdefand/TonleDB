package tonledb

# Default deny.
default allow = false

# Input shape we expect the DB to send the gatekeeper:
# {
#   "subject": { "sub": "...", "roles": ["operator"], "tenants": ["TENANT_A"] },
#   "action": "backup:create",    # or "kv:write", "compaction:run", etc.
#   "resource": { "tenant": "TENANT_A", "path": "/orders/..." },
#   "context": { "approvals": 2 } # optional, for 4-eyes ops
# }

# 1) Role-based permission check (simple)
allow {
  some r
  r := input.subject.roles[_]
  perm := input.action
  allowed := data.roles[r][_]
  allowed == perm
}

# 2) Wildcards in role permission map
allow {
  some r
  r := input.subject.roles[_]
  prefix := split(input.action, ":")[0]
  data.roles[r][_] == sprintf("%s:*", [prefix])
}

# 3) Sensitive operations require 2 approvals (4-eyes)
allow {
  input.action == "backup:restore"  # add more if needed
  input.context.approvals >= 2
  base_allow
}

# Base allow = role check only (used by rule 3)
base_allow {
  some r
  r := input.subject.roles[_]
  perm := input.action
  data.roles[r][_] == perm
} else {
  some r
  r := input.subject.roles[_]
  prefix := split(input.action, ":")[0]
  data.roles[r][_] == sprintf("%s:*", [prefix])
}

# 4) Tenant isolation: subject must own the target tenant
allow {
  allowed_by_rbac
  input.resource.tenant == input.subject.tenants[_]
}

# Helper: RBAC allow (used by isolation rule)
allowed_by_rbac {
  base_allow
}

# ====== Data section: role → permissions
# Keep this in the same bundle for simplicity; you can split to data.json later.
# Expand as you grow.
roles = {
  "super_admin": ["kv:*","metrics:*","policy:*","backup:create","backup:restore","compaction:run","config:write"],
  "operator":    ["kv:read","kv:write","compaction:run","snapshot:read","metrics:read"],
  "readonly":    ["kv:read","metrics:read"]
}

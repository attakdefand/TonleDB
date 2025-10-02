
** the roadmap lands.**

# What it can power

## Right now (MVP you have)

* **Lightweight OLTP apps** (small/medium CRUD)

  * Example: users, orders, settings with simple `CREATE/INSERT/SELECT … WHERE =`.
* **Key–Value config/feature flags**

  * Example: `/kv/feature/my-flag → "on"`, read from services at startup.
* **Document collections** for flexible JSON blobs

  * Example: a TODO app, small CMS docs, lightweight profiles.
* **Serverless/edge microservices state**

  * Small stateful endpoints with simple durability (WAL-backed in-mem).

### Concrete examples

* **SQL**: minimal “accounts” table for a side project

  * `CREATE TABLE accounts(id INT, name TEXT); INSERT …; SELECT * FROM accounts WHERE name="ann";`
* **KV**: centralized config store

  * `PUT /kv/ui.theme = "dark"`, `GET /kv/ui.theme`
* **Doc**: JSON feed

  * `POST /doc/posts {"title":"hello","tags":["rust","db"]}` then `GET` by id

## Near-term (M1–M3)

*As you add the page store + MVCC + indexes + richer query engine:*

* **Serious OLTP** for product features

  * User management, billing rows, audit trails, idempotent inserts.
* **Searchable document apps**

  * JSON field indexes (e.g., `user.address.city = "Phnom Penh"`), TTL for expiring docs/sessions.
* **Event sourcing / append streams**

  * Durable WAL; logical snapshots; changefeeds for consumers.
* **Light time-series / telemetry**

  * With TTL + secondary indexes, store metrics/activities by time buckets.
* **Embedded/single-binary data plane**

  * Bundle TonleDB with services for local-first apps or on-prem installs.

## Mid-term (M4–M5)

*With full transactions/constraints + Raft replication:*

* **Multi-tenant SaaS core**

  * Strong isolation, FK integrity, migrations with FK/UNIQUE/CHECK.
* **Highly available stateful services**

  * Raft leader election, read replicas, rolling upgrades.
* **Operational data stores** behind APIs

  * API gateways, auth/user sessions, rate limit counters, quotas.

## Longer-term (M6–M7)

*With Arrow/Parquet + vectorized ops + security & wire protocols:*

* **Hybrid serving + light analytics**

  * Fresh operational data + fast projections to Arrow/Parquet.
* **Interoperability with ecosystems**

  * Postgres/Mongo wire compatibility (drop-in adapters), BI tools reading Parquet.
* **Regulated workloads**

  * TLS, RBAC, row-level security, audit trails, PITR backups.

# Which workloads fit best?

| Fit                               | Examples                                      | Notes                                                                                |
| --------------------------------- | --------------------------------------------- | ------------------------------------------------------------------------------------ |
| ✅ **Transactional CRUD (OLTP)**   | Orders, users, sessions, configs              | Strong fit as SQL features grow (indexes/constraints/MVCC).                          |
| ✅ **KV/Doc apps**                 | Feature flags, JSON content, IoT device state | Great starting point now; gets better with field indexes/TTL.                        |
| ✅ **Event streams / changefeeds** | Audit logs, outbox events                     | Begin with WAL tail; later, Raft + snapshots.                                        |
| ⚠️ **Heavy analytics (OLAP)**     | Massive joins, huge scans                     | Roadmap item (Arrow/Parquet + vectorized engine). Use external warehouse until then. |
| ⚠️ **Time-series at huge scale**  | Billions of points/sec                        | Possible later with LSM + compactions. Start small or integrate with TSDB.           |

# Example mini blueprints

* **Feature Flags Service (KV)**

  * Write: `PUT /kv/flags/my_feature="enabled"`
  * Read: `GET /kv/flags/my_feature` from all microservices
  * Roadmap bonus: watch streams → services auto-refresh on change.

* **User Profiles (Doc)**

  * `POST /doc/users {"email":"a@b","plan":"pro","meta":{…}}`
  * Index fields `email`, `plan` (M2) for fast filtering & paging.

* **Orders (SQL)**

  * `CREATE TABLE orders(id INT, user_id INT, total DOUBLE);`
  * Add B-Tree index on `user_id` (M2), FK to `users(id)` (M4), transactions over multi-statement flows (M4).

* **Audit/Event Log**

  * Append-only doc/kv keys with time-prefix.
  * TTL to expire old entries; snapshots/backups for retention.

# Why use TonleDB here?

* **One engine for two data shapes** (relational + document) reduces ops sprawl.
* **Rust safety + performance** for embedded or server use.
* **Clear path** from “works today” → “production-ready features” (MVCC, indexes, replication, backups).

---



# ---- Build stage ----
FROM rust:1.80 as build
WORKDIR /app

# Caching trick: copy manifests first
COPY Cargo.toml ./
COPY crates/tonledb-core/Cargo.toml       crates/tonledb-core/Cargo.toml
COPY crates/tonledb-wal/Cargo.toml        crates/tonledb-wal/Cargo.toml
COPY crates/tonledb-storage/Cargo.toml    crates/tonledb-storage/Cargo.toml
COPY crates/tonledb-sql/Cargo.toml        crates/tonledb-sql/Cargo.toml
COPY crates/tonledb-nosql-kv/Cargo.toml   crates/tonledb-nosql-kv/Cargo.toml
COPY crates/tonledb-nosql-doc/Cargo.toml  crates/tonledb-nosql-doc/Cargo.toml
COPY crates/tonledb-network/Cargo.toml    crates/tonledb-network/Cargo.toml
COPY crates/tonledb-cli/Cargo.toml        crates/tonledb-cli/Cargo.toml
COPY crates/tonledb-metrics/Cargo.toml    crates/tonledb-metrics/Cargo.toml
COPY crates/tonledb-backup/Cargo.toml     crates/tonledb-backup/Cargo.toml

# Build deps first (empty src) to leverage Docker layer cache
RUN mkdir -p crates/tonledb-core/src \
    crates/tonledb-wal/src \
    crates/tonledb-storage/src \
    crates/tonledb-sql/src \
    crates/tonledb-nosql-kv/src \
    crates/tonledb-nosql-doc/src \
    crates/tonledb-network/src \
    crates/tonledb-cli/src \
    crates/tonledb-metrics/src \
    crates/tonledb-backup/src \
 && echo "fn main() {}" > crates/tonledb-network/src/main.rs \
 && echo "" > crates/tonledb-core/src/lib.rs \
 && echo "" > crates/tonledb-wal/src/lib.rs \
 && echo "" > crates/tonledb-storage/src/lib.rs \
 && echo "" > crates/tonledb-sql/src/lib.rs \
 && echo "" > crates/tonledb-nosql-kv/src/lib.rs \
 && echo "" > crates/tonledb-nosql-doc/src/lib.rs \
 && echo "" > crates/tonledb-cli/src/main.rs \
 && echo "" > crates/tonledb-metrics/src/lib.rs \
 && echo "" > crates/tonledb-backup/src/lib.rs \
 && cargo build -p tonledb-network --release || true

# Now copy full sources and do the real build
COPY . .
RUN cargo build --release -p tonledb-network

# ---- Runtime stage ----
FROM gcr.io/distroless/cc
# If you need CA certs for HTTPS out, use gcr.io/distroless/cc-debian12 and add /etc/ssl/certs
COPY --from=build /app/target/release/tonledb-network /bin/tonledb
EXPOSE 8383
ENTRYPOINT ["/bin/tonledb"]

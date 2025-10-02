Name:           tonledb
Version:        0.2.0
Release:        1%{?dist}
Summary:        TonleDB - A hybrid SQL/NoSQL database with advanced features

License:        MIT
URL:            https://github.com/attakdefand/TonleDB
Source0:        %{name}-%{version}.tar.gz

BuildRequires:  cargo rustc
BuildArch:      x86_64

Requires:       systemd

%description
TonleDB is a Rust database with SQL + NoSQL over a shared storage+txn kernel.
Enhanced with advanced features including:
- SQL: CREATE TABLE, INSERT, SELECT (WHERE =), CREATE INDEX (single-col equality)
- NoSQL: KV CRUD, Document insert/get
- Secondary indexes, TTL for documents, MVCC, full ACID transactions
- Arrow/Parquet support for analytics workloads
- Row-level security and point-in-time recovery
- In-memory + WAL persistence, LRU cache wrapper
- HTTP API (Axum) + CLI

%prep
%setup -q

%build
cargo build --release -p tonledb-core -p tonledb-storage -p tonledb-sql -p tonledb-nosql-kv -p tonledb-nosql-doc -p tonledb-metrics -p tonledb-network -p tonledb-backup -p tonledb-arrow -p tonledb-wire-pg

%install
mkdir -p %{buildroot}/%{_bindir}
mkdir -p %{buildroot}/%{_libdir}/tonledb
mkdir -p %{buildroot}/%{_sysconfdir}/tonledb
mkdir -p %{buildroot}/%{_sharedstatedir}/tonledb
mkdir -p %{buildroot}/%{_unitdir}

install -m 755 target/release/tonledb-network %{buildroot}/%{_bindir}/tonledb
cp -r crates %{buildroot}/%{_libdir}/tonledb/
install -m 644 tonledb.toml %{buildroot}/%{_libdir}/tonledb/tonledb.toml.example

install -m 644 packaging/systemd/tonledb.service %{buildroot}/%{_unitdir}/

%pre
getent group tonledb >/dev/null || groupadd -r tonledb
getent passwd tonledb >/dev/null || useradd -r -g tonledb -d %{_sharedstatedir}/tonledb -s /sbin/nologin -c "TonleDB" tonledb

%post
/bin/systemctl daemon-reload >/dev/null 2>&1 || :
/bin/systemctl enable tonledb.service >/dev/null 2>&1 || :

%preun
/bin/systemctl stop tonledb.service >/dev/null 2>&1 || :
/bin/systemctl disable tonledb.service >/dev/null 2>&1 || :

%postun
/bin/systemctl daemon-reload >/dev/null 2>&1 || :

%files
%{_bindir}/tonledb
%{_libdir}/tonledb/*
%{_unitdir}/tonledb.service
%dir %{_sysconfdir}/tonledb
%dir %{_sharedstatedir}/tonledb

%changelog
* Sat Oct 03 2025 TonleDB Team <tonledb@example.com> - 0.2.0-1
- Enhanced with secondary indexes, TTL for documents, MVCC
- Added full ACID transactions and constraint validation
- Implemented Arrow/Parquet support for analytics
- Added row-level security and point-in-time recovery
- Added PostgreSQL wire protocol compatibility

* Sat Oct 02 2025 TonleDB Team <tonledb@example.com> - 0.1.0-1
- Initial package
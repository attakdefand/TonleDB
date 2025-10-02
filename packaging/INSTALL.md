# Installing TonleDB from Packages

This guide explains how to install TonleDB from pre-built packages.

## Debian/Ubuntu Installation

### From Release Packages

1. Download the `.deb` package from the [releases page](https://github.com/your-username/tonledb/releases)
2. Install the package:
   ```bash
   sudo dpkg -i tonledb_0.1.0_amd64.deb
   sudo apt-get install -f  # Fix dependencies if needed
   ```

### From Package Repository (when available)

```bash
# Add the repository (example, adjust for actual repository)
echo "deb [trusted=yes] https://repo.tonledb.org/apt/ /" | sudo tee /etc/apt/sources.list.d/tonledb.list
sudo apt update
sudo apt install tonledb
```

## CentOS/RHEL/Fedora Installation

### From Release Packages

1. Download the `.rpm` package from the [releases page](https://github.com/your-username/tonledb/releases)
2. Install the package:
   ```bash
   # For CentOS/RHEL
   sudo yum install tonledb-0.1.0-1.x86_64.rpm
   
   # For Fedora
   sudo dnf install tonledb-0.1.0-1.x86_64.rpm
   ```

### From Package Repository (when available)

```bash
# For CentOS/RHEL with EPEL
sudo yum install tonledb

# For Fedora with Copr
sudo dnf copr enable your-username/tonledb
sudo dnf install tonledb
```

## Post-Installation

After installation, TonleDB will be available as a system service:

### Starting the Service
```bash
sudo systemctl start tonledb
```

### Enabling Auto-Start on Boot
```bash
sudo systemctl enable tonledb
```

### Checking Service Status
```bash
sudo systemctl status tonledb
```

### Viewing Logs
```bash
sudo journalctl -u tonledb -f
```

## Configuration

The default configuration file is located at `/etc/tonledb/tonledb.toml`. You can modify this file to change settings like:

- Server binding address and port
- TLS settings
- Rate limiting parameters
- Storage paths
- Authentication settings

Example configuration:
```toml
[server]
bind = "0.0.0.0:8080"
require_tls = false
idle_timeout_ms = 30000
request_body_limit_bytes = 2097152
rate_limit_rps = 100
rate_limit_burst = 200

[tls]
enabled = false
cert_path = "/etc/tonledb/cert.pem"
key_path = "/etc/tonledb/key.pem"
require_client_auth = false
ca_path = "/etc/tonledb/ca.pem"

[auth]
mode = "none"
token_file = "/etc/tonledb/tokens.json"

[storage]
encrypt_at_rest = false
kek_env = "TONLEDB_KEK"
wal_path = "/var/lib/tonledb/wal"
```

## Usage

Once the service is running, you can interact with TonleDB via HTTP:

```bash
# Health check
curl http://localhost:8080/health

# SQL query
curl -X POST http://localhost:8080/sql \
  -H "Content-Type: application/json" \
  -d '{"sql": "SELECT * FROM users"}'

# Key-Value operations
curl http://localhost:8080/kv/mykey
curl -X POST http://localhost:8080/kv/mykey -d "myvalue"
curl -X DELETE http://localhost:8080/kv/mykey

# Document operations
curl -X POST http://localhost:8080/doc/users \
  -H "Content-Type: application/json" \
  -d '{"name": "John", "age": 30}'
```

## Uninstalling

### Debian/Ubuntu
```bash
sudo apt remove tonledb
```

### CentOS/RHEL/Fedora
```bash
# For CentOS/RHEL
sudo yum remove tonledb

# For Fedora
sudo dnf remove tonledb
```

Note: Configuration files in `/etc/tonledb/` and data in `/var/lib/tonledb/` are typically preserved during uninstallation.
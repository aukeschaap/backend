# Backend

A lightweight Rust HTTP server that exposes system monitoring metrics (CPU and disk usage) via a REST API. Built with [Axum](https://github.com/tokio-rs/axum) and running on [Tokio](https://tokio.rs/).

## Requirements

- [Rust](https://www.rust-lang.org/tools/install) (edition 2024)

## Getting Started

```bash
# Clone the repository
git clone https://github.com/aukeschaap/backend.git
cd backend

# Build and run
cargo run
```

The server starts on `http://0.0.0.0:3000`.

## API Endpoints

| Method | Path | Description |
|--------|------|-------------|
| `GET` | `/server/cpu_usage` | Returns system uptime (seconds) and CPU usage per core (%) |
| `GET` | `/server/disk_usage` | Returns name, mount point, total space, and available space for each disk |

### Example responses

**`GET /server/cpu_usage`**

```json
{
  "uptime_seconds": 123456,
  "cpu_usage": [12.5, 8.3, 45.1, 5.0]
}
```

**`GET /server/disk_usage`**

```json
[
  {
    "name": "sda1",
    "mount_point": "/",
    "total_space": 500107862016,
    "available_space": 320000000000
  }
]
```

## Dependencies

| Crate | Purpose |
|-------|---------|
| [`axum`](https://crates.io/crates/axum) | HTTP framework |
| [`tokio`](https://crates.io/crates/tokio) | Async runtime |
| [`serde`](https://crates.io/crates/serde) | JSON serialization |
| [`sysinfo`](https://crates.io/crates/sysinfo) | CPU and disk metrics |
| [`tower-http`](https://crates.io/crates/tower-http) | CORS middleware |
| [`tracing`](https://crates.io/crates/tracing) / [`tracing-subscriber`](https://crates.io/crates/tracing-subscriber) | Structured logging |

## Architecture

See [Architecture.md](Architecture.md) for a detailed description of the planned k3s-based deployment architecture.

# solana-validator-optimizer

[![Crates.io](https://img.shields.io/crates/v/solana-validator-optimizer.svg)](https://crates.io/crates/solana-validator-optimizer)
[![Release](https://img.shields.io/github/v/release/0rlych1kk4/solana-validator-optimizer?display_name=tag)](https://github.com/0rlych1kk4/solana-validator-optimizer/releases)
[![Docs.rs](https://docs.rs/solana-validator-optimizer/badge.svg)](https://docs.rs/solana-validator-optimizer)

> A production-grade Rust toolkit to enhance Solana validator performance by reducing sync latency and improving RPC responsiveness.

---

## Overview

`solana-validator-optimizer` is a modular enhancement suite for Solana validator infrastructure. It helps operators by:

- Prefetching ledger snapshots from trusted mirrors  
- Validating snapshot integrity via SHA-256  
- Auto-tuning validator configuration based on hardware resources  
- Adding an in-memory LRU cache for frequently used RPC calls  
- Exposing Prometheus metrics via a `/metrics` endpoint  

---

## Ideal For

- Solana Validator Operators  
- RPC Infrastructure Maintainers  
- Performance-Focused Mainnet/Devnet Deployments  

---

## Core Features

| Feature                | Description                                                                |
|------------------------|----------------------------------------------------------------------------|
| Snapshot Prefetching   | Downloads or copies snapshots locally with optional SHA-256 validation     |
| RPC LRU Cache          | Reduces redundant RPC calls like `getBalance`, `getEpochInfo`, etc.        |
| Prometheus Metrics     | `/metrics` endpoint for cache hits, misses, request counts, and latency    |
| Config Auto-Tuner      | Adjusts validator configuration based on CPU, RAM, disk, and network       |

---

## Architecture

![Architecture Overview](docs/architecture.png)

---

## Getting Started

### 1. Clone the Repository

```bash
git clone https://github.com/0rlych1kk4/solana-validator-optimizer.git
cd solana-validator-optimizer
```

---

### 2. Configuration

Create a `Config.toml` file in the project root:

```toml
# Config.toml

# Snapshot settings
snapshot_url   = ""    # e.g. "https://snapshots.myvalidator.com/latest.tar.zst"
snapshot_sha256 = ""   # optional SHA-256 checksum for validation

# RPC and Metrics
rpc_url      = "https://api.mainnet-beta.solana.com"
metrics_port = 9090    # Prometheus scrapes at http://<host>:9090/metrics

# Cache settings
cache_size = 128       # Number of entries in the RPC LRU cache
```

---

## Usage – CLI Mode

Run all modules (snapshot, RPC cache, auto-tuner, metrics):

```bash
cargo run -p solana-validator-optimizer-cli --release
```

---

## Usage – As a Library

Add this to your `Cargo.toml`:

```toml
[dependencies]
solana-validator-optimizer = "1"
tokio = { version = "1", features = ["full"] }
anyhow = "1"
```

Example usage:

```rust
use solana_validator_optimizer::{Optimizer, Config};

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    let cfg = Config::from_file("Config.toml")?;
    let mut optimizer = Optimizer::new(cfg).await?;
    optimizer.start().await?;
    Ok(())
}
```

---

## Environment Variables

You can override any `Config.toml` values using environment variables.

| Config Key     | Environment Variable        |
|----------------|------------------------------|
| snapshot_url   | `OPTIMIZER_SNAPSHOT_URL`    |
| rpc_url        | `OPTIMIZER_RPC_URL`         |
| metrics_port   | `OPTIMIZER_METRICS_PORT`    |
| cache_size     | `OPTIMIZER_CACHE_SIZE`      |

**Example:**

```bash
OPTIMIZER_RPC_URL=https://api.mainnet-beta.solana.com OPTIMIZER_CACHE_SIZE=256 cargo run -p solana-validator-optimizer-cli --release
```

---

## Contributing

Contributions are welcome!  
Feel free to open issues, feature requests, or pull requests.

---

## License

Licensed under the **Apache-2.0 License**.

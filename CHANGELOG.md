# Changelog

All notable changes to this project will be documented in this file.  
This project follows [Semantic Versioning](https://semver.org/).

---

## [1.2.0] – 2025-11-15
### **Solana 3.x Upgrade**

This release upgrades **solana-validator-optimizer** to the **Solana 3.x** stack and modernizes internal modules for long-term compatibility.

###  Added / Updated
- Upgraded core Solana crates to **3.0.x**:
  - `solana-client`
  - `solana-sdk`
  - `solana-ledger`
- Added `solana-commitment-config` (replaces the removed `commitment_config` module in Solana v3)
- Updated all affected internal modules for the new Solana API structure:
  - `rpc_cache_layer.rs`
  - `blockstream_optimizer.rs`
  - `config.rs` / `config_autotuner.rs`
  - `snapshot_prefetcher.rs`
  - `metrics.rs`
  - `utils.rs`
- Verified end-to-end functionality against:
  - **Devnet** (`https://api.devnet.solana.com`)
  - **Local validator** (`solana-test-validator`)
- Confirmed correct behavior of:
  - RPC LRU cache
  - Snapshot prefetcher
  - Blockstream optimizer
  - Metrics server

### ️ Fixed
- Replaced all Solana v1.18 path imports removed or relocated in Solana v3.x  
  (e.g., `commitment_config` now comes from `solana-commitment-config`)
- Cleaned workspace structure:
  - `core` crate is now **library-only**
  - Binary lives in `bin/cli`

###  Impact on Users
- **No breaking API changes** in the public Rust library
- Downstream crates can now use SVO with **Solana 3.x** without any `[patch]` overrides
- Eliminates future incompatibility warnings related to outdated Solana crates

###  Upgrade

```toml
[dependencies]
solana-validator-optimizer = "1.2"
```

## [1.1.1] - 2025-11-09

### Added
- Introduced `lib.rs` with public module exports (`config`, `metrics`, `rpc_cache_layer`, etc.).
- Added `help_summary()` and `VERSION` constant to public API.
- Enabled full API documentation on [docs.rs](https://docs.rs/solana-validator-optimizer).

### Improved
- Project is now usable as both a CLI tool *and* a Rust library crate.

---

Older versions are described in release tags.


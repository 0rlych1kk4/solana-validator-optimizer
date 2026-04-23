# Changelog

All notable changes to this project will be documented in this file.  
This project follows [Semantic Versioning](https://semver.org/).

---

## [2.1.2] - 2026-04-23
### Security & Maintenance
- Updated Solana dependencies to v3.1.12
- Updated OpenSSL bindings to latest compatible versions
- Reduced exposure to upstream OpenSSL advisories

### Notes
- OpenSSL is a transitive dependency via Solana runtime
- No direct usage in this project’s execution path
- Continuing to monitor upstream patches

---

## [1.2.1] - 2026-01-25
### Changed
- Documentation and metadata cleanup
- Removed legacy NOTICE file for MIT-only licensing clarity

---

## [1.2.0] – 2025-11-15
### Solana 3.x Upgrade

### Added / Updated
- Upgraded core Solana crates to 3.0.x:
  - `solana-client`
  - `solana-sdk`
  - `solana-ledger`
- Added `solana-commitment-config`
- Updated internal modules for Solana v3 API

### Fixed
- Replaced deprecated Solana imports
- Cleaned workspace structure

### Impact
- No breaking API changes
- Full compatibility with Solana 3.x

---

## [1.1.1] - 2025-11-09
### Added
- Introduced `lib.rs` with public module exports
- Added `help_summary()` and `VERSION`
- Enabled docs.rs support

### Improved
- CLI + library dual usage

---

Older versions are described in release tags.

# solana-validator-optimizer

> A production-grade Rust tool to enhance the performance of Solana validator nodes by reducing sync latency and improving RPC responsiveness.

---

##  Overview

`solana-validator-optimizer` is a modular infrastructure enhancement tool for Solana validators, written in Rust. It helps validator operators:

- Prefetch snapshots from trusted mirrors
- Validate snapshot integrity using SHA256
- Auto-tune Solana validator configurations based on hardware
- Add an in-memory LRU cache layer for RPC endpoints
- Monitor metrics via Prometheus-ready endpoint

Ideal for:
- Validator operators
- RPC infrastructure maintainers
- Performance-tuned Solana deployment environments

---

##  Architecture

![Architecture Overview](docs/architecture.png)


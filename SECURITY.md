# Security Policy

## Dependency Security

This project depends on Solana runtime crates, which may include transitive vulnerabilities such as OpenSSL, rustls, webpki, ed25519-dalek, curve25519-dalek, and other upstream ecosystem dependencies.

We:

- Regularly update dependencies
- Analyze dependency graphs using `cargo tree`
- Validate dependency updates using `cargo test`
- Review security advisories from GitHub Dependabot and RustSec
- Evaluate vulnerabilities based on actual execution paths
- Avoid direct usage of unsafe or vulnerable APIs
- Avoid forced dependency overrides that may break Solana or Agave compatibility

## Vulnerability Handling

Not all reported vulnerabilities are directly exploitable in this project.

Each alert is reviewed based on:

- Direct vs transitive dependency
- Whether the vulnerable code path is used
- Whether the dependency is reachable from this project
- Exposure to untrusted input
- Availability of a compatible upstream fix
- Risk of breaking Solana or Agave dependency compatibility

Non-applicable or upstream-controlled vulnerabilities are documented and monitored for upstream fixes.

## Completed Security Remediations

### OpenSSL Dependabot Alert Remediation

**Repository:** `0rlych1kk4/solana-validator-optimizer`  
**Status:** Completed  
**Date:** May 2026  
**Affected dependency:** `openssl`  
**Manifest:** `Cargo.lock`

#### Summary

Resolved the GitHub Dependabot OpenSSL security alert by updating the resolved Rust dependency versions in `Cargo.lock`.

#### Remediation Completed

- Updated `openssl` from `0.10.78` to `0.10.79`
- Updated `openssl-sys` from `0.9.114` to `0.9.115`
- Validated the project with `cargo test`
- Pushed the fix to the `main` branch
- GitHub Dependabot rescanned the dependency files successfully
- Dependabot alerts now show `0 Open`
- Local branch is clean and synced with `origin/main`

#### Validation Evidence

```text
cargo test completed successfully
openssl v0.10.79 compiled successfully
GitHub Dependabot alerts: 0 Open
git status: nothing to commit, working tree clean
```

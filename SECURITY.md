# Security Policy

## Dependency Security

This project depends on Solana runtime crates, which may include transitive vulnerabilities (e.g., OpenSSL, rustls, webpki).

We:

- Regularly update dependencies
- Analyze dependency graphs using `cargo tree`
- Evaluate vulnerabilities based on actual execution paths
- Avoid direct usage of unsafe or vulnerable APIs

## Vulnerability Handling

Not all reported vulnerabilities are exploitable in this project.

Each alert is reviewed based on:

- Direct vs transitive dependency
- Whether the vulnerable code path is used
- Exposure to untrusted input

Non-applicable vulnerabilities are documented and monitored for upstream fixes.

## Reporting a Vulnerability

If you believe you have found a security issue:

- Open a GitHub issue
- Or contact the maintainer directly

## Scope

This project does NOT:

- Implement custom TLS/SSL handling
- Perform custom certificate validation
- Expose raw cryptographic primitives

# Security Policy

## Supported Versions

Security fixes are applied to the latest version available from the default
branch and, when applicable, the latest published release.

| Version        | Supported |
| -------------- | --------- |
| Latest version | Yes       |
| Older versions | No        |

## Reporting a Vulnerability

Please do not report security vulnerabilities through public GitHub issues.

Use GitHub's private vulnerability reporting feature for this repository, if
available. Include the following information:

- A clear description of the vulnerability
- Steps to reproduce the issue
- The affected version or commit
- The potential security impact
- Any relevant logs, proof of concept, or supporting evidence
- A suggested remediation, if known

Reports will be reviewed as soon as reasonably possible. Please allow time for
validation, coordination, and remediation before publicly disclosing the issue.

## Dependency Security

This project depends on Solana and Agave runtime crates and their transitive
dependencies. These may include security-sensitive libraries such as OpenSSL,
rustls, webpki, ed25519-dalek, curve25519-dalek, Quinn, and other upstream Rust
ecosystem components.

Dependency security is managed by:

- Reviewing GitHub Dependabot and RustSec advisories
- Inspecting dependency paths with `cargo tree`
- Evaluating whether vulnerable code paths are reachable
- Assessing exposure to untrusted input
- Updating compatible dependency versions
- Running workspace tests after dependency updates
- Running Clippy across all targets and features
- Avoiding forced dependency overrides that could break Solana or Agave compatibility
- Monitoring upstream-controlled vulnerabilities until compatible fixes are available

## Vulnerability Assessment

Not every reported vulnerability is directly exploitable in this project.

Each alert is evaluated based on:

- Whether the dependency is direct or transitive
- Whether the affected dependency is part of the active build graph
- Whether the vulnerable code path is used
- Whether the vulnerable functionality is reachable from this project
- Whether the dependency processes untrusted input
- The severity and practical impact of the issue
- The availability of a compatible upstream fix
- The risk of breaking Solana or Agave compatibility

Vulnerabilities that are not reachable, are controlled by upstream dependencies,
or cannot yet be remediated safely may be documented and monitored until a
compatible fix becomes available.

## Security Update Validation

Dependency security updates should be validated with the following commands, as
applicable:

```bash
cargo tree -i <dependency>
cargo tree --workspace --target all --all-features
cargo test --workspace
cargo clippy --workspace --all-targets --all-features
git diff --check
git status --short
```
## Security Remediation Records

Completed security remediations are tracked through:
- Git commit history
- Pull requests
- GitHub Dependabot alert history
- Release notes or CHANGELOG.md, when applicable

Individual remediation records are not maintained in this file so that the
security policy remains current and does not become tied to a specific alert,
dependency version, or point-in-time repository status.

## Disclosure and Coordination

Please avoid public disclosure until the issue has been validated and a fix or
mitigation is available.
When appropriate, maintainers may coordinate with upstream Solana, Agave, or
Rust crate maintainers where the vulnerability originates in a transitive
dependency or requires an upstream-compatible resolution.

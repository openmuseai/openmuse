# Security Policy

## Reporting a Vulnerability

DSH Office is an open-source project. If you discover a security
vulnerability, please report it privately through GitHub Security
Advisories:

- https://github.com/openmuseai/openmuse/security/advisories/new

Please do **not** disclose vulnerabilities publicly before we have had
a reasonable opportunity to respond.

## Scope

In scope:

- The DSH Office client (`frontend/client/`, Flutter + Rust: documents,
  database, local storage, FFI).
- DSH plugin packages (`middlewares/dsh/`): tools, approval / policy
  paths, host bridging.
- Build and packaging scripts that run on developer machines.

Out of scope (not in this repository):

- Upstream AppFlowy-IO repositories and third-party dependencies — report
  issues to their respective maintainers.
- Private workspace trees (`backend/`, `vendors/`, `local/`, `frontend/web/`).

## Local-first posture

DSH Office is local-only by design: no accounts, no login, data stays on
the device by default. Security-relevant changes that weaken this posture
(for example adding network identity, telemetry, or cloud sync) are
treated as high-priority review items.

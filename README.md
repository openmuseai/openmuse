# DSH Office

<h1 align="center">
  <img src="logo.png" alt="DSH Office" width="160" />
</h1>

> **AI-native office suite built on DSH.**

DSH Office is a **local-only** office client: desktop (Windows / macOS) and mobile (Android / iOS). Install and use it with **no account and no login**. Documents stay on the device by default.

This public repository contains two independently runnable trees:

| Tree | What it is |
|---|---|
| [`frontend/client/`](frontend/client/README.md) | Flutter + Rust desktop and mobile app |
| [`middlewares/`](middlewares/README.md) | DSH plugins, local sidecar, and container image |

- **License**: [AGPL-3.0](LICENSE) (derived from [AppFlowy-IO](https://github.com/AppFlowy-IO); attribution in [NOTICE](NOTICE))
- **Source**: [github.com/openmuseai/dsh-office](https://github.com/openmuseai/dsh-office)

<p align="center">
  <a href="https://github.com/openmuseai/dsh-office/releases"><b>Releases</b></a> •
  <a href="https://github.com/openmuseai/dsh-office/discussions"><b>Discussions</b></a> •
  <a href="https://github.com/openmuseai/dsh-office/issues"><b>Issues</b></a>
</p>

## Quick start

```bash
# TypeScript packages used by the local DSH sidecar
./middlewares/scripts/build-muse-packages.sh

# macOS debug app
./frontend/client/scripts/build-macos-appflowy.sh

# Android debug APK → frontend/client/dist/android/
./frontend/client/scripts/build-android-client.sh --debug

# iOS debug (device or simulator flags on the script)
./frontend/client/scripts/build-ios-client.sh --debug

# Local DSH sidecar (default :3080)
./middlewares/scripts/run-dsh-appflowy.sh
```

Plugin checks: `cd middlewares/dsh && pnpm install && pnpm check`.

Build outputs live under `frontend/client/dist/` (gitignored).

## What this repo is not

Cloud, Web clones, production deploy playbooks, and private plans are **not** part of the open-source tree. A full local workspace may still keep `backend/`, `vendors/`, `frontend/web/`, and a private `local/` directory beside this checkout; those paths are listed in `.gitignore`.

## Security and compliance

- Report vulnerabilities: [SECURITY.md](SECURITY.md)
- License: [LICENSE](LICENSE)
- Upstream and third-party notices: [NOTICE](NOTICE)

## License

Distributed under the GNU Affero General Public License v3.0. See [`LICENSE`](LICENSE) and [`NOTICE`](NOTICE).

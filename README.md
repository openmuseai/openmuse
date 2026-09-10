# OpenMuse

<h1 align="center">
  <img src="frontend/client/brand/logo.png" alt="OpenMuse" width="160" />
</h1>

> **AI-native office suite built on DSH.**

OpenMuse（中文名 **思构**）是一个 **local-only** office client: desktop (Windows / macOS) and mobile (Android / iOS). Install and use it with **no account and no login**. Documents stay on the device by default.

Product names are configured in [`frontend/client/brand/config.yaml`](frontend/client/brand/config.yaml).

This public repository contains two independently runnable trees:

| Tree | What it is |
|---|---|
| [`frontend/client/`](frontend/client/README.md) | Flutter + Rust desktop and mobile app |
| [`middlewares/`](middlewares/README.md) | DSH plugins, local sidecar, and container image |

- **License**: [AGPL-3.0](LICENSE) (derived from [AppFlowy-IO](https://github.com/AppFlowy-IO); attribution in [NOTICE](NOTICE))
- **Source**: [github.com/openmuseai/openmuse](https://github.com/openmuseai/openmuse)

<p align="center">
  <a href="https://github.com/openmuseai/openmuse/releases"><b>Releases</b></a> •
  <a href="https://github.com/openmuseai/openmuse/discussions"><b>Discussions</b></a> •
  <a href="https://github.com/openmuseai/openmuse/issues"><b>Issues</b></a>
</p>

## Quick start

```bash
# TypeScript packages used by the local DSH sidecar
./middlewares/scripts/build-muse-packages.sh

# macOS debug app / packed .app
./frontend/client/scripts/build-macos-appflowy.sh
./frontend/client/scripts/pack-macos-client.sh

# Windows portable zip + optional setup.exe
python frontend/client/scripts/pack-windows-client.py

# Android debug APK → frontend/client/dist/android/
./frontend/client/scripts/build-android-client.sh --debug

# iOS debug (device or simulator flags on the script)
./frontend/client/scripts/build-ios-client.sh --debug

# Local DSH sidecar (default :3080)
./middlewares/scripts/run-dsh-appflowy.sh
```

## Rebrand

User-visible branding for the Flutter client (pages, icons, display names, links) is in [docs/rebrand-plan.md](docs/rebrand-plan.md). Package names and bundle IDs are unchanged.

Web app plan: [docs/rebrand-plan-web.md](docs/rebrand-plan-web.md).

## Security and compliance

- Report vulnerabilities: [SECURITY.md](SECURITY.md)
- License: [LICENSE](LICENSE)
- Upstream and third-party notices: [NOTICE](NOTICE)

## License

Distributed under the GNU Affero General Public License v3.0. See [`LICENSE`](LICENSE) and [`NOTICE`](NOTICE).

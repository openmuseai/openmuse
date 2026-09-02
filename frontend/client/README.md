<h1 align="center" style="border-bottom: none">
    <img src="brand/logo.png" alt="DSH Office" width="120" /><br>
    <b>DSH Office</b>
</h1>

> **AI-native office suite built on DSH.**

Desktop (Windows · macOS) and mobile (Android · iOS). **Local mode only**: no account, no login, install and use. Documents, databases, and notes stay on the device.

- **Stack**: Flutter (UI) + Rust (core), local FFI
- **License**: AGPL-3.0 (derived from [AppFlowy-IO/AppFlowy](https://github.com/AppFlowy-IO/AppFlowy); see [NOTICE](../../NOTICE))
- **Repository**: [github.com/openmuseai/dsh-office](https://github.com/openmuseai/dsh-office)

<p align="center">
    <a href="https://github.com/openmuseai/dsh-office/releases"><b>Releases</b></a> •
    <a href="https://github.com/openmuseai/dsh-office/discussions"><b>Discussions</b></a> •
    <a href="https://github.com/openmuseai/dsh-office/issues"><b>Issues</b></a>
</p>

## Layout

```
frontend/client/
├── brand/logo.png          # product mark
├── scripts/                # build / run / pack / install
├── dist/                   # local artifacts (gitignored)
└── frontend/
    ├── appflowy_flutter/   # Flutter app
    └── rust-lib/           # Rust core (FFI)
```

Directory names such as `appflowy_flutter` are inherited internals. Do not use them in user-visible copy.

## Develop

Requires Flutter ≥ 3.27, Rust 1.85, cargo-make. See `frontend/appflowy_flutter/README.md`.

```bash
# from repository root
./frontend/client/scripts/build-macos-appflowy.sh
./frontend/client/scripts/run-macos-appflowy.sh --skip-packages
./frontend/client/scripts/build-android-client.sh --debug
./frontend/client/scripts/build-ios-client.sh --debug
```

Artifacts: `frontend/client/dist/`.

The DSH sidecar used by the app is built from `middlewares/` (`./middlewares/scripts/build-muse-packages.sh` and `./middlewares/scripts/run-dsh-appflowy.sh`).

## Install

- GitHub [Releases](https://github.com/openmuseai/dsh-office/releases)
- macOS: `DSH Office.app`
- Windows: inno installer when published
- Android / iOS: packages from Releases or local `dist/`

## Security

- [SECURITY.md](../../SECURITY.md)
- [NOTICE](../../NOTICE)

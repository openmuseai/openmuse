<h1 align="center" style="border-bottom: none">
    <img src="brand/logo.png" alt="OpenMuse" width="120" /><br>
    <b>OpenMuse</b>
</h1>

> **AI-native office suite built on DSH.**

Desktop (Windows · macOS) and mobile (Android · iOS). **Local mode only**: no account, no login, install and use. Documents, databases, and notes stay on the device.

- **Stack**: Flutter (UI) + Rust (core), local FFI
- **License**: AGPL-3.0 (derived from [AppFlowy-IO/AppFlowy](https://github.com/AppFlowy-IO/AppFlowy); see [NOTICE](../../NOTICE))
- **Repository**: [github.com/openmuseai/openmuse](https://github.com/openmuseai/openmuse)

<p align="center">
    <a href="https://github.com/openmuseai/openmuse/releases"><b>Releases</b></a> •
    <a href="https://github.com/openmuseai/openmuse/discussions"><b>Discussions</b></a> •
    <a href="https://github.com/openmuseai/openmuse/issues"><b>Issues</b></a>
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
./frontend/client/scripts/pack-macos-client.sh
python frontend/client/scripts/pack-windows-client.py
./frontend/client/scripts/pack-android-client.sh
./frontend/client/scripts/build-ios-client.sh --debug
```

Windows 开箱即用分发说明：[doc/packaging-windows.md](doc/packaging-windows.md)。
macOS 开箱即用分发说明：[doc/packaging-macos.md](doc/packaging-macos.md)。
Android 打包 / 真机验证 / 生产推送：[doc/android/README.md](doc/android/README.md)。

Artifacts: `frontend/client/dist/`.

The DSH sidecar used by the app is built from `middlewares/` (`./middlewares/scripts/build-muse-packages.sh` and `./middlewares/scripts/run-dsh-appflowy.sh`).

## Install

- GitHub [Releases](https://github.com/openmuseai/openmuse/releases)
- macOS: `OpenMuse.app`（`pack-macos-client.sh`；名称见 `brand/config.yaml`）
- Windows: zip / Inno 安装包（[`doc/packaging-windows.md`](doc/packaging-windows.md)）
- Android：[`doc/android/README.md`](doc/android/README.md)（`pack-android-client.py` → 真机验证 → `push-android-client.py`）
- iOS: packages from Releases or local `dist/`

## Security

- [SECURITY.md](../../SECURITY.md)
- [NOTICE](../../NOTICE)

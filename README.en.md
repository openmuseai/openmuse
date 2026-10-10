<div align="center">

# OpenMuse

**Open ecosystem. Open trust. Fully compatible with DSH.**

One DSH, running on desktop, in the browser, and on the phone. The fastest way in is the Web App.

[![Open the Web App](https://img.shields.io/badge/Open_the_Web_App-Try_it_now-2563EB?style=for-the-badge)](https://app.openmuseai.com)

### [https://app.openmuseai.com](https://app.openmuseai.com)

[简体中文](README.md) · **English**

[![License: AGPL-3.0](https://img.shields.io/badge/License-AGPL--3.0-555555)](LICENSE)
[![Web](https://img.shields.io/badge/Web-open_in_the_browser-2563EB)](https://app.openmuseai.com)
[![macOS](https://img.shields.io/badge/macOS-Apple_Silicon_%7C_Intel-555555)](#build-and-validate)
[![Android](https://img.shields.io/badge/Android-phone-555555)](#phone)

</div>

---

https://github.com/user-attachments/assets/4c98083d-6573-40ee-be89-161ba8c9e5d5

<div align="center">Desktop → browser → phone · 42 seconds</div>

## Web

The short path. Open it in a browser. After sign-in you get the same three-pane workbench as desktop: project on the left, editor in the middle, DSH on the right.

<div align="center">

[![Open the Web App now](https://img.shields.io/badge/Open_now-app.openmuseai.com-2563EB?style=for-the-badge)](https://app.openmuseai.com)

**[https://app.openmuseai.com](https://app.openmuseai.com)**

Chinese entry [app.openmuseai.com/zh](https://app.openmuseai.com/zh) · Site [openmuseai.com](https://openmuseai.com)

</div>

![OpenMuse Web App in the browser](docs/media/web-app.jpg)

The official DSH Web client is mounted in the right-hand pane, in the same tab as the Host. Directories load a page at a time as you expand them.

## Desktop

On macOS this is the full workbench. Project Workspace on the left holds the tree, search, plugins, and trash. Helix sits in the middle for code and documents, with preview and a terminal. DSH sits on the right and talks to the current workspace: edit files, switch models.

![Desktop: workspace, Helix, and DSH](docs/media/desktop-workbench.jpg)

## Phone

The same DSH conversation continues on the phone. The bottom bar is tasks, experts, library, scheduled tasks, and projects. You can message the workspace, switch models, and stop a generation in progress.

![Phone: DSH conversation, workspace edits, and model](docs/media/mobile-dsh.jpg)

Settings shows whether this desktop and the phone are online, and whether the realtime channel and the public channel are ready. Devices on the same account can see each other once they are online.

![Account and devices on the same account](docs/media/desktop-devices.jpg)

## What it does

| | |
| --- | --- |
| Three-pane workbench | Workspace, editor, and DSH stay in one window. Desktop, browser, and phone share that structure. |
| Local workspace | Project tree, search, plugins, trash. Resources move through authorized references. Permissions stay in the Host. |
| Helix editor | Code and Markdown, preview, terminal. The editor is its own plugin and follows the Host lifecycle. |
| DSH | Conversation, trace, models, edits inside the workspace, plugin commands. All three clients mount pinned DSH 0.1.7. Desktop runs the sidecar. The browser mounts the DSH Web client in the page. |
| Account and devices | One account ties the desktop and the phone together. Presence, the realtime channel, and the public channel are visible in settings. |
| Plugins | Helix, the file viewer, DSH, and Native View ship separately. Capability calls go through the Broker, with permission, cancel, and audit. |
| Release | The macOS package is closed inside this repository: Helix, the DSH lockfile, and checksums live here. Third-party components keep their own licenses. |

## Three lines

**Open ecosystem.** The Host owns the window, the workspace, plugin lifecycle, and permissions. The editor, the viewer, and DSH are composable plugins. Protocols, manifests, and the SDK are public, under AGPL-3.0.

**Open trust.** Dependencies have pinned versions, checksums, and a written origin. Capability calls go through the Broker. The macOS package is checked against the lockfile before it is packed, and third-party notices stay separate. Where the code comes from is written in [provenance](docs/CODE-REUSE-PROVENANCE.zh-CN.md).

**Fully compatible with DSH.** OpenMuse treats DSH as part of the workbench. All three clients run the same DSH client and the same account. Workspace binding, models, conversation, and plugin commands go through DSH, and upgrades follow the pinned [DSH 0.1.7 contract](docs/DSH-0.1.7-PROVIDER-CONTRACT-TCK.zh-CN.md).

## Build and validate

```bash
cargo test --workspace
./scripts/test_muse_packages.sh
cd app/openmuse_host
flutter analyze
flutter test
flutter build macos --debug
```

The macOS release package uses only what is in this checkout. Helix assets, the DSH registry lockfile, the local model-plugin tarball, and the official Node archives live here. On macOS, `./scripts/package_macos.sh` assembles the DSH npm closure in `target/`, verifies checksums, builds the Flutter app, and writes `dist/OpenMuse-macos.zip`. npm still downloads third-party registry dependencies pinned by `third_party/dsh/package-lock.json` when they are not already cached.

Web build notes are in [app/openmuse_web/README.md](app/openmuse_web/README.md). The live app is [https://app.openmuseai.com](https://app.openmuseai.com).

## License

OpenMuse source uses the repository's AGPL-3.0 [LICENSE](LICENSE). Third-party components keep their own licenses; see [third_party/README.md](third_party/README.md). A distributable release still needs complete third-party notices and each platform's own checks.

## Design docs

- Architecture: [docs/PLUGIN-HOST-DSH-ARCHITECTURE.zh-CN.md](docs/PLUGIN-HOST-DSH-ARCHITECTURE.zh-CN.md)
- Workbench parity: [docs/WORKBENCH-PARITY-SPEC.zh-CN.md](docs/WORKBENCH-PARITY-SPEC.zh-CN.md)
- Provenance: [docs/CODE-REUSE-PROVENANCE.zh-CN.md](docs/CODE-REUSE-PROVENANCE.zh-CN.md)
- Account and devices: [docs/ACCOUNT-DEVICE-PRESENCE-PAIRING.zh-CN.md](docs/ACCOUNT-DEVICE-PRESENCE-PAIRING.zh-CN.md), [docs/SAME-ACCOUNT-MULTI-DEVICE-PRODUCT-INTERACTION.zh-CN.md](docs/SAME-ACCOUNT-MULTI-DEVICE-PRODUCT-INTERACTION.zh-CN.md)
- Phone: [docs/MOBILE-PRODUCT-PRD.zh-CN.md](docs/MOBILE-PRODUCT-PRD.zh-CN.md)
- Web workbench acceptance: [docs/WEB-DESKTOP-WORKBENCH-UI-ACCEPTANCE.zh-CN.md](docs/WEB-DESKTOP-WORKBENCH-UI-ACCEPTANCE.zh-CN.md)
- DSH contract: [docs/DSH-0.1.7-PROVIDER-CONTRACT-TCK.zh-CN.md](docs/DSH-0.1.7-PROVIDER-CONTRACT-TCK.zh-CN.md)

The rest of the design docs are in [docs/](docs/).

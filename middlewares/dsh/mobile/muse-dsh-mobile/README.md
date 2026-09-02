# muse_dsh_mobile

Host-agnostic Flutter library for embedding the Remote DSH Web Client.

This package is vendored into the Muse monorepo at `middlewares/dsh/mobile/muse-dsh-mobile/` and tracked by the `openmuseai/muse` repository. AppFlowy (or any other host) depends on it via a relative `path` dependency (or `git` once published), and implements:

- [`DshMobileControlHost`](lib/src/dsh_mobile_control_host.dart) — optional Facet / parent-bridge
- [`DshNativeCapabilityHost`](lib/src/capabilities/dsh_native_capability_host.dart) — speech, share, capability snapshot
- [`DshFileChooserHost`](lib/src/capabilities/dsh_file_chooser_host.dart) — Photo Picker / Camera / SAF URI list

The library depends on Flutter, `webview_flutter`, `webview_flutter_android`, and `connectivity_plus` only. It does **not** import AppFlowy, Cloud auth, `speech_to_text`, `image_picker`, or Muse Facets.

Tokens, filesystem paths, `content://` URIs, and file bytes never go through the JS channel (`muse.native-capability/v1`). Files use the Android WebView file selector.

Vendored contracts: [`contracts/`](contracts/).

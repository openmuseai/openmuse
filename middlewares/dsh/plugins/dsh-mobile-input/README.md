# `@muse/dsh-mobile-input`

Host-side DSH plugin. Adds circular attachment and microphone controls inside `[data-composer-card]` when `data-muse-surface="mobile"` and the Flutter `MuseNativeCapability` channel exists.

- Files go through a hidden `<input type="file">` (WebView file selector). Bytes never ride the JS channel.
- Speech results append to the composer textarea. Nothing is auto-sent.

Flag: `MUSE_DSH_NATIVE_CAPABILITIES_ENABLED` (default on).

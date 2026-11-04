# OpenMuse runtime internationalization

`assets/catalog.json` is the reviewed message and locale registry. It declares canonical tags, self-names, aliases, direction, format locale and message parameters. Application code uses generated methods and an injected `OpenMuseLocaleController`; it does not select a language from a build flag or a global singleton.

```dart
final locale = OpenMuseLocaleController(
  defaultTag: config.defaultLocale,
  enabledLocales: config.enabledLocales,
  systemLocales: platformLocales,
  preferences: preferences,
);
await locale.restore();
// Inside a mounted scope:
Text(stringsOf(context).commonSave());
// Outside a widget: inject locale.strings into the service/plugin constructor.
```

Explicit selection persists per deployment. Following the system preserves that mode and reacts to platform changes. Unsupported script variants do not silently become another script. Each window has its own controller. Switching languages preserves the navigator, workspace, buffers and account session.

Run `python3 scripts/generate_i18n.py` from the repository root after changing the registry. It generates the typed facade, compiled offline catalog, auth compatibility adapter and deployable namespace packs. `--check` detects stale generated output without modifying files. Flutter's Dart executable must be on `PATH`, or set `DART_BIN`.

The bounded message format supports arguments, numbers, `select`, cardinal `plural`, exact plural arms, nesting, `#` and ICU apostrophe escaping. It rejects arbitrary formatters and executable expressions. Language resources must have a compatible `sourceRevision`, a known locale/namespace/message ID and unchanged argument contracts. Signed `localization.localeCatalog` metadata can introduce downloadable languages within installed Material rendering capabilities. English and Simplified Chinese remain bundled. New native rendering support, fonts or message IDs still need an installation capability update. New translations and existing registered messages can change remotely.

`OpenMuseClientRuntime` composes the signed configuration client, resource validation and locale controller. Restore it before constructing credential-bearing adapters; call `start()` after restoration. Use `OpenMuseRuntimeHost` around the application and `OpenMuseRuntimeNotice` in `MaterialApp.builder`. Presentation data changes live. Network endpoints remain bound to the current account session; changed endpoints stage for the next application boot. Every credential-bearing request calls `runtime.checkNetwork()` before sending credentials. A newer pending release does not extend an older session's expiration.

The host-owned HTML bridge consumes the same parsed message AST. It updates explicitly tagged text/placeholder/ARIA/date nodes. User filenames, document bodies and form values are never searched and replaced. Translations resembling HTML are displayed as text. Mounted forms and editor/reopen controls survive language changes.

Validation:

```sh
cd services/config-control && npm ci && npm test
cd ../../packages/openmuse_i18n
flutter pub get
flutter analyze
flutter test
```

The integration test launches the Node publisher with an ephemeral Ed25519 key and verifies its signed releases in Dart, including cohort persistence, language overlays, endpoint staging, expiry and atomic rejection. Mobile's `test/html_localization_test.dart` additionally compares both message parsers and runs the generated plugin HTML against a DOM with a mock host bridge.

Region boot, downloadable languages, plugin adapters and runtime parameters are described in `../../docs/REGIONAL-APPS-RUNTIME-REMEDIATION.zh-CN.md`. `OpenMuseClientRuntime.bootstrap()` restores, refreshes the selected regional feed, and prepares the preferred language before the account adapters are created.

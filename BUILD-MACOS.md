# OpenMuse Desktop 1.0.0 — macOS corresponding source

This source accompanies the Mac App Store document workspace. OpenMuse is
licensed under GNU AGPL version 3 or later; the license is in `Muse-Client/LICENSE`.
Original third-party licenses and copyright notices remain applicable. There is
no warranty except where separately provided. You may modify and redistribute
the covered software under the AGPL. Apple's signing private keys and production
credentials are not required or supplied. Sign your own modified build with your
own identity, or use an ad-hoc signature for local development.

Keep the three top-level directories as siblings. `SOURCE-SHA256.json` records
every archived input. Generated native binaries and build caches are omitted;
their source and build scripts are included. Public package dependencies are
pinned by the included Dart, npm/pnpm, Go and Cargo lockfiles. Resolve them with
the respective package managers. The source version is offered without charge.

## Tools and disk space

The release used Flutter 3.44.2 / Dart 3.12.2, Apple Xcode tools, Go 1.26.10,
Node.js 22.19.0 and an Apple Silicon Mac. Native Rust components use Cargo and
the installed Rust toolchain. PDFium additionally needs GN, Ninja, and its pinned
gclient dependencies. Native rebuilding needs substantially more space than the
final source tree. Reserve at least 30 GiB for a full native rebuild; the macOS app
packager itself refuses to start below 10 GiB free.

## Notes (SiYuan)

The complete pinned upstream source is under
`Muse-Plugin-Ecosystem/plugins/markdown/vendors/siyuan`, commit
`d3af9d5782082b379d45a69eedf4ebcfe8ba6889` (3.8.9). The OpenMuse adapter,
loopback policy and original resource-editor entry are in the adjacent `native`,
`web`, `scripts` and `flutter_runtime` directories. `upstream.json` pins the
source, Go version, `fts5` and `sqlcipher` build tags. The plugin's `scripts/build.py`
applies its recorded modifications in `.build/siyuan` and leaves upstream intact.

The upstream guard checks Git metadata. In a source tree extraction, restore that
metadata without replacing the supplied files:

```sh
cd Muse-Plugin-Ecosystem/plugins/markdown/vendors/siyuan
git init
git remote add origin https://github.com/siyuan-note/siyuan.git
git fetch --depth=1 origin d3af9d5782082b379d45a69eedf4ebcfe8ba6889
git reset --mixed FETCH_HEAD
git status --short
```

Verify the working tree is clean before proceeding. Then, from the markdown
plugin directory, run `python3 scripts/build.py web` and
`python3 scripts/build.py macos`. Its pinned OCR asset preparation obtains the
permissively licensed ONNX Runtime from its upstream location. SiYuan's own
third-party notices are included alongside its LICENSE and in the application.

## Slides

From `Muse-Plugin-Ecosystem/plugins/genoffice-pptx`, run `npm ci` then
`npm run pack:desktop`. This creates `dist/genoffice-pptx-0.8.2.mcpb`, including
the self-contained fixed server and editor. Its vendored original sources,
Apache license, NOTICE, package lock, font licenses and packaging scripts are
included. Node.js is an unmodified interpreter; its release source and licenses
are available at `https://nodejs.org/dist/v22.19.0/`. The app includes Node's
complete third-party license file.

## Additional linked document libraries

Although the Store entry exposes Notes and Slides, its Flutter dependency tree
also includes local Word, Spreadsheet and PDF library assets. Their source is
included under `ioffice/libs`, `ioffice/samples/libs`,
`ioffice/samples/docx-mvp-render/native`, `ioffice/vendors/gridcraft` and
`ioffice/koffice/pdf/rebuild`.

Build the Word FFI with Cargo from `ioffice/samples/docx-mvp-render/native`,
targeting `aarch64-apple-darwin` in release mode. Place the output
`libword_core_ffi.dylib` in the `word_render/native/macos/arm64` asset path.
The bridge's Cargo.toml identifies the included relative Rust crate dependencies.
The Spreadsheet bridge's Cargo.toml similarly points to the included GridCraft
source. Its own source and tests remain available even where the Store entry does
not register the corresponding optional product bundle.

For PDF, `pdf-rebuild/3rdparty/prebuilt/PRODUCT_BINARIES.md` and
`pdf-rebuild/scripts/build_product_pdfium.sh` identify base revision
`784a524ddaa26d86c0f499625b97902095c26dfe`, the supplied product patch, build
arguments and dependencies. The source and patch are included. Restore the pinned
Git metadata in the PDFium source tree as for SiYuan, preserving the supplied
patched working files. Use GN/Ninja to build `macos-arm64` and
`macos-viewer-arm64`, then `scripts/build_freetext.sh` to build the C++ bridge.
`pdf-flutter-app/tool/prepare_macos_native.sh` stages those assets. When building a
modified fork, update the local expected binary hashes to your own verified
outputs rather than presenting them as the original release binaries.

## Build the host

After preparing the native assets and Slides bundle:

```sh
cd Muse-Client
bash scripts/package_macos.sh --distribution app-store --profile release \
  --components ppt --skip-tests --build-name 1.0.0 --build-number 13
```

The Store compile flag selects the fixed document workspace, sandbox bookmarks,
account deletion, bundled software notices and Store update policy. The default
`direct` distribution retains the full workbench. `--skip-tests` in the packaging
example skips the separate Host test stage; run the tests with the documented
fixtures before distributing your own build. This command does not upload to
Apple or publish anything. Its output is `Muse-Client/dist/OpenMuse.app`.

The original release's exact Client commit and file hashes are recorded beside
the source download. A successful local rebuild is distinct from Apple signing,
notarization, App Review and a claim that the release pipeline has passed.

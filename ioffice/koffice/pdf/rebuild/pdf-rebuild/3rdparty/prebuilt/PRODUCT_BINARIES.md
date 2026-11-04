# Rebuild PDFium product binaries

`pdfium-mac-arm64-ksp.tgz` contains the macOS arm64 PDFium dylib used by the
rebuild editor. The older `pdfium-mac-arm64.tgz` is the upstream archive and
does not export the `KSP*` ExtensionAPI symbols required by this project.

| Target | Binary | SHA-256 |
|---|---|---|
| macOS arm64 | `lib/libpdfium.dylib` from `pdfium-mac-arm64-ksp.tgz` | `c0ff3e1e104c262ffec28bf27a846d3cbaf72fd7025c3883aec3cacf180ad8b9` |
| Android arm64 | `android-arm64/lib/libpdfium.so` | `00eb0c6af4bef9468b6883f0c0def00ba64c24dce4debe96870528ed633be6d7` |
| Android arm64 reader instance | `android-arm64/lib/libpdfium_viewer.so` | `c454a50a4b4bdb87c5fe0f7dde7fa5b0b8eb8b3d3a325165a4e62134ca50eb20` |
| macOS arm64 reader instance | `lib/libpdfium_viewer.dylib` from `pdfium-mac-arm64-viewer.tgz` | `8e48ad539ec119619ee86e683a91aba438d6c29874424f1c2d95111054a9c01e` |
| iOS simulator arm64 | `libpdfium.dylib` from `pdfium-ios-simulator-arm64-ksp.tgz` | `b9348c51fa81fc268fac9a45288af9092d0d2036d30f3f30140c65a374ce488f` |
| iOS simulator arm64 bridge | `libksp_product.dylib` from the same archive | `5f927c372798fb2023982d0eb7e9278f9619022a32efd2589bf96f3b0f2cae2c` |
| Web product module | `../../../pdf-flutter-app/web/ksp_product.mjs` | `c46a4ece28a25b0d1f6d9bff275740c09d881acc61ec946e6f903f485b7000c9` |
| Web product WASM | `../../../pdf-flutter-app/web/ksp_product.wasm` | `39f817fb694648fa18201a359070a1b177e5dce464eca46f1db019cc42b70ef2` |

The macOS and Android product binaries match the local patched source outputs
byte for byte. The source checkout is `3rdparty/src/pdfium` at base revision
`784a524ddaa26d86c0f499625b97902095c26dfe` (PDFium build 8021). Apply
the canonical `3rdparty/patches/pdfium-product.patch` to a fresh checkout;
it includes shaping/edit extensions, the iOS build fix and an optional reader
library output name. `scripts/build_product_pdfium.sh` checks that this exact
patch is applied. The older split patches document the individual changes but
should not be applied in addition to the canonical patch. The iOS simulator
output is arm64 and is not an iOS device binary.

To reconstruct the source checkout, fetch that PDFium commit and its DEPS with
`gclient`, apply `pdfium-product.patch`, then run:

```bash
bash scripts/build_product_pdfium.sh macos-arm64
bash scripts/build_product_pdfium.sh android-arm64
bash scripts/build_product_pdfium.sh android-viewer-arm64
bash scripts/build_product_pdfium.sh macos-viewer-arm64
bash scripts/build_product_ios_simulator.sh
```

The Web module uses the same PDFium base and canonical product patch, followed
by `3rdparty/patches/pdfium-web.patch`. Its secondary GN toolchain builds a
static PDFium archive, then Emscripten links the product C ABI and C++ editing
runtime into one module. `scripts/build_product_web.sh` checks the PDFium and
Emsdk revisions (`3b3963d86a5c97adaf6eaa845cdfe0b811c2f960`, Emscripten
4.0.15), runs the ABI and JavaScript bridge smoke tests, and copies the module
into the Flutter Web assets. Supply the ignored GN binary and Emsdk checkout
under `3rdparty/src`; on macOS set `EMSDK_PYTHON=/opt/homebrew/bin/python3`
if the system Python is older than Emsdk requires. The checked-in Web module
lets ordinary Flutter CI run without building PDFium on every job.

The source and GN/depot tools live under the ignored `3rdparty/src` and
`3rdparty/depot_tools` directories in this workspace; the checked-in patches,
revision and binary hashes are the reproducibility contract for a fresh
checkout. iOS device, Windows and Linux product binaries remain to be built
and verified. Build scripts must check required ExtensionAPI symbols before
packaging.

The reader variants use the same PDFium source and ExtensionAPI but a separate
dynamic-library identity, so pdfrx's isolate-local font callbacks cannot be
invoked by product editing or page operations on another isolate. This is a
runtime isolation measure until the product core owns rendering, font mapping
and editing in one session. It costs an extra library in native app bundles.

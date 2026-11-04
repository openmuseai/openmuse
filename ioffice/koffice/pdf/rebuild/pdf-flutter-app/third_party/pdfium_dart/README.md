# pdfium_dart adapter

Based on `pdfium_dart` 0.2.5 from [pdfrx](https://github.com/espresso3389/pdfrx), under the bundled MIT license. The Dart bindings and link hook are unchanged. The build hook uses this repository's fixed PDFium binaries for macOS and Android arm64, so `pdfrx` does not download or bundle another PDFium for those targets. Other native targets retain the upstream download behavior until their rebuild binaries are available.

# pdfrx viewer adapter

This is the `pdfrx` 2.4.8 Flutter package source, copied from the pinned
package version in `pubspec.lock`. Its upstream license is in `LICENSE`.

The product patch adds `PdfDocumentRef.requiresBackendInitialization` and
lets `PdfDocumentRefDirect` opt out. `PdfViewer` and `PdfDocumentViewBuilder`
then skip `pdfrxFlutterInitialize()` for documents whose page rendering and
text access are supplied by `ProductPdfDocument`.

The package's Web PDFium assets are omitted. Web always uses the product
reader and `ksp_product.wasm`; a failed product session is surfaced as an
open error. Native builds retain the diagnostic pdfrx reader fallback.

On a pdfrx upgrade, compare the three changed Dart files with the new
upstream release, reapply this narrow patch, and run `tool/verify.sh` for
shared, Web, macOS, and Android before changing the pinned version.

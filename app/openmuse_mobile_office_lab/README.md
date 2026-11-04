# Optional mobile Office laboratory

The Word/PDF/spreadsheet editor screens and native-engine integration fixtures are preserved here outside the store application's dependency closure. This package is not a store entry point. Its dependencies intentionally include the sibling `ioffice` checkout and unready engines.

`flutter pub get` and `flutter analyze` validate the preserved source package. `flutter test test/word_product_bundle_test.dart` checks the signed Word product resources. Device integration fixtures require a separate development host with the corresponding native engines linked; they are not release acceptance tests.

Production mobile code depends on `openmuse_mobile_core` ports and the kernel-free shared presentation package only. The signed product MCPBs remain in `openmuse_mobile/assets`; the store app neither registers these native editor routes nor advertises their capabilities. GenOffice uses its independently verified offline MCP Apps renderer.

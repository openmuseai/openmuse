# Third-party components

OpenMuse sources are AGPL-3.0; see the repository LICENSE and the bundled copy.
This Linux package also carries independently licensed components:

- Helix fork: MPL-2.0, source in `third_party/helix`, LICENSE bundled.
- Node.js 22.19.0: upstream combined LICENSE bundled; source and checksums at https://nodejs.org/dist/v22.19.0/.
- DeepSeek Harness 0.2.0-rc.2: MIT, LICENSE bundled, exact npm dependency integrity in `third_party/dsh/package-lock.json`.
- DSH plugins, pnpm and transitive npm dependencies: manifests and license files remain inside the bundled `node_modules`; consult each package's LICENSE and package.json.
- Flutter engine, Dart and Flutter packages: upstream engine and package notices are included in Flutter-generated assets; consult `data/flutter_assets/NOTICES.Z` and the committed pubspec.lock.
- Tree-sitter grammars: fixed source revisions and repositories are specified by the Helix languages configuration. Source repositories fetched during grammar builds retain their upstream licenses. A complete per-grammar license inventory must be reviewed before a public release.
- GTK, WebKitGTK, Secret Service and other ELF dependencies are provided by the operating system and listed in the Debian package metadata.

This is the initial engineering distribution notice. Public release requires a complete license inventory/SBOM and source-offer review for all redistributed components. Engineering build success does not complete that release review.

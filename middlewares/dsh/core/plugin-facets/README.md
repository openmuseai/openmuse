# @muse/plugin-facets

Domain-neutral public contracts for composing one logical Muse Plugin across Flutter UI, Rust Host Domain, and DSH
Cordis Agent runtimes.

The package owns only the stable v1 envelopes and composition metadata:

- Plugin and Facet descriptors;
- Context Contribution;
- Domain Change;
- Presentation Intent and result;
- contract-edge compatibility decisions.

It intentionally contains no Markdown, Word, Excel, AppFlowy EditorState, or DSH session DTO. Domain payload schemas
remain owned by each logical Plugin and are referenced by `type + schemaDigest`.

## Layout

- `schemas/v1/`: normative JSON Schema documents;
- `fixtures/v1/`: shared positive/negative fixtures and canonical schema digests;
- `src/`: TypeScript reference API and AJV validators;
- `rust/`: Rust types, validator, and digest projection;
- AppFlowy's Dart projection lives in
  `frontend/appflowy_flutter/packages/muse_plugin_facets` and reads the same fixtures in tests.

## Validation

```bash
pnpm check
```

The command checks the public boundary, TypeScript, shared fixtures, Rust formatting/tests, and build. Dart validation
is run separately from the AppFlowy package with `flutter analyze && flutter test`.

## Evolution

v1 may add optional fields only. Removing or reinterpreting a public field requires a new protocol major. Domain schema
changes do not change this package: the owning Plugin publishes a new digest/type and supplies any compatibility adapter.


# word_render

Render pipeline for `libs/word-core`: FRB `layoutDocument` → twip commands →
Flutter paint. Hosts should not measure or wrap text.

Public surface:

- `WordCoreEngine` — init FFI, inject fonts, layout a `.docx`
- `RenderEnvelope` / `WcRenderCommand` — Stage3 command model
- `DocumentPainter` / `PagePainter` — paint all pages or one page
- `hitTestPage` / `caretFromTap` — geometry for selection UI

Bundled fallback face: `packages/word_render/assets/MvpLatin.ttf`.

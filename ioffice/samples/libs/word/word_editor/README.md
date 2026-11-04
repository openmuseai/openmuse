# word_editor

Upper application layer for a Word-like host:

- Ribbon (File / Home / View)
- Paginated document surface with vertical scroll
- Caret / drag selection (geometry from `word_render`, not kernel mutate)
- Style intents: alignment, font size, underline **relayout** via `ViewParams`;
  bold/italic are draft until the kernel exposes run edits

Does not own FFI or paint commands.

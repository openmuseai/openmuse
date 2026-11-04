# ppt Flutter libs

Portable PowerPoint **render pipeline** and **slide shell**, split so a host app
does not ship one giant mixed package.

| Package | Role |
|---------|------|
| [`ppt_render`](ppt_render/) | C ABI FFI + `RenderCommand` JSON + `CustomPainter`. One-shot hello fixture or `.pptx` via `PptxReader`. |
| [`ppt_show`](ppt_show/) | Slide chrome: title bar, open/reload, fitted slide canvas, status. |

Native `ppt_core_ffi` stays with the sample (`samples/ppt-render/native`).
The sample host (`ppt-render/flutter`) only owns assets, file open, and wiring.

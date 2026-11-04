# Java `InstractionBuilder` dump (optional)

CI compares **normalized Stage3** (expanded glyphs). Frozen files `*.stage3.json` are the latin FT/hmtx contract.

To overlay a **real Java dump** (when LayoutRender + InstractionBuilder can run):

1. Record `drawText(char[], xpos[], y_baseline, bold, italic)` / `drawRect` / `drawLine`.
2. Map to the same JSON as `*.stage3.json` (`t: glyph|rect|line`, twip, page-absolute, y = baseline).
3. Place at `java/<id>.stage3.json` (e.g. `java/hello.stage3.json`).
4. CI prefers the Java file over the frozen Rust golden.

Do **not** check in int-block `instruction_cache` binaries.

Expand-xpos is already done in `Stage3Normalize` — Java may emit one `ins_draw_x_pos_text` per run; the dump script should expand to glyphs **or** dump already-expanded glyphs. Same schema either way.

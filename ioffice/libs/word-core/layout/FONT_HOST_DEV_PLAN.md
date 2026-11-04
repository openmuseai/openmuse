# FontHost 开发计划（WPS 字体引擎 · Java 管线）

> 日期：2026-08-23  
> 决策：**合进 wr-03-measure-shape**，新增 `04_function_font_host`；不新开 wr-09。  
> 样例外壳：桌面默认注入本机 Arial（或 `WORD_CORE_FONT`）；CI / Web / `flutter test` 仍用打包 `MvpLatin.ttf`。  
> 理论底稿：[FONT_ENGINE_GUIDE.md](./FONT_ENGINE_GUIDE.md) · 管线：[JAVA_RUST_PIPELINE.md](./JAVA_RUST_PIPELINE.md) S08  
> 样例阶段：[samples/docx-mvp-render/docs/phases/P6_FONT_HOST.md](../../../samples/docx-mvp-render/docs/phases/P6_FONT_HOST.md)

---

## 0. 为什么合进 wr-03，而不是新 feature

`JAVA_RUST_PIPELINE.md` 已把 **FontHost / TypefaceFactory（族名 → typeface）** 记在 wr-03，原先写成「单栈一套默认字体」。度量（hmtx）仍是 `01_font_stack` + `02_run_metrics`；本轮补的是 Java 在度量**之前**的那一层：

```text
docx rFonts 名字
    → FontHost / FontTool.addFontMapScheme（显示名 → 实际族名）
    → IFontTable / TypefaceFactory.createTypeface（族 → 文件）
    → AndroidTypeface + FreeTypeJNI（hmtx / hhea）
```

Rust 对应：

```text
宿主注入 FontBlob[]（或空 → 打包 MvpLatin）
    → FontHost（表 + SchemaItem）
    → FontStack（ttf-parser Face）
    → RunMetrics
```

新开 wr-09 会把「选字体」从 S08 拆出去，容易长成「Flutter `fontFamily: Arial` 碰运气」——那条路**脱离 Java 管线**，Web 也没有系统字体目录。合进 wr-03 才能保证：度量与绘制仍吃**同一份字节**，名字解析走 FontHost，而不是第二套引擎。

---

## 1. Java 管线（本轮必须对齐的切面）

| Java | 职责 | 本轮 Rust | 明确不做 |
|------|------|-----------|----------|
| `cn.wps.font.FontHost` | 缓存生命周期、`loadCache` / `initCache`、把表交给 `FontTool` | `layout/text/FontHost.rs`：由**已注入的字节**建表 | `makeFontCache2` 扫盘、DelayFontTable、水印字体拷贝 |
| `FontTool.addFontMapScheme` | `SchemaItem[]`：显示名 → 实际族名 | `FontHost::add_font_map_scheme` | 精华文案 / 日文 IPA 整表落地（无那些 TTF） |
| `FontHostSettings.SchemaItem` | `displayFontName` / `actualFontName` / 可选列表标记 | 同名字段 | UI 字体列表 |
| `Platform.getAllFontsDirectory` | **宿主**枚举系统/内置/SD 目录 | Flutter / 测试 = 宿主：读 Arial 或打包 TTF，再 `FontBlob` | 内核 `FontStack::default()` 在 `layout_document` 里探盘 |
| `TypefaceFactory.createTypeface` | 族文件 → `AndroidTypeface`；失败回落系统 serif/default | `FontHost::into_font_stack` → `FontStack`；未知名回落默认 Face | `android.graphics.Typeface.createFromFile` |
| `AndroidTypeface` + `FreeTypeJNI` | glyph advance | 已有 wr-03 `01`/`02`（ttf-parser ≡ 读 hmtx） | Paint sticky 回退（见 MEASURE_DRAW_INSTRUCTION） |

**切断点（与 Java 一致）：** 内核不自己发明「当前 OS 的默认 UI 字体」。Java 也是 Platform 给目录，FontHost 建表。Rust 的 Platform 角色在样例里是 Flutter：读文件 → 注入。

---

## 2. 分层：内核 vs 宿主

```text
                    ┌─ CI / Web / flutter test ── assets/MvpLatin.ttf
宿主（本机文件 I/O）─┤
                    └─ 桌面 App ── WORD_CORE_FONT 或本机 Arial.ttf
                              │
                              ▼
                     FontBlob { id, bytes }     ← 与 Java「已扫到的 font file」同构
                              │
                     word-core FontHost         ← 本 feature
                              │
                     FontStack + RunMetrics     ← wr-03 01/02（不变）
                              │
                     同一 bytes → Flutter loadFontFromList(WcDefault)
```

| 环境 | 谁选文件 | 内核空 `fonts[]` |
|------|----------|------------------|
| `cargo test` / Stage3 金文件 | 测试自己注入 fixture 或走 `DEFAULT_MVP_FONT` | **打包 MvpLatin**（确定性，无 Arial） |
| `flutter test` / CI | `FLUTTER_TEST` / `WORD_CORE_FORCE_BUNDLED_FONT=1` | 壳注入打包 TTF |
| Web | 无可靠系统字体目录 | 打包 TTF |
| 桌面 Debug/Release App | 默认 Arial 候选路径，或 `WORD_CORE_FONT` | 不应走到空列表：壳必须注入 |

内核 **禁止** 在 `layout_document` 调用 `FontStack::default()` 探测 `/System/Library/Fonts/...`。探盘是宿主职责（对标 `Platform.getAllFontsDirectory`）。`FontStack::default()` 仍保留给 wr-03-01 单测，不进产品热路径。

---

## 3. Function 与里程碑

| 序 | 项 | 出口 |
|----|----|------|
| **F0** | 本计划 + wr-03 `04_function_font_host` 文档 | 本文 + FUNCTIONS.md / FUNCTION.md |
| **F1** | `FontHost`：注入表、`add_font_map_scheme`、未知名回落、`into_font_stack` | `cargo test --test 04_function_font_host` |
| **F2** | `layout_document` 经 FontHost 建栈；空列表 = 打包 MvpLatin | 现有 P1/P4/P5 回归绿 |
| **F3** | 样例：桌面注入 Arial；测试/Web 打包 TTF | P6；`flutter test` 仍 12pt 行高 240（MvpLatin） |
| **F4**（后续） | docx `w:rFonts` 名字走 `resolve`；多 blob 按 id 命中 | 需 parse 已落 rFonts；本轮 layout MVP 仍单默认栈 |
| **F5**（后续） | TTC `collection_index`、粗斜 Face、精华/日文 scheme 资源 | 有文件再注册，不 stub |

---

## 4. 验收

1. **Java 形状**：名字解析在 FontHost，度量仍在 FontStack/hmtx；无 cosmic-text、无 Flutter `TextPainter` 测宽。  
2. **确定性 CI**：word-core 测试与 Stage3 金文件不依赖本机 Arial。  
3. **同字节**：壳注入的 bytes == `loadFontFromList` 的 bytes。  
4. **桌面观感**：有 Arial（或指定路径）时字形是真 Arial，不再是 MvpLatin 块状轮廓。  
5. **失败**：坏 TTF → `LayoutErrorKind::Font`；缺 glyph 仍 `Measure`（不静默 `width=size`）。

---

## 5. 禁令

- 不在 `libs/core-libs/stubs` 为 `cn.wps.font.*` 建工程 stub。  
- 不把 `FreeTypeJNI` 壳当度量后端。  
- 不在 Flutter 用系统 `fontFamily: "Arial"` 画、同时用另一份 TTF 度量。  
- 不把精华映射表硬编码进内核却不带对应 TTF（scheme API 可测，资源后续）。

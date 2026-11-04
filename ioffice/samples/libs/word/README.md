# word Flutter libs

Portable Word **render pipeline** and **editor shell**, split so a host app
does not ship one giant mixed package.

| Package | Role |
|---------|------|
| [`word_render`](word_render/) | FFI + layout + paint. One-shot `layoutDocument` or heap `WordSession` (caret / selection / insert / relayout). |
| [`word_editor`](word_editor/) | Word-like chrome: ribbon, paginated scroll surface, selection, style intents. |

Native `word_core_ffi` stays with the sample (`samples/docx-mvp-render/native`);
prebuilt artifacts are **bundled** into [`word_render/native/`](word_render/native/)
and extracted at runtime, so consumer Flutter apps need no Rust toolchain.
The sample host (`docx-mvp-render/flutter`) only owns assets, file open, and wiring.

## 供其他工程使用（开箱引入）

→ 完整说明见 **[DISTRIBUTION.md](DISTRIBUTION.md)**。

- 消费者用 **git 依赖**即可（本仓库即独立 git 仓库，本地/内网起步）：
  `word_render` / `word_editor` 各一个 `git: {url: <repo>, path: <subdir>}` 条目。
- 也可发布到 **私有 pub server** 或 **pub.dev**（pubspec 的 `publish_to` 已注释，删除即发布）。
- native 库（macOS ✅ 已入库）由 `resolveFfiLibrary` 自动加载；新增平台见
  `tool/build_native.sh` 与平台支持矩阵。
- 这两个是 **Flutter/Dart 包**，用 pub 依赖体系；maven 仅对 JVM/Android 工程有意义（JNI/AAR 路线，另行规划）。
// M-PPT-KMOSLIDE W8a：本文件原先是 **0 字节** —— `cn.wps.drawing.dmlr.handler.media` 整包缺失
//   （形态 C：空壳目录）。本波按 Java 权威逐类真实移植 6 个 handler 并在此登记。
// owner: ppt.drawing.dmlr.handler.media    wave: M-PPT-KMOSLIDE-W8a
//
// Java → Rust 文件映射（Java 权威：`koffice/presentation/core/cn/wps/drawing/dmlr/handler/media/`）：
//   | Java 文件                            | Java 行 | Rust 文件                          | Rust 行 |
//   |------------------------------------|--------|----------------------------------|--------|
//   | `AudioCDHandler.java`              | 47     | `AudioCDHandler.rs`              | 195     |
//   | `AudioFileHandler.java`            | 28     | `AudioFileHandler.rs`            | 117     |
//   | `EmbeddedWAVAudioFileHandler.java` | 28     | `EmbeddedWAVAudioFileHandler.rs` | 126     |
//   | `MediaExtLstHandler.java`          | 141    | `MediaExtLstHandler.rs`          | 686     |
//   | `QuickTimeFileHandler.java`        | 24     | `QuickTimeFileHandler.rs`        | 94      |
//   | `VideoFileHandler.java`            | 28     | `VideoFileHandler.rs`            | 116     |
//   6/6 全部登记，无 `todo!`、无 `#[cfg(any())]`、无注释掉的模块声明。
//
// `MediaExtLstHandler.java` 的内层类一并移植为**模块级结构体**（Java 内层类 → Rust 结构体）：
//   `MediaExtLstHandler` → `MediaExtHandler` → `MediaHandler` → `TrimHandler` / `FadeHandler` /
//   `BookmarkLstHandler` → `BookmarkHandler`（每个都带 `// Java: (inner class) …` 注）。
//
// 公共前提（6 个 handler 共有，非本包缺陷）：
//   · `cn.wps.drawing.data.media.Media` **在本仓库任何构建下都不参与编译**：
//     `drawing/data/mod.rs` 是 `#[cfg(feature = "full")] pub mod media;`，
//     而 `drawing/data/media/mod.rs` 全文是 `#[cfg(any())] pub mod Media;`
//     ⇒ 其嵌套类 `Media.AudioCD` / `Media.AudioFile` / `Media.VideoFile` /
//       `Media.QuickTimeFile` / `Media.EmbeddedWAVAudioFile` / `Media.MediaExtAttr` /
//       `Media.Bookmark` / `Media.BookmarkLst` 均无法在类型里直接命名。
//     本包的处理办法（不使用替身结构体、不留 `todo!`）：
//       ① 能借 `HandlerCallback` 关联类型承载的（`Media` / `AudioFile` / `VideoFile` /
//          `QuickTimeFile` / `EmbeddedWAVAudioFile`）直接作为模型槽 —— 关联类型正是 Java 侧
//          `HandlerCallbackImpl` 绑定 `Media.X` 的位置；
//       ② 没有关联类型槽的（`Media.AudioCD` / `Media.AudioCD.AudioCDTime` /
//          `Media.MediaExtAttr` / `Media.Bookmark` / `Media.BookmarkLst`）用文件内**本地模型
//          契约 trait** 承载其成员面（方法名与 Java 一致）：`AudioCDModel` / `AudioCDTimeModel`
//          （`AudioCDHandler.rs`）、`AudioFileModel`（`AudioFileHandler.rs`）、
//          `VideoFileModel`（`VideoFileHandler.rs`）、`EmbeddedWAVAudioFileModel`
//          （`EmbeddedWAVAudioFileHandler.rs`）、`MediaModel` / `MediaExtAttrModel` /
//          `BookmarkModel` / `BookmarkLstModel`（`MediaExtLstHandler.rs`）。
//     接线时（下一波）：为①在 `HandlerCallbackImpl` 里给关联类型、为②补契约 `impl`，
//     本包的分支、属性号、调用顺序无需改动。
//     owner: ppt.drawing.dmlr.handler.media    wave: M-PPT-KMOSLIDE-W8a
//   · 6 个 handler 在 Java 侧都 `extends DefaultElementHandler`。Rust 侧
//     `kfc_kfc_alpha::…::reader::DefaultElementHandler` 已是**真实结构体**（本波同步落地，
//     4 个方法体皆空），故本包统一以字段 `parent: DefaultElementHandler` 组合表达 `extends`，
//     未覆写的方法体委托给它（各处「继承空实现」注）。
//   · 暴露的 `ElementHandler` 实现是 `kfc_kfc_alpha` 的**真实 trait**
//     （`startElement(&self, i32, &dyn XmlRoAttr)`、
//      `enterSubElement(&self, i32) -> Option<Box<dyn ElementHandler>>`），
//     与 `SwNvPrHandler` / `SwPictureHandler` 的宿主契约一致；`&self` 形参下对模型/回调的写入
//     经 `Rc<RefCell<_>>` 完成（无 `Arc<Mutex<_>>`、无 `*const`/`*mut`）。
pub mod AudioCDHandler;
pub mod AudioFileHandler;
pub mod EmbeddedWAVAudioFileHandler;
pub mod MediaExtLstHandler;
pub mod QuickTimeFileHandler;
pub mod VideoFileHandler;

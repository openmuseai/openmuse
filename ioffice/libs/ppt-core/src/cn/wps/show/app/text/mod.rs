// M-PPT-KMOSLIDE SP3：`show/app/text/` 原先只有 3 个 form-E 自动桩（`use crate::*;` + 全 `todo!()`），
// 且整目录未登记（无 `mod.rs`），是不可达死码。本波按 Java 权威包
// `koffice/presentation/core/cn/wps/show/app/text/`（`KmoSlide.java` 从 `cn.wps.show.app.text` 导入这四个类）
// 落地 6 个真类并登记：
//   · `KmoTextStyleManager.java`（303 行，`KmoSlide` 成员 D3） → `KmoTextStyleManager.rs`
//   · `KmoTextOperator.java`（1236 行）                         → `KmoTextOperator.rs`
//   · `KmoCellTextOperator.java`（277 行）                      → `KmoCellTextOperator.rs`
//   · `KmoContext.java`（70 行）                                → `KmoContext.rs`
//   · `KmoPargraphRange.java`（40 行，兄弟类）                  → `KmoPargraphRange.rs`
//   · `KmoTextHyphen.java`（50 行，兄弟类）                     → `KmoTextHyphen.rs`
// 注意：**不要**与 `koffice/et/core/cn/wps/chart/text/`（`cn.wps.chart.text`）同名类混淆。
//
// ⚠ 上游依赖面（默认构建实测；`#[cfg(feature = "full")]` 不算「已装配」，`full` 因
//   `libs/core-libs/thirdparty/picparser` 10 个编译错误不可用）——SP3b 前置波：
//   · `cn.wps.drawing.app.text.KmoTextBody`：默认构建下 `#[cfg(any())]`，Rust 侧 5748 行里
//     1709 处 `TODO_RUST_CODE`（不可编译）⇒ `KmoTextOperator` 的**文本体面**整体 DEFERRED。
//   · `cn.wps.drawing.data.text.{TextListStyle,TextCharacterProperties,TextParagraphProperties,
//     TextBodyProperties,TextParagraph}`：真身在 `#[cfg(feature = "full")]` 后；默认构建只剩
//     `textbody_hosts` 空宿主与 `util/layout_shape` 的 ZST 影子 ⇒ **样式面**整体 DEFERRED。
//   · `cn.wps.show.app.slide.{KmoSlide,KmoSlideLayout,KmoSlideMaster}`：仅 `util/layout_shape.rs`
//     影子（且该文件归他波，禁改）⇒ **幻灯片/版式/母版导航面**整体 DEFERRED。
//   · `KmoTable`/`KmoTableCell`/`TableCell`/`KmoRange`、`FontCollection`/`FontRef`/`FontScheme`/
//     `ThemeableFontStyles`、`KmoShapeBeautifyTags`/`TagUtil`/`ClearPropInfo`/`DateUtil`：默认构建下不可达。
//   逐项注记见各文件内 `DEFERRED_NON_FUNCTION`（均带 owner/wave，供 SP3b 直接驱动）。
// owner: show.app.text    wave: M-PPT-KMOSLIDE-SP3

pub mod KmoContext;
pub mod KmoPargraphRange;
pub mod KmoTextHyphen;
pub mod KmoTextOperator;
pub mod KmoCellTextOperator;
pub mod KmoTextStyleManager;

pub mod KTableRange;
// CARGO_BLOCKER: MergeSpilit/KColIterator cluster not generated
// pub mod KColRange;
/* DEFERRED_NON_FUNCTION —— 依赖 `core::PropertyTool`（48 错误）与 `core::shape`
 *   闭包（`ShapeTool`/`ShapeMetrics`/`KShapeRange`，登记实测 29+ 错误）。二者属
 *   图形/属性子系统，与 docx 文本解析→排版→渲染必径无关 ⇒ 延后。 */
// pub mod Tools;
/* DEFERRED_NON_FUNCTION —— 依赖 `core::PropertyTool`（48 错误）与 `core::shape`
 *   闭包（`ShapeTool`/`ShapeMetrics`/`KShapeRange`，登记实测 29+ 错误）。二者属
 *   图形/属性子系统，与 docx 文本解析→排版→渲染必径无关 ⇒ 延后。 */
/* DEFERRED_NON_FUNCTION —— `KTableRangeBase`（1215 行）经 `RegularChecker`/`Tools`
 *   依赖 `data::table`（自动生成桩子树）+ `core::PropertyTool` + `core::shape` 闭包；
 *   其唯一调用点 `RangeTool.normalize3`（表格选区归一化）已随之延后。
 *   与 docx 文本解析→排版→渲染必径无关。 */
// pub mod KTableRangeBase;
pub mod CellsItem;
/* DEFERRED_NON_FUNCTION —— 依赖 `core::PropertyTool`（48 错误）与 `core::shape`
 *   闭包（`ShapeTool`/`ShapeMetrics`/`KShapeRange`，登记实测 29+ 错误）。二者属
 *   图形/属性子系统，与 docx 文本解析→排版→渲染必径无关 ⇒ 延后。 */
// pub mod RegularChecker;
pub mod KCellIterator;
/* DEFERRED_NON_FUNCTION —— 依赖 `data::table::{CellInfo,RowInfo,TableInfo}`（该子树延后）。 */
// pub mod KCellIteratorForCol;
/* DEFERRED_NON_FUNCTION —— `KComplexTableRange` 依赖 `KTableRangeBase`
 *   （经 `RegularChecker`/`Tools` → `core::PropertyTool` + `core::shape` 延后）。 */
// pub mod KComplexTableRange;
/* DEFERRED_NON_FUNCTION —— 依赖 `DefTableFixer`（延后）。 */
// pub mod RowRevisionFixer;
/* DEFERRED_NON_FUNCTION —— `DefTableFixer`/`RowRevisionFixer` 依赖
 *   `data::table::TableTool`（结构体持有 `PropertyTool` 字段 ⇒ 经 `core::PropertyTool`
 *   拉入 `core::shape` 闭包）。且 `DefTableFixer` 还经 glob 导出
 *   `DefTableTransport`（Java 嵌套接口）与 `Tools`，Rust 侧未生成对应项。
 *   属表格合并修复，与 docx 文本解析→排版→渲染必径无关 ⇒ 整体延后。 */
// pub mod DefTableFixer;
pub mod BrcCvFactory;

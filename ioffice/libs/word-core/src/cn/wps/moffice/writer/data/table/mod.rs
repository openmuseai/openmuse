pub mod CellInfo;
pub mod Data;
pub mod InfoFactory;
pub mod RowInfo;
pub mod TableInfo;
pub mod TableInfos;
pub mod TableInfosImpl;
pub mod TableJCData;
/* DEFERRED_NON_FUNCTION —— 同上（依赖 `core::PropertyTool`/`core::table::Tools`）。 */
/* DEFERRED_NON_FUNCTION —— `data/table/TableTool` 结构体持有
 *   `mPropertyTool: Option<PropertyTool>` 字段 ⇒ 整体登记经 `core::PropertyTool`
 *   拉入 `core::shape` 闭包（实测 24+ 错误）。
 *   但其中 `DefTableFixer` 需要的两个 `DefTable` helper 是自包含的 ⇒ 已单独承载于
 *   `TableToolMerge.rs`，使 `core::table::DefTableFixer` 真实接线。 */
// pub mod TableTool;
// pub mod TableToolMerge;
pub mod TablePos;
pub mod grid;
/* DEFERRED_NON_FUNCTION —— `tree` 子目录依赖 `data::table::TableTool`（延后）。 */
// pub mod tree;
pub mod KTableManager;
pub mod ITableManager;
pub mod ObjPools;
pub mod TableManagerUtil;

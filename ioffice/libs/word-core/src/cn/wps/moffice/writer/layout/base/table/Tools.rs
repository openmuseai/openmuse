// Rust stub for Tools (auto-generated)
// Original: cn/wps/moffice/writer/layout/base/table/Tools.java
use crate::*;

pub struct Tools {
    // TODO: fields
}

impl Tools {
    pub fn new() -> Self { todo!() }
    pub fn createCellEnvsForState(layoutState: LayoutState, typoState: StateSnapshot, pools: Pools) { todo!() }
    pub fn canReuseCells(cellEnvList: KList, typoState: StateSnapshot) -> bool { todo!() }
    pub fn canReuseCellEnv(tableCellLayoutEnv: TableCellLayoutState, cellState: StateSnapshot) -> bool { todo!() }
    pub fn hasBreakCells(pRow: i32, snapshot: TypoSnapshot) -> bool { todo!() }
    pub fn createCellEnvsForTableRow(layoutState: LayoutState, pRow: i32, setPage: bool, forceSetCellEnvs: bool, docEnv: DocumentEnv, mainState: MainState) { todo!() }
    pub fn createEnvForMergeCells(env: LayoutState, row: RowInfo, mergeCellEnvList: KList, pools: Pools) { todo!() }
    pub fn createCellEnv(env: LayoutState, cell: TypoTableCell, setPage: bool, forceSetCellEnvs: bool, docEnv: DocumentEnv, mainState: MainState) -> TableCellLayoutState { todo!() }
    pub fn getTextFlow(textFlow: i32) -> i32 { todo!() }
    pub fn hasVertMergedCells(pRow: i32, snapshot: TypoSnapshot) -> bool { todo!() }
    pub fn createCellEnvForCell(layoutState: LayoutState, cellInfo: CellInfo, pools: Pools) -> TableCellLayoutState { todo!() }
    pub fn createCellEnvForCell_1(layoutState: LayoutState, cellInfo: CellInfo, cantSplit: bool, pool: Pools) -> TableCellLayoutState { todo!() }
    pub fn findValidNextVMergeCell(cellInfo: CellInfo) -> bool { todo!() }
    pub fn findValidFirstVMergeCell(cellInfo: CellInfo) -> bool { todo!() }
    pub fn getFirstVertMergedCell(cellInfo: CellInfo) -> CellInfo { todo!() }
    pub fn setState(newCellEnv: TableCellLayoutState, cellState: StateSnapshot, pools: Pools) { todo!() }
    pub fn createTableCellLayoutEnv(env: LayoutState, pools: Pools) -> TableCellLayoutState { todo!() }
    pub fn isFirstLineOfPage(curPage: TypoPage, snapshot: TypoSnapshot) -> bool { todo!() }
    pub fn getFirstMergecell(cellInfo: CellInfo) -> CellInfo { todo!() }
}

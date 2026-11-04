// Rust stub for TableLayouter (auto-generated)
// Original: cn/wps/moffice/writer/layout/base/table/TableLayouter.java
use crate::*;

pub struct TableLayouter {
    // TODO: fields
}

impl TableLayouter {
    pub fn new() -> Self { todo!() }
    pub fn dispose(&self) { todo!() }
    pub fn reset(&self) { todo!() }
    pub fn initTableRowLayouter(&self) { todo!() }
    pub fn hasTablePos(&self, stateEnvoy: IStateEnvoy) -> bool { todo!() }
    pub fn flow(&self, mHasBreakRow: bool, layoutState: LayoutState) { todo!() }
    pub fn initForCalcTableFit(&self, stateEnvoy: IStateEnvoy) { todo!() }
    pub fn checkRowNeedStartFit(&self, stateEnvoy: IStateEnvoy) -> bool { todo!() }
    pub fn afterForCalTableFit(&self) { todo!() }
    pub fn getTableInfo(&self, stateEnvoy: IStateEnvoy) { todo!() }
    pub fn getAllowOverCoreHeightForNormal(&self, stateEnvoy: IStateEnvoy) { todo!() }
    pub fn getAllowOverCoreHeightForWrap(&self, stateEnvoy: IStateEnvoy) { todo!() }
    pub fn shouldCutWrapTableIfExceeded(&self, pos: Box<dyn IPos>, stateEnvoy: IStateEnvoy) -> bool { todo!() }
    pub fn flowNormalTableRow(&self, stateEnvoy: IStateEnvoy) { todo!() }
    pub fn hasLinesOfDrawing(&self, drawing: TypoDrawing) -> bool { todo!() }
    pub fn flowWrapTable(&self, stateEnvoy: IStateEnvoy) { todo!() }
    pub fn getTableRowForAnchor(&self, wrapTableDrawing: TypoDrawing, stateEnvoy: IStateEnvoy) -> TypoTableRow { todo!() }
    pub fn setIsFirstRowOfTable(&self, stateEnvoy: IStateEnvoy) { todo!() }
    pub fn getWrapTableDrawing(&self, stateEnvoy: IStateEnvoy) -> TypoDrawing { todo!() }
    pub fn getEmbedMinFirstLine(&self, pFirsLine: i32, snapshot: TypoSnapshot) -> i32 { todo!() }
    pub fn checkOverlapToFootNote(&self, drawing: TypoDrawing) -> bool { todo!() }
    pub fn getFloatCotainerPage(&self, stateEnvoy: IStateEnvoy) -> TypoPage { todo!() }
    pub fn isOverFlow(&self, drawing: TypoDrawing, stateEnvoy: IStateEnvoy) -> bool { todo!() }
    pub fn calJCWidth(&self, jcData: TableJCData) -> i32 { todo!() }
    pub fn processCardInfo(&self, layoutState: LayoutState, stateEnvoy: IStateEnvoy) { todo!() }
    pub fn getWrapTableHeight(&self, stateEnvoy: IStateEnvoy) -> i32 { todo!() }
    pub fn postFlowWrapTableRow(&self, stateEnvoy: IStateEnvoy) { todo!() }
    pub fn postFlowNormalTableRow(&self, stateEnvoy: IStateEnvoy) { todo!() }
    pub fn onAdjustDrawing(&self, drawing: TypoDrawing, stateEnvoy: IStateEnvoy) { todo!() }
}

pub trait IStateEnvoy {}
pub trait ParentLayouterTask {}

// Rust stub for ColumnsLayouter (auto-generated)
// Original: cn/wps/moffice/writer/layout/pageview/columns/ColumnsLayouter.java
use crate::*;

pub struct ColumnsLayouter {
    // TODO: fields
}

impl ColumnsLayouter {
    pub fn new() -> Self { todo!() }
    pub fn dispose(&self) { todo!() }
    pub fn shouldRecycleFlowedColumns(&self, columns: TypoColumns, layoutState: LayoutState) -> bool { todo!() }
    pub fn flow(&self, layoutState: PageViewLayoutState) -> bool { todo!() }
    pub fn hasBottomPageFootNotes(&self) -> bool { todo!() }
    pub fn getColumnsHeight(&self, layoutState: LayoutState) -> i32 { todo!() }
    pub fn doFlow(&self, layoutState: LayoutState) { todo!() }
    pub fn recycleFloatsCacheAfterFlow(&self, layoutState: LayoutState) { todo!() }
    pub fn hasFinishColumns(&self) -> bool { todo!() }
    pub fn updatePageViewEnv(&self, layoutState: PageViewLayoutState) { todo!() }
    pub fn initColumnLayoutEnv(&self, layoutState: PageViewLayoutState, preColumnTotalHeight: i32) { todo!() }
    pub fn setColumnPageEnvRangeEnd(&self, colEnv: ColumnPageLayoutState, pageViewEnv: LayoutState) { todo!() }
    pub fn flowColumnPage(&self, columnInfo: ColumnInfo, layoutState: LayoutState) { todo!() }
    pub fn postFlow(&self) { todo!() }
    pub fn preFlow(&self, columnInfo: ColumnInfo, layoutState: LayoutState) { todo!() }
    pub fn calcColumnsInfo(&self, layoutState: LayoutState) { todo!() }
    pub fn adjustColumnsHeightToMini(&self, layoutState: PageViewLayoutState) { todo!() }
    pub fn restoreLastVaildColumns(&self, layoutState: PageViewLayoutState, validColumnsData: ValidColumnsData) -> bool { todo!() }
    pub fn saveVaildColumns(&self, layoutState: PageViewLayoutState, vaildColumnsData: ValidColumnsData) { todo!() }
    pub fn prepareColumnsHeight(&self, avgColumnPageHeight: i32) -> HeightsResult { todo!() }
    pub fn prepareColumnsHeightForPage(&self, page: i32, pageTop: i32, searchTopForPage: i32, prepareBefore: i32, prepareAfter: i32) -> HeightsResult { todo!() }
    pub fn prepareColumnsHeightForTableRow(&self, table: i32, tableTop: i32, searchTopForTable: i32, prepareBefore: i32, prepareAfter: i32) -> HeightsResult { todo!() }
    pub fn saveValidDrawingList(&self, drawingList: WIntArrayList, layoutState: PageViewLayoutState) { todo!() }
    pub fn restoreFromDrawingList(&self, drawingList: WIntArrayList, layoutState: PageViewLayoutState) { todo!() }
    pub fn clearDrawingsBeforeBalance(&self, layoutState: PageViewLayoutState) { todo!() }
    pub fn needBalance(&self, columnPage: i32) -> bool { todo!() }
    pub fn adjsutColumnsHeight(&self) { todo!() }
    pub fn getMinHeightForColumnPage(&self, columnPage: i32) -> i32 { todo!() }
    pub fn getMinHeightForFootnotes(&self, footnotes: i32) -> i32 { todo!() }
    pub fn getMinHeightForPage(&self, page: i32) -> i32 { todo!() }
    pub fn getMinHeightForTableCell(&self, cell: i32) -> i32 { todo!() }
    pub fn getMinHeightForTableRow(&self, row: i32) -> i32 { todo!() }
    pub fn getContentHeight(&self, columnPage: i32) -> i32 { todo!() }
    pub fn getAreaOfContent(&self, columnPage: i32) -> i32 { todo!() }
    pub fn resetColumnInfoIterator(&self, it: KPList::Iterator) { todo!() }
    pub fn reset(&self) { todo!() }
    pub fn copyState(&self, src: PageViewLayoutState, des: PageViewLayoutState, copyRangeAndPageEnv: bool) { todo!() }
    pub fn copyState_1(&self, src: LayoutState, des: LayoutState, copyRangeAndPageEnv: bool) { todo!() }
    pub fn copyPageEnv(&self, src: PageEnv, des: PageEnv) { todo!() }
    pub fn copyState_2(&self, src: TableCellLayoutState, des: TableCellLayoutState, copyRangeAndPageEnv: bool) { todo!() }
    pub fn copyState_3(&self, src: FootEndNoteEnv, des: FootEndNoteEnv, copyRangeAndPageEnv: bool) { todo!() }
}

pub struct HeightsResult { /* TODO */ }

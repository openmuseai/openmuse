#![allow(dead_code, non_snake_case, unused_variables, unused_mut, unused_imports)]
// Java: import cn.wps.base.assertion.Assert
// CARGO_OFF_SURFACE use util::base::assertion::Assert::Assert;
// Java: import cn.wps.moffice.drawing.IPos
use drawing::cn::wps::moffice::drawing::IPos::IPos;
// Java: TableInfo.getPos() returns the TablePos implementation of IPos.
use crate::cn::wps::moffice::writer::data::table::TablePos::TablePos;
// Java: import cn.wps.moffice.writer.base.ViewMode
// CARGO_OFF_SURFACE use crate::cn::wps::moffice::writer::base::ViewMode::ViewMode;
// Java: import cn.wps.moffice.writer.cache.TypoDrawing
use crate::cn::wps::moffice::writer::cache::TypoDrawing::TypoDrawing;
// Java: import cn.wps.moffice.writer.cache.TypoDrawingLayers
// CARGO_OFF_SURFACE use crate::cn::wps::moffice::writer::cache::TypoDrawingLayers::TypoDrawingLayers;
// Java: import cn.wps.moffice.writer.cache.TypoFootNotes
// CARGO_OFF_SURFACE use crate::cn::wps::moffice::writer::cache::TypoFootNotes::TypoFootNotes;
// Java: import cn.wps.moffice.writer.cache.TypoList
// CARGO_OFF_SURFACE use crate::cn::wps::moffice::writer::cache::TypoList::TypoList;
// Java: import cn.wps.moffice.writer.cache.TypoMemoryObj
// CARGO_OFF_SURFACE use crate::cn::wps::moffice::writer::cache::TypoMemoryObj::TypoMemoryObj;
// Java: import cn.wps.moffice.writer.cache.TypoObjBase
// CARGO_OFF_SURFACE use crate::cn::wps::moffice::writer::cache::TypoObjBase::TypoObjBase;
// Java: import cn.wps.moffice.writer.cache.TypoPage
use crate::cn::wps::moffice::writer::cache::TypoPage::TypoPage;
// Java: import cn.wps.moffice.writer.cache.TypoSnapshot
use crate::cn::wps::moffice::writer::cache::TypoSnapshot::TypoSnapshot;
// Java: import cn.wps.moffice.writer.cache.TypoTableRow
use crate::cn::wps::moffice::writer::cache::TypoTableRow::TypoTableRow;
// Java: import cn.wps.moffice.writer.cache.TypoPageLinesIterator
// CARGO_OFF_SURFACE use crate::cn::wps::moffice::writer::cache::TypoPageLinesIterator::TypoPageLinesIterator;
// Java: import cn.wps.moffice.writer.cache.TypoRangeBase
// CARGO_OFF_SURFACE use crate::cn::wps::moffice::writer::cache::TypoRangeBase::TypoRangeBase;
// Java: import cn.wps.moffice.writer.cache.TypoRectCoreImpl
// CARGO_OFF_SURFACE use crate::cn::wps::moffice::writer::cache::TypoRectCoreImpl::TypoRectCoreImpl;
// Java: import cn.wps.moffice.writer.cache.TypoSnapshot
// CARGO_OFF_SURFACE use crate::cn::wps::moffice::writer::cache::TypoSnapshot::TypoSnapshot;
// Java: import cn.wps.moffice.writer.cache.TypoTableCell
// CARGO_OFF_SURFACE use crate::cn::wps::moffice::writer::cache::TypoTableCell::TypoTableCell;
// Java: import cn.wps.moffice.writer.cache.TypoTableRow
// CARGO_OFF_SURFACE use crate::cn::wps::moffice::writer::cache::TypoTableRow::TypoTableRow;
// Java: import cn.wps.moffice.writer.cache.TypoTextBox
use crate::cn::wps::moffice::writer::cache::TypoTextBox::TypoTextBox;
// Java: import cn.wps.moffice.writer.cache.TypoToolsManager
// CARGO_OFF_SURFACE use crate::cn::wps::moffice::writer::cache::TypoToolsManager::TypoToolsManager;
// Java: import cn.wps.moffice.writer.core.Document
// CARGO_OFF_SURFACE use crate::cn::wps::moffice::writer::core::Document::Document;
// Java: import cn.wps.moffice.writer.data.table.RowInfo
// CARGO_OFF_SURFACE use crate::cn::wps::moffice::writer::data::table::RowInfo::RowInfo;
// Java: import cn.wps.moffice.writer.data.table.TableInfo
// CARGO_OFF_SURFACE use crate::cn::wps::moffice::writer::data::table::TableInfo::TableInfo;
// Java: import cn.wps.moffice.writer.data.table.TableJCData
// CARGO_OFF_SURFACE use crate::cn::wps::moffice::writer::data::table::TableJCData::TableJCData;
// Java: import cn.wps.moffice.writer.data.value.BRC
// CARGO_OFF_SURFACE use crate::cn::wps::moffice::writer::data::value::BRC::BRC;
// Java: import cn.wps.moffice.writer.layout.base.LayouterBase
// CARGO_OFF_SURFACE use crate::cn::wps::moffice::writer::layout::base::LayouterBase::LayouterBase;
// Java: import cn.wps.moffice.writer.layout.base.drawing.DrawingCacheTool
// CARGO_OFF_SURFACE use crate::cn::wps::moffice::writer::layout::base::drawing::DrawingCacheTool::DrawingCacheTool;
// Java: import cn.wps.moffice.writer.layout.base.drawing.DrawingCoordTool
// CARGO_OFF_SURFACE use crate::cn::wps::moffice::writer::layout::base::drawing::DrawingCoordTool::DrawingCoordTool;
// Java: import cn.wps.moffice.writer.layout.base.env.DocumentEnv
use crate::cn::wps::moffice::writer::layout::base::env::DocumentEnv::DocumentEnv;
// Java: import cn.wps.moffice.writer.layout.base.env.LayoutState
// CARGO_OFF_SURFACE use crate::cn::wps::moffice::writer::layout::base::env::LayoutState::LayoutState;
// Java: import cn.wps.moffice.writer.layout.base.env.MainState
use crate::cn::wps::moffice::writer::layout::base::env::MainState::MainState;
// Java: import cn.wps.moffice.writer.layout.base.env.PageEnv
// CARGO_OFF_SURFACE use crate::cn::wps::moffice::writer::layout::base::env::PageEnv::PageEnv;
// Java: import cn.wps.moffice.writer.layout.base.env.StateSnapshot
// CARGO_OFF_SURFACE use crate::cn::wps::moffice::writer::layout::base::env::StateSnapshot::StateSnapshot;
// Java: import cn.wps.moffice.writer.layout.base.env.ViewEnv
use crate::cn::wps::moffice::writer::layout::base::env::ViewEnv::ViewEnv;
// Java: import cn.wps.moffice.writer.layout.base.page.PageRangesCollector
// CARGO_OFF_SURFACE use crate::cn::wps::moffice::writer::layout::base::page::PageRangesCollector::PageRangesCollector;
// Java: import cn.wps.moffice.writer.layout.base.table.TableRowLayouter
use crate::cn::wps::moffice::writer::layout::base::table::TableRowLayouter::{CellGeom, RowFlowInput, RowFlowResult, TableRowLayouter};
// Java: import cn.wps.moffice.writer.layout.base.table.TableCellBrcTool
use crate::cn::wps::moffice::writer::layout::base::table::TableCellBrcTool::TableCellBrcTool;
// Java: import cn.wps.moffice.writer.layout.base.table.TableState
// CARGO_OFF_SURFACE use crate::cn::wps::moffice::writer::layout::base::table::TableState::TableState;
// Java: import cn.wps.moffice.writer.layout.base.table.TableCellWidthReceiver
// CARGO_OFF_SURFACE use crate::cn::wps::moffice::writer::layout::base::table::TableCellWidthReceiver::TableCellWidthReceiver;
use core::marker::PhantomData as _;
use crate::cn::wps::moffice::writer::cache::TypoList::TypoList as SnapList;
use crate::cn::wps::moffice::writer::cache::TypoRangeBase::TypoRangeBase as SnapRange;
use crate::cn::wps::moffice::writer::cache::TypoMemoryObj::TypoMemoryObj as SnapMem;
use crate::cn::wps::moffice::writer::cache::TypoTableCell::TypoTableCell as SnapCell;
use crate::cn::wps::moffice::writer::cache::TypoTextLine::TypoTextLine as SnapLine;
use crate::cn::wps::moffice::writer::layout::base::env::Pools::Pools;
use crate::cn::wps::moffice::writer::layout::base::env::TableCellLayoutState::TableCellLayoutState;

#[derive(Default, Clone)]
pub struct LineIt {
    pub page: i32,
    pub lines: i32,
    pub size: i32,
    pub pos: i32,
    snapshot: *mut TypoSnapshot,
}
impl LineIt {
    // java assignment(TypoPage, pos) then update reads getLines and TypoList.size
    pub fn assignment(&mut self, page: i32, pos: i32, snapshot: *mut TypoSnapshot) {
      self.page = page;
      self.snapshot = snapshot;
      self.update(pos);
    }
    pub fn update(&mut self, new_pos: i32) {
      if self.page != 0 && !self.snapshot.is_null() {
        let snapshot = unsafe { &*self.snapshot };
        self.lines = crate::cn::wps::moffice::writer::cache::TypoPage::TypoPage::getLines__int_typosnapshot(self.page, snapshot);
        self.size = if self.lines != 0 {
          crate::cn::wps::moffice::writer::cache::TypoList::TypoList::size__int_typosnapshot(self.lines, snapshot)
        } else {
          0
        };
      } else {
        self.lines = 0;
        self.size = 0;
      }
      self.pos = new_pos;
    }
    pub fn setChangeListerner(&mut self, _l: Option<&dyn IStateEnvoy>) {}
    pub fn value(&mut self) -> i32 {
      self.update(self.pos);
      if self.pos >= 0 && self.pos < self.size && !self.snapshot.is_null() {
        crate::cn::wps::moffice::writer::cache::TypoList::TypoList::get__int_int_typosnapshot(self.pos, self.lines, unsafe { &mut *self.snapshot })
      } else {
        0
      }
    }
    // java next() returns the item and then advances. Callers here only need the advance.
    pub fn next(&mut self) {
      self.update(self.pos);
      if self.pos < self.size {
        self.pos += 1;
      }
    }
    pub fn isEnd(&mut self) -> bool {
      self.update(self.pos);
      self.pos == -1 || self.pos >= self.size
    }
    pub fn remove(&mut self) -> i32 {
      self.update(self.pos);
      if self.pos < 0 || self.pos >= self.size {
        return 0;
      }
      let value = if !self.snapshot.is_null() {
        crate::cn::wps::moffice::writer::cache::TypoList::TypoList::get__int_int_typosnapshot(self.pos, self.lines, unsafe { &mut *self.snapshot })
      } else {
        0
      };
      if self.page != 0 && !self.snapshot.is_null() {
        let mut page = crate::cn::wps::moffice::writer::cache::TypoPage::TypoPage::default();
        page.snapshot = self.snapshot;
        page.snapshot_pointer = self.page;
        page.removeLine(self.pos);
      }
      self.update(self.pos);
      value
    }
}
#[derive(Default, Clone)]
pub struct TableState {
    pub lineIt: LineIt,
    pub hasBreakCellOfLayoutPage: bool,
    pub page: Option<TypoPage>,
    pub coreWidth: i32,
    pub firstLineOfPage: bool,
    pub curLayoutHeight: i32,
    pub maxLayoutHeight: i32,
    pub row: Option<RowInfo>,
    pub allowOverCoreHeight: bool,
    pub allowBreakLine: bool,
    pub stopFlowTableHeader: bool,
    pub rowHeightOverflow: bool,
    pub findUnlayoutCell: bool,
    pub findBreakCell: bool,
    pub tableHeader: bool,
    pub firstTableHeader: bool,
    pub isTableFit: bool,
    pub rowFinalProperty: i32,
    pub mLineBefore: i32,
    pub mLineAfter: i32,
    pub mIsCardBegin: bool,
    pub mIsCardEnd: bool,
    pub mIsInUiCard: bool,
    pub flowedNormal: i32,
    pub flowedWrap: i32,
    pub flowedRows: i32,
}
impl TableState {
    pub fn new() -> Self { Self::default() }
    pub fn dispose(&mut self) {
        // java row = null; rowFinalProperty = null; page = null; lineIt.recycle()
        self.row = None;
        self.rowFinalProperty = 0;
        self.page = None;
        self.lineIt = LineIt::default();
    }
    pub fn resetCardInfo(&mut self) {
        self.mLineBefore = 0;
        self.mLineAfter = 0;
        self.mIsCardBegin = false;
        self.mIsCardEnd = false;
        self.mIsInUiCard = false;
    }
}
#[derive(Default, Clone)]
pub struct RowInfo {
    pub index: i32,
    pub startCP: i32,
    pub endCP: i32,
    pub row_height: i32,
    pub cant_split: bool,
    pub table_header: bool,
    pub rtl: bool,
    pub rgdxa: Vec<i32>,
    pub cells: Vec<CellGeom>,
    // java row.getParent().getJC()
    pub jc: i32,
    pub jc_left: i32,
    pub jc_right: i32,
    pub table_indent: i32,
    // java row.getFinalProperty()
    pub final_property: util::moffice::property::PropertySet::PropertySet,
    // java RowInfo.isRevisionDelete / isRevisionInsert / isFormatChange
    pub revision_delete: bool,
    pub revision_insert: bool,
    pub format_change: bool,
}
impl RowInfo {
    pub fn getIndex(&self) -> i32 { self.index }
    pub fn getStartCP(&self) -> i32 { self.startCP }
    pub fn getEndCP(&self) -> i32 { self.endCP }
    pub fn getFinalProperty(&self) -> i32 { 0 }
    pub fn getRowHeight(&self) -> i32 { self.row_height }
    pub fn cantSplit(&self) -> bool { self.cant_split }
}
#[derive(Default, Clone)]
pub struct TableInfo {
    pub pos: Option<TablePos>,
    pub startCP: i32,
    pub endCP: i32,
    pub size: i32,
    pub complete: bool,
    pub tableFit: bool,
    pub level: i32,
    pub row: Option<RowInfo>,
    pub rows: Vec<RowInfo>,
    pub jc: TableJCData,
}
impl TableInfo {
    pub fn getPos(&self) -> Option<&dyn IPos> { self.pos.as_ref().map(|pos| pos as &dyn IPos) }
    pub fn getRowByCP(&self, cp: i32) -> Option<RowInfo> {
        self.rows.iter().find(|row| cp >= row.startCP && cp < row.endCP).cloned().or_else(|| self.row.clone())
    }
    pub fn getRowByIndex(&self, i: i32) -> Option<RowInfo> {
        self.rows.iter().find(|row| row.index == i).cloned().or_else(|| self.row.clone())
    }
    pub fn getStartCP(&self) -> i32 { self.startCP }
    pub fn getEndCP(&self) -> i32 { self.endCP }
    pub fn size(&self) -> i32 { self.size }
    pub fn isComplete(&self) -> bool { self.complete }
    pub fn isTableFit(&self) -> bool { self.tableFit }
    pub fn getLevel(&self) -> i32 { self.level }
    pub fn getJC(&self) -> TableJCData { self.jc.clone() }
    pub fn getData(&self) -> TableData { TableData::default() }
    pub fn requestAutoFitDelay(&mut self) {}
}
#[derive(Default, Clone)]
pub struct TableData {
    pub waitFitUpdate: bool,
}
impl TableData {
    pub fn isWaitFitUpdate(&self) -> bool { self.waitFitUpdate }
    pub fn setWaitFitUpdate(&mut self, v: bool) { self.waitFitUpdate = v; }
}
#[derive(Default)]
pub struct PageRangesCollector;
impl PageRangesCollector {
    pub fn new() -> Self { Self::default() }
}
#[derive(Clone)]
pub struct ParentLayouterTask {
    parent: *mut (),
    end_para: Option<unsafe fn(*mut (), *mut crate::cn::wps::moffice::writer::layout::base::env::SuperEnvoy::SuperEnvoy)>,
    adjust_wrap_table: Option<unsafe fn(*mut (), *mut TypoDrawing, *const TableInfo, i32, bool, *mut EnvLayoutState)>,
}
impl Default for ParentLayouterTask {
    fn default() -> Self { Self { parent: std::ptr::null_mut(), end_para: None, adjust_wrap_table: None } }
}
impl ParentLayouterTask {
    pub fn bind(&mut self, parent: *mut (), end_para: unsafe fn(*mut (), *mut crate::cn::wps::moffice::writer::layout::base::env::SuperEnvoy::SuperEnvoy), adjust_wrap_table: unsafe fn(*mut (), *mut TypoDrawing, *const TableInfo, i32, bool, *mut EnvLayoutState)) {
        self.parent = parent;
        self.end_para = Some(end_para);
        self.adjust_wrap_table = Some(adjust_wrap_table);
    }
    pub fn endParaLayouter(&mut self, envoy: *mut crate::cn::wps::moffice::writer::layout::base::env::SuperEnvoy::SuperEnvoy) {
        if let Some(call) = self.end_para { if !self.parent.is_null() && !envoy.is_null() { unsafe { call(self.parent, envoy); } } }
    }
    pub fn adjustWrapTable(&mut self, drawing: &mut TypoDrawing, table: &TableInfo, para_top: i32, need_cut: bool, state: &mut EnvLayoutState) {
        if let Some(call) = self.adjust_wrap_table { if !self.parent.is_null() { unsafe { call(self.parent, drawing, table, para_top, need_cut, state); } } }
    }
}
#[derive(Default)]
pub struct LayoutState {
    pub envoy: Option<Box<dyn IStateEnvoy>>,
}
impl LayoutState {
    pub fn getEnvoy(&mut self) -> &mut dyn IStateEnvoy {
        &mut **self.envoy.as_mut().expect("envoy")
    }
    pub fn isUiCardStartCP(&self, _cp: i32) -> bool { false }
    pub fn isUiCardEndCP(&self, _cp: i32) -> bool { false }
    pub fn isInUiCard(&self) -> bool { false }
}
#[derive(Default, Clone)]
pub struct TableJCData {
    pub jc: i32,
    pub jcLeft: i32,
    pub jcRight: i32,
    pub tableIndent: i32,
}
#[derive(Default, Clone)]
pub struct ErrorCollector {
    pub has_error: bool,
}
impl ErrorCollector {
    pub fn hasError(&self) -> bool { self.has_error }
}
#[derive(Default, Clone)]
pub struct TableInfos {
    pub table: Option<TableInfo>,
}
impl TableInfos {
    pub fn getTableInfoByLevel(&self, _cp: i32, _layer: i32) -> Option<TableInfo> { self.table.clone() }
}
#[derive(Default, Clone)]
pub struct Document {
    pub type_: i32,
    pub errorCollector: ErrorCollector,
    pub tableInfos: TableInfos,
}
impl Document {
    pub fn getType(&self) -> i32 { self.type_ }
    pub fn getErrorCollector(&self) -> &ErrorCollector { &self.errorCollector }
    pub fn getTableInfos(&self) -> &TableInfos { &self.tableInfos }
}
#[derive(Default, Clone)]
pub struct PageEnv {
    pub coreWidth: i32,
    pub maxLayoutHeight: i32,
    pub minLayoutHeight: i32,
    pub allowOverCoreHeight: bool,
    pub allowBreakLine: bool,
}
impl PageEnv {
    pub fn getCoreWidth(&self) -> i32 { self.coreWidth }
}
#[derive(Default)]
pub struct FitState {
    pub anyCellChanged: bool,
    pub endRowIndex: i32,
}
impl FitState {
    pub fn reSet(&mut self, _a: bool, endRowIndex: i32, _c: bool) {
        self.endRowIndex = endRowIndex;
        self.anyCellChanged = false;
    }
}
pub struct TableCellWidthReceiver;
impl TableCellWidthReceiver {
    pub fn getFitState() -> FitState { FitState::default() }
}
pub struct DrawingCoordTool;
impl DrawingCoordTool {
    pub fn isVertTextflow(page: &TypoPage) -> bool {
        0 != page.getVisualTextflow()
    }
    pub fn isVertTextflow__typopage(page: &TypoPage) -> bool {
        Self::isVertTextflow(page)
    }
}
pub struct TypoObjBase;
impl TypoObjBase {
    pub fn getBottom__int_typosnapshot(_p: i32, _s: &TypoSnapshot) -> i32 { 0 }
}
pub struct TypoList;
impl TypoList {
    pub fn get__int_int_typosnapshot(_a: i32, _b: i32, _s: &TypoSnapshot) -> i32 { 0 }
    pub fn last(_p: i32, _s: &TypoSnapshot) -> i32 { 0 }
    pub fn size(_p: i32, _s: &TypoSnapshot) -> i32 { 0 }
}
pub struct TypoRangeBase;
impl TypoRangeBase {
    pub fn getEndCP(_p: i32, _s: &TypoSnapshot) -> i32 { 0 }
}
pub struct TypoTextBoxStatic;
impl TypoTextBoxStatic {
    pub fn getLines(_p: i32, _s: &TypoSnapshot) -> i32 { 0 }
}
pub struct Tools;
impl Tools {
    // java public static int getTextFlow(int textFlow)
    pub fn getTextFlow(text_flow: i32) -> i32 {
        match text_flow {
            1 => 5,
            5 => 1,
            _ => text_flow,
        }
    }

    // java public static boolean isFirstLineOfPage(TypoPage curPage, TypoSnapshot snapshot)
    pub fn isFirstLineOfPage(cur_page: &TypoPage, snapshot: &TypoSnapshot) -> bool {
        let lines = cur_page.getLines();
        if lines == 0 || SnapList::size__int_typosnapshot(lines, snapshot) == 0 {
            return true;
        }
        let last = SnapList::last__int_typosnapshot(lines, snapshot);
        if SnapMem::getType__int_typosnapshot(last, snapshot) != 5 {
            return false;
        }
        if !TypoTableRow::isTableHeader__int_typosnapshot(last, snapshot) {
            return false;
        }
        let mut i = SnapList::size__int_typosnapshot(lines, snapshot) - 2;
        while i >= 0 {
            let line = SnapList::get__int_int_typosnapshot(i, lines, snapshot);
            if SnapMem::getType__int_typosnapshot(line, snapshot) != 5 {
                return false;
            }
            if !TypoTableRow::isTableHeader__int_typosnapshot(line, snapshot) || TypoTableRow::isFirstHeader__int_typosnapshot(line, snapshot) {
                return false;
            }
            i -= 1;
        }
        true
    }

    // java public static void createCellEnvsForTableRow(...)
    pub fn createCellEnvsForTableRow(
        layout_state: &mut EnvLayoutState,
        mut p_row: i32,
        set_page: bool,
        force_set_cell_envs: bool,
        doc_env: &mut DocumentEnv,
        main_state: &mut MainState,
    ) {
        if p_row == 0 { return; }
        let main_ptr = main_state as *mut MainState;
        let drained = layout_state.cellStateList.drain();
        doc_env.pools.recycleTableCellLayoutStates(drained);
        let snap = &mut main_state.snapshot as *mut TypoSnapshot;
        let snapshot = unsafe { &*snap };
        if TypoTableRow::isWrap__int_typosnapshot(p_row, snapshot) {
            let page_ptr = layout_state.curPage.as_ref().map(|page| page.getPointer()).unwrap_or(0);
            let page_type = layout_state.curPage.as_ref().map(|page| page.getType()).unwrap_or(0);
            let start_cp = SnapRange::getStartCP__int_typosnapshot(p_row, snapshot);
            let drawing = if let Some(doc) = layout_state.document.filter(|p| !p.is_null()) {
                let doc = unsafe { &*doc };
                if let Some(layers) = main_state.getDrawingLayers(doc) {
                    if page_type == 6 {
                        layers.getWrapTableByCP(start_cp, page_ptr)
                    } else {
                        layers.getWrapTableByCP(start_cp, 0)
                    }
                } else {
                    0
                }
            } else {
                0
            };
            let p_lines = if drawing != 0 {
                TypoDrawing::getTextbox__int_typosnapshot(drawing, unsafe { &*snap })
            } else {
                0
            };
            // java pTypoTextLines = TypoDrawing.getTextBoxLines(drawing); last line of that list replaces pRow
            let text_lines = if p_lines != 0 {
                crate::cn::wps::moffice::writer::cache::TypoTextBox::TypoTextBox::getLines__int_typosnapshot(p_lines, unsafe { &*snap })
            } else {
                0
            };
            if text_lines != 0 {
                p_row = unsafe { SnapList::last__int_typosnapshot(text_lines, &mut *snap) };
            }
            if p_row == 0 { return; }
        }
        let snapshot = unsafe { &*snap };
        if Self::has_break_cells(p_row, snapshot) {
            layout_state.cp = SnapRange::getStartCP__int_typosnapshot(p_row, snapshot);
            let size = TypoTableRow::getCellsCount__int_typosnapshot(p_row, snapshot);
            let mut i = 0;
            while i < size {
                let p_cell = TypoTableRow::getCell__int_int_typosnapshot(i, p_row, unsafe { &*snap });
                let mut cell = SnapCell::new();
                cell.assignment(p_cell, snap);
                let cur = Self::create_cell_env(layout_state, &cell, set_page, force_set_cell_envs, doc_env, main_ptr);
                layout_state.cellStateList.add(cur);
                i += 1;
            }
        } else {
            layout_state.cp = SnapRange::getEndCP__int_typosnapshot(p_row, unsafe { &*snap });
            if Self::has_vert_merged_cells(p_row, unsafe { &*snap }) || force_set_cell_envs {
                let mut merge_cells: Vec<TableCellLayoutState> = Vec::new();
                let size = TypoTableRow::getCellsCount__int_typosnapshot(p_row, unsafe { &*snap });
                let mut i = 0;
                while i < size {
                    let p_cell = TypoTableRow::getCell__int_int_typosnapshot(i, p_row, unsafe { &*snap });
                    if p_cell != 0 {
                        let mut cell = SnapCell::new();
                        cell.assignment(p_cell, snap);
                        let first = cell.getFirstVertMergeCell();
                        if first != 0 && SnapCell::isBreakCell__int_typosnapshot(first, unsafe { &*snap }) {
                            cell.assignment(first, snap);
                            merge_cells.push(Self::create_cell_env(layout_state, &cell, set_page, force_set_cell_envs, doc_env, main_ptr));
                        } else {
                            let cur = Self::create_cell_env(layout_state, &cell, set_page, force_set_cell_envs, doc_env, main_ptr);
                            if !cell.isBreakCell() && (!cur.vertMerged || cur.lastMerged) {
                                doc_env.pools.recycleTableCellLayoutState(cur);
                            } else {
                                merge_cells.push(cur);
                            }
                        }
                    }
                    i += 1;
                }
                if merge_cells.is_empty() { return; }
                let mut keep = false;
                for cell_env in &merge_cells {
                    if cell_env.breakCell || cell_env.firstMergerCellState.as_ref().is_some_and(|first| first.breakCell) {
                        keep = true;
                        break;
                    }
                }
                if !keep {
                    doc_env.pools.recycleTableCellLayoutStates(merge_cells);
                    return;
                }
                Self::create_env_for_merge_cells(layout_state, &mut merge_cells, doc_env);
            }
        }
    }

    fn has_break_cells(p_row: i32, snapshot: &TypoSnapshot) -> bool {
        if p_row == 0 { return false; }
        let size = TypoTableRow::getCellsCount__int_typosnapshot(p_row, snapshot);
        let mut i = 0;
        while i < size {
            let p_cell = TypoTableRow::getCell__int_int_typosnapshot(i, p_row, snapshot);
            if p_cell != 0
                && SnapCell::isBreakCell__int_typosnapshot(p_cell, snapshot)
                && !SnapCell::isVertMergeCell__int_typosnapshot(p_cell, snapshot)
            {
                return true;
            }
            i += 1;
        }
        false
    }

    fn has_vert_merged_cells(p_row: i32, snapshot: &TypoSnapshot) -> bool {
        if p_row == 0 { return false; }
        let size = TypoTableRow::getCellsCount__int_typosnapshot(p_row, snapshot);
        let mut i = 0;
        while i < size {
            let p_cell = TypoTableRow::getCell__int_int_typosnapshot(i, p_row, snapshot);
            if SnapCell::isVertMergeCell__int_typosnapshot(p_cell, snapshot)
                && !SnapCell::isLastVertMergeCell__int_typosnapshot(p_cell, snapshot)
            {
                return true;
            }
            i += 1;
        }
        false
    }

    fn create_cell_env(
        env: &mut EnvLayoutState,
        cell: &SnapCell,
        set_page: bool,
        force_set_cell_envs: bool,
        doc_env: &mut DocumentEnv,
        main: *mut MainState,
    ) -> TableCellLayoutState {
        let snap = unsafe { &mut (*main).snapshot as *mut TypoSnapshot };
        let mut cell_env = Self::create_table_cell_layout_env(env, &mut doc_env.pools);
        if let Some(info) = Self::cell_info(env, cell.getCellEndCp()) {
            cell_env.setCellnfo(info.clone());
            cell_env.super_.setRange__int_int(info.getStartCP(), info.getEndCP());
            cell_env.super_.pageEnv.textFlow = Self::getTextFlow(info.getTextFlow());
        }
        cell_env.super_.tablePeriLayer = env.tablePeriLayer + 1;
        cell_env.vertMerged = cell.isVertMergeCell();
        cell_env.lastMerged = cell.isLastVertMergeCell();
        cell_env.firstMerged = cell.isFirstVertMergeCell();
        cell_env.breakCell = cell.isBreakCell();
        cell_env.super_.pageEnv.marginLeft = cell.getMarginLeft();
        cell_env.super_.pageEnv.marginTop = cell.getMarginTop();
        cell_env.super_.pageEnv.marginRight = cell.getMarginRight();
        cell_env.super_.pageEnv.marginBottom = cell.getMarginBottom();
        if set_page {
            let mut page = TypoPage::default();
            page.snapshot = snap;
            page.snapshot_pointer = cell.getPointer();
            cell_env.super_.curPage = Some(page);
        }
        if cell_env.vertMerged && !cell_env.firstMerged {
            let first_ptr = cell.getFirstVertMergeCell();
            let mut first_cell = SnapCell::new();
            first_cell.assignment(first_ptr, snap);
            if first_cell.isFirstVertMergeCell() && (set_page || first_cell.isBreakCell()) {
                let first_state = Self::create_cell_env(env, &first_cell, set_page, force_set_cell_envs, doc_env, main);
                cell_env.firstMergerCellState = Some(Box::new(first_state));
            }
        }
        if cell_env.breakCell && (!cell_env.vertMerged || cell_env.firstMerged) && cell.hasLines() {
            let lines = cell.getLines();
            let last_line = unsafe { SnapList::last__int_typosnapshot(lines, &mut *snap) };
            let last_type = SnapMem::getType__int_typosnapshot(last_line, unsafe { &*snap });
            if last_type == 5 {
                Self::createCellEnvsForTableRow(&mut cell_env.super_, last_line, set_page, force_set_cell_envs, doc_env, unsafe { &mut *main });
            } else if last_type == 3 {
                cell_env.super_.cp = SnapLine::getEndCP__int_typosnapshot(last_line, unsafe { &*snap });
            } else {
                cell_env.super_.cp = SnapRange::getEndCP__int_typosnapshot(last_line, unsafe { &*snap });
            }
        } else {
            cell_env.super_.cp = cell_env.super_.getRangeEnd();
        }
        cell_env
    }

    fn create_env_for_merge_cells(env: &mut EnvLayoutState, merge_cells: &mut Vec<TableCellLayoutState>, doc_env: &mut DocumentEnv) {
        let Some(row) = Self::row_at(env) else {
            doc_env.pools.recycleTableCellLayoutStates(std::mem::take(merge_cells));
            return;
        };
        let cant_split = row.cantSplit();
        let cell_size = row.size();
        let mut i = 0;
        while i < cell_size {
            if let Some(cell_info) = row.getCellByIndex(i).cloned() {
                if cell_info.getVertMerge() == 1 {
                    let cur = if merge_cells.is_empty() { None } else { Some(merge_cells.remove(0)) };
                    match cur {
                        None => {
                            let new_cell = Self::create_cell_env_for_cell(env, &cell_info, &mut doc_env.pools);
                            env.cellStateList.add(new_cell);
                        }
                        Some(cur_cell) if cur_cell.firstMerged => {
                            let mut new_cell = Self::create_cell_env_for_cell(env, &cell_info, &mut doc_env.pools);
                            new_cell.super_.cp = cell_info.getStartCP();
                            new_cell.super_.curPage = cur_cell.super_.curPage.clone();
                            new_cell.firstMergerCellState = Some(Box::new(cur_cell));
                            env.cellStateList.add(new_cell);
                        }
                        Some(cur_cell) => {
                            // java curCellEnv.lastMerged = cellInfo.isLastVertMergeCell();
                            // The typo cell already stored lastMerged. CellInfoNode has no next-merge link to recompute it.
                            env.cellStateList.add(cur_cell);
                        }
                    }
                } else if cell_info.getHoriMerge() != 2 {
                    let mut new_cell = Self::create_cell_env_for_cell_split(env, &cell_info, cant_split, &mut doc_env.pools);
                    new_cell.super_.cp = cell_info.getStartCP();
                    env.cellStateList.add(new_cell);
                }
            }
            i += 1;
        }
        if !merge_cells.is_empty() {
            doc_env.pools.recycleTableCellLayoutStates(std::mem::take(merge_cells));
        }
    }

    fn create_cell_env_for_cell(env: &EnvLayoutState, cell_info: &crate::cn::wps::moffice::writer::data::table::tree::CellInfoNode::CellInfoNode, pools: &mut Pools) -> TableCellLayoutState {
        Self::create_cell_env_for_cell_split(env, cell_info, false, pools)
    }

    // java public static TableCellLayoutState createCellEnvForCell(LayoutState, CellInfo, boolean cantSplit, Pools)
    pub fn createCellEnvForCell(
        env: &EnvLayoutState,
        cell_info: &crate::cn::wps::moffice::writer::data::table::tree::CellInfoNode::CellInfoNode,
        cant_split: bool,
        pools: &mut Pools,
    ) -> TableCellLayoutState {
        Self::create_cell_env_for_cell_split(env, cell_info, cant_split, pools)
    }

    fn create_cell_env_for_cell_split(
        env: &EnvLayoutState,
        cell_info: &crate::cn::wps::moffice::writer::data::table::tree::CellInfoNode::CellInfoNode,
        cant_split: bool,
        pools: &mut Pools,
    ) -> TableCellLayoutState {
        let mut new_cell = Self::create_table_cell_layout_env(env, pools);
        new_cell.setCellnfo(cell_info.clone());
        new_cell.super_.setRange__int_int(cell_info.getStartCP(), cell_info.getEndCP());
        new_cell.super_.tablePeriLayer = env.tablePeriLayer + 1;
        if cell_info.isVertMergeCell() {
            if cell_info.isFirstVertMergeCell() {
                new_cell.vertMerged = true;
                new_cell.firstMerged = true;
            } else {
                new_cell.vertMerged = true;
                new_cell.lastMerged = false;
            }
        }
        new_cell.super_.pageEnv.marginTop = cell_info.margin_top();
        new_cell.super_.pageEnv.marginLeft = cell_info.margin_left();
        new_cell.super_.pageEnv.marginRight = cell_info.margin_right();
        new_cell.super_.pageEnv.marginBottom = cell_info.margin_bottom();
        new_cell.super_.pageEnv.textFlow = Self::getTextFlow(cell_info.getTextFlow());
        new_cell.cantSplit = if new_cell.vertMerged { false } else { cant_split };
        new_cell
    }

    fn create_table_cell_layout_env(env: &EnvLayoutState, pools: &mut Pools) -> TableCellLayoutState {
        let mut cell = pools.obtainTableCellLayoutState();
        cell.super_.document = env.document;
        cell.super_.pageEnv.gridHeight = env.pageEnv.gridHeight;
        cell.super_.pageEnv.gridWidth = env.pageEnv.gridWidth;
        cell.super_.pageEnv.gridModel = env.pageEnv.gridModel;
        cell.super_.pageEnv.charSpaceForGrid = env.pageEnv.charSpaceForGrid;
        cell.super_.pageEnv.allowOverCoreHeight = env.pageEnv.allowOverCoreHeight;
        cell.super_.textFrameLayer = env.textFrameLayer + 1;
        cell
    }

    fn cell_info(env: &EnvLayoutState, cp: i32) -> Option<crate::cn::wps::moffice::writer::data::table::tree::CellInfoNode::CellInfoNode> {
        let doc = env.document.filter(|p| !p.is_null())?;
        let infos = unsafe { (*doc).getTableInfos() }.filter(|p| !p.is_null())?;
        unsafe { (*infos).getCellInfoByLevel(cp, env.tablePeriLayer) }
    }

    fn row_at(env: &EnvLayoutState) -> Option<crate::cn::wps::moffice::writer::data::table::tree::RowInfoNode::RowInfoNode> {
        let doc = env.document.filter(|p| !p.is_null())?;
        let infos = unsafe { (*doc).getTableInfos() }.filter(|p| !p.is_null())?;
        let table = unsafe { (*infos).getTableInfoByLevel(env.cp, env.tablePeriLayer) }?;
        table.getRowByCP(env.cp).cloned()
    }
}
pub struct Assert;
impl Assert {
    pub fn fail__string(_m: &str) {}
    pub fn assertNotNull__object<T>(_v: T) {}
    pub fn assertNotNull(_msg: &str, v: &Option<RowInfo>) {
        let _ = (v.is_some(), _msg);
    }
}
pub struct Thread;
impl Thread {
    pub fn sleep__long(ms: i64) {
        #[cfg(not(target_family = "wasm"))]
        std::thread::sleep(std::time::Duration::from_millis(ms as u64));
        #[cfg(target_family = "wasm")]
        let _ = ms;
    }
}
pub trait IStateEnvoy {
    fn getDocument(&self) -> &dyn crate::cn::wps::moffice::writer::core::Document::Document;
    fn getCurPage(&self) -> &TypoPage;
    fn setHasFlowTableHeader(&mut self, v: bool);
    fn setValidLine(&mut self, v: bool);
    fn incrementCP(&mut self);
    fn getCP(&self) -> i32;
    fn setCP(&mut self, v: i32);
    fn getTablePeriLayer(&self) -> i32;
    fn getPageEnv(&self) -> &crate::cn::wps::moffice::writer::layout::base::env::PageEnv::PageEnv;
    fn getCurLayoutHeight(&self) -> i32;
    fn setCurLayoutHeight(&mut self, v: i32);
    fn getLinePos(&self) -> i32;
    fn setFinishPage(&mut self, v: bool);
    fn getCellStateList(&self) -> &crate::cn::wps::moffice::writer::layout::base::env::LayoutState::CellStateList;
    fn drainCellStates(&mut self) -> Vec<crate::cn::wps::moffice::writer::layout::base::env::TableCellLayoutState::TableCellLayoutState> { Vec::new() }
    fn as_para_envoy(&mut self) -> *mut crate::cn::wps::moffice::writer::layout::base::env::SuperEnvoy::SuperEnvoy { std::ptr::null_mut() }
}

impl IStateEnvoy for crate::cn::wps::moffice::writer::layout::base::env::SuperEnvoy::SuperEnvoy {
    fn getDocument(&self) -> &dyn crate::cn::wps::moffice::writer::core::Document::Document {
        crate::cn::wps::moffice::writer::layout::base::env::SuperEnvoy::SuperEnvoy::getDocument(self)
    }
    fn getCurPage(&self) -> &TypoPage {
        crate::cn::wps::moffice::writer::layout::base::env::SuperEnvoy::SuperEnvoy::getCurPage(self)
    }
    fn setHasFlowTableHeader(&mut self, v: bool) {
        crate::cn::wps::moffice::writer::layout::base::env::SuperEnvoy::SuperEnvoy::setHasFlowTableHeader(self, v);
    }
    fn setValidLine(&mut self, v: bool) {
        crate::cn::wps::moffice::writer::layout::base::env::SuperEnvoy::SuperEnvoy::setValidLine(self, v);
    }
    fn incrementCP(&mut self) {
        crate::cn::wps::moffice::writer::layout::base::env::SuperEnvoy::SuperEnvoy::incrementCP(self);
    }
    fn getCP(&self) -> i32 {
        crate::cn::wps::moffice::writer::layout::base::env::SuperEnvoy::SuperEnvoy::getCP(self)
    }
    fn setCP(&mut self, v: i32) {
        crate::cn::wps::moffice::writer::layout::base::env::SuperEnvoy::SuperEnvoy::setCP(self, v);
    }
    fn getTablePeriLayer(&self) -> i32 {
        crate::cn::wps::moffice::writer::layout::base::env::SuperEnvoy::SuperEnvoy::getTablePeriLayer(self)
    }
    fn getPageEnv(&self) -> &crate::cn::wps::moffice::writer::layout::base::env::PageEnv::PageEnv {
        crate::cn::wps::moffice::writer::layout::base::env::SuperEnvoy::SuperEnvoy::getPageEnv(self)
    }
    fn getCurLayoutHeight(&self) -> i32 {
        crate::cn::wps::moffice::writer::layout::base::env::SuperEnvoy::SuperEnvoy::getCurLayoutHeight(self)
    }
    fn setCurLayoutHeight(&mut self, v: i32) {
        crate::cn::wps::moffice::writer::layout::base::env::SuperEnvoy::SuperEnvoy::setCurLayoutHeight(self, v);
    }
    fn getLinePos(&self) -> i32 {
        crate::cn::wps::moffice::writer::layout::base::env::SuperEnvoy::SuperEnvoy::getLinePos(self)
    }
    fn setFinishPage(&mut self, v: bool) {
        crate::cn::wps::moffice::writer::layout::base::env::SuperEnvoy::SuperEnvoy::setFinishPage(self, v);
    }
    fn getCellStateList(&self) -> &crate::cn::wps::moffice::writer::layout::base::env::LayoutState::CellStateList {
        // Java: return this.mLayoutState.cellStateList;
        unsafe { &(*(self.getLayoutState() as *const EnvLayoutState)).cellStateList }
    }
    fn drainCellStates(&mut self) -> Vec<crate::cn::wps::moffice::writer::layout::base::env::TableCellLayoutState::TableCellLayoutState> {
        unsafe { (*(self.getLayoutState() as *mut EnvLayoutState)).cellStateList.drain() }
    }
    fn as_para_envoy(&mut self) -> *mut crate::cn::wps::moffice::writer::layout::base::env::SuperEnvoy::SuperEnvoy {
        self as *mut crate::cn::wps::moffice::writer::layout::base::env::SuperEnvoy::SuperEnvoy
    }
}

// java private static final String TAG = null;
const TAG: &str = "";

// java public class TableLayouter
pub struct TableLayouter {
    // java protected TableRowLayouter mTableRowLayouter;
    pub(crate) mTableRowLayouter: Option<TableRowLayouter>,
    // java protected TableRowLayouter mTableHeaderLayouter;
    pub(crate) mTableHeaderLayouter: Option<TableRowLayouter>,
    // java protected TableState mTableState;
    pub mTableState: TableState,
    // java protected RowInfo mRow;
    pub(crate) mRow: Option<RowInfo>,
    // java flowTableHeader reads parent.getRowByIndex(0) when that row is a header
    mHeaderRow: Option<RowInfo>,
    // java protected TableInfo mTable;
    pub mTable: Option<TableInfo>,
    // java protected boolean mIsInPage;
    pub(crate) mIsInPage: bool,
    // java protected boolean mFirstRowOfTable;
    pub(crate) mFirstRowOfTable: bool,
    // java protected PageRangesCollector mPageRangesCollector;
    pub(crate) mPageRangesCollector: Option<PageRangesCollector>,
    // java protected final ParentLayouterTask mLayouterTask;
    pub(crate) mLayouterTask: ParentLayouterTask,
    // java protected final DocumentEnv mDocEnv;
    pub(crate) mDocEnv: DocumentEnv,
    // java protected final ViewEnv mViewEnv;
    pub(crate) mViewEnv: ViewEnv,
    // java protected final MainState mMainState;
    pub(crate) mMainState: MainState,
    // Java passes the parent layouter's env by reference. Null uses the owned copies.
    mDocEnvAlias: *mut DocumentEnv,
    mViewEnvAlias: *mut ViewEnv,
    mMainStateAlias: *mut MainState,
}

type EnvLayoutState = crate::cn::wps::moffice::writer::layout::base::env::LayoutState::LayoutState;

// java public class TableLayouter
impl TableLayouter {
    fn doc_env(&self) -> &DocumentEnv {
        if self.mDocEnvAlias.is_null() { &self.mDocEnv } else { unsafe { &*self.mDocEnvAlias } }
    }

    fn main_state(&mut self) -> &mut MainState {
        if self.mMainStateAlias.is_null() { &mut self.mMainState } else { unsafe { &mut *self.mMainStateAlias } }
    }

    fn view_env(&self) -> &ViewEnv {
        if self.mViewEnvAlias.is_null() { &self.mViewEnv } else { unsafe { &*self.mViewEnvAlias } }
    }

    // Java `this` for the parent TextLayouter's DocumentEnv / ViewEnv / MainState.
    pub fn bind_env(&mut self, doc: *mut DocumentEnv, view: *mut ViewEnv, main: *mut MainState) {
        self.mDocEnvAlias = doc;
        self.mViewEnvAlias = view;
        self.mMainStateAlias = main;
    }

    // java ParentLayouterTask is the enclosing TextLayouter.
    pub fn bind_parent(&mut self, parent: *mut (), end_para: unsafe fn(*mut (), *mut crate::cn::wps::moffice::writer::layout::base::env::SuperEnvoy::SuperEnvoy), adjust_wrap_table: unsafe fn(*mut (), *mut TypoDrawing, *const TableInfo, i32, bool, *mut EnvLayoutState)) {
        self.mLayouterTask.bind(parent, end_para, adjust_wrap_table);
    }

    fn adjust_wrap_table(&mut self, drawing: &mut TypoDrawing, para_top: i32, need_cut: bool, state: &mut EnvLayoutState) {
        let Some(table) = self.mTable.clone() else { return; };
        self.mLayouterTask.adjustWrapTable(drawing, &table, para_top, need_cut, state);
    }

    fn end_para_layouter(&mut self, layout_state: &mut EnvLayoutState) {
        let envoy = layout_state.getEnvoy() as *mut crate::cn::wps::moffice::writer::layout::base::env::SuperEnvoy::SuperEnvoy;
        self.end_para_envoy(envoy);
    }

    fn end_para_envoy(&mut self, envoy: *mut crate::cn::wps::moffice::writer::layout::base::env::SuperEnvoy::SuperEnvoy) {
        self.mLayouterTask.endParaLayouter(envoy);
    }

    // java public  TableLayouter(ParentLayouterTask layouterTask, DocumentEnv docEnv, ViewEnv viewEnv, MainState mainState)
    pub fn TableLayouter(layouterTask: ParentLayouterTask, docEnv: DocumentEnv, viewEnv: ViewEnv, mainState: MainState) -> Self {
      // java this.mLayouterTask = layouterTask;
      let mut this = Self { mTableRowLayouter: Default::default(), mTableHeaderLayouter: Default::default(), mTableState: TableState::new(), mRow: Default::default(), mHeaderRow: None, mTable: Default::default(), mIsInPage: Default::default(), mFirstRowOfTable: Default::default(), mPageRangesCollector: Some(PageRangesCollector::new()), mLayouterTask: layouterTask, mDocEnv: docEnv, mViewEnv: viewEnv, mMainState: mainState, mDocEnvAlias: std::ptr::null_mut(), mViewEnvAlias: std::ptr::null_mut(), mMainStateAlias: std::ptr::null_mut() };
      // java this.mDocEnv = docEnv;
      // java this.mViewEnv = viewEnv;
      // java this.mMainState = mainState;
      // java this.mTableState = new TableState();
      // java this.mTableState.lineIt = this.mMainState.snapshot.getTypoTools().obtainTypoListIterator((TypoPage)null, -1);
      // rust: no-stmt — TypoSnapshot.getTypoTools is DEFERRED_NON_FUNCTION
      // java this.initTableRowLayouter();
      this.initTableRowLayouter();
      // java this.mPageRangesCollector = new PageRangesCollector();
      this
    }

    // java public void dispose()
    pub fn dispose(&mut self) {
      // java this.mRow = null;
      self.mRow = None;
      self.mHeaderRow = None;
      // java this.mTable = null;
      self.mTable = None;
      // java this.mTableState.dispose(); TableState is owned, so it is cleared in place.
      self.mTableState.dispose();
      // java if (this.mTableRowLayouter != null) { dispose(); mTableRowLayouter = null; }
      if let Some(mut layouter) = self.mTableRowLayouter.take() {
        layouter.dispose();
      }
      if let Some(mut layouter) = self.mTableHeaderLayouter.take() {
        layouter.dispose();
      }
    }

    // java public void reset()
    pub fn reset(&self) {
    }

    // java protected void initTableRowLayouter()
    pub fn initTableRowLayouter(&mut self) {
      // java this.mTableRowLayouter = new TableRowLayouter(this.mLayouterTask, this.mDocEnv, this.mViewEnv, this.mMainState, this.mTableState);
      self.mTableRowLayouter = Some(TableRowLayouter::new());
      // java this.mTableHeaderLayouter = new TableRowLayouter(this.mLayouterTask, this.mDocEnv, this.mViewEnv, this.mMainState, this.mTableState);
      self.mTableHeaderLayouter = Some(TableRowLayouter::new());
    }

    // java protected boolean hasTablePos(IStateEnvoy stateEnvoy)
    pub fn hasTablePos(&self, stateEnvoy: &dyn IStateEnvoy) -> bool {
      // java if (null != this.mTable.getPos())
      if self.mTable.as_ref().unwrap().getPos().is_some() {
        // java return true;
        return true;
      }
      // java else
      else {
        // java int type = stateEnvoy.getDocument().getType();
        let r#type: i32 = stateEnvoy.getDocument().getType__on_IDocument();
        // java return (0 == type || 2 == type) && DrawingCoordTool.isVertTextflow(stateEnvoy.getCurPage());
        return (0 == r#type || 2 == r#type) && DrawingCoordTool::isVertTextflow(stateEnvoy.getCurPage());
      }
    }

    // java public void flow(boolean mHasBreakRow, LayoutState layoutState)
    pub fn flow(&mut self, mHasBreakRow: bool, layoutState: &mut EnvLayoutState) {
      // java IStateEnvoy stateEnvoy = layoutState.getEnvoy();
      // java stateEnvoy.setHasFlowTableHeader(false);
      layoutState.getEnvoy().setHasFlowTableHeader(false);
      // java stateEnvoy.setValidLine(false);
      layoutState.getEnvoy().setValidLine(false);
      // java this.mTableState.hasBreakCellOfLayoutPage = mHasBreakRow;
      self.mTableState.hasBreakCellOfLayoutPage = mHasBreakRow;
      // java this.mTableState.resetCardInfo();
      self.mTableState.resetCardInfo();
      // java while (true)
      loop {
        // java this.getTableInfo(stateEnvoy);
        self.getTableInfo(layoutState);
        // java if (null == this.mTable)
        if self.mTable.is_none() {
          // java if (stateEnvoy.getDocument().getErrorCollector().hasError())
          let has_error = layoutState.document.map(|doc| unsafe {
            (*doc).getErrorCollector().map(|collector| collector.hasError()).unwrap_or(false)
          }).unwrap_or(false);
          if has_error {
            // java return;
            return;
          }
          // java Thread.sleep(10L);
          Thread::sleep__long(10);
          // java if (this.mDocEnv.textDocument.isInIOState())
          if self.doc_env().textDocument.as_ref().map(|d| d.isInIOState()).unwrap_or(false) {
            // java continue;
            continue;
          }
        }
        // java if (null == this.mTable)
        if self.mTable.is_none() {
          // java this.getTableInfo(stateEnvoy);
          self.getTableInfo(layoutState);
          // java if (null == this.mTable)
          if self.mTable.is_none() {
            // java Assert.fail("表格数据异常");
            Assert::fail__string("表格数据异常");
            // java stateEnvoy.incrementCP();
            layoutState.getEnvoy().incrementCP();
            // java return;
            return;
          }
        }
        // java this.initForCalcTableFit(stateEnvoy);
        self.initForCalcTableFit(layoutState);
        // java this.processCardInfo(layoutState, stateEnvoy);
        self.processCardInfo(layoutState);
        // java if (this.hasTablePos(stateEnvoy))
        if self.has_table_pos(layoutState) {
          // java this.flowWrapTable(stateEnvoy);
          self.flowWrapTable(layoutState);
        }
        // java else
        else {
          // java this.flowNormalTableRow(stateEnvoy);
          self.flowNormalTableRow(layoutState);
        }
        // java this.afterForCalTableFit();
        self.afterForCalTableFit();
        // java return;
        return;
      }
    }

    fn has_table_pos(&self, layoutState: &EnvLayoutState) -> bool {
      if self.mTable.as_ref().and_then(|table| table.getPos()).is_some() {
        return true;
      }
      let doc_type = layoutState.document.map(|doc| unsafe { (*doc).getType__on_IDocument() }).unwrap_or(-1);
      let visual = layoutState.curPage.as_ref().map(|page| page.getVisualTextflow()).unwrap_or(0);
      (doc_type == 0 || doc_type == 2) && visual != 0
    }

    // java private void initForCalcTableFit(IStateEnvoy stateEnvoy)
    pub fn initForCalcTableFit(&mut self, _layoutState: &mut EnvLayoutState) {
      // java if (this.checkRowNeedStartFit(stateEnvoy))
      if self.checkRowNeedStartFit() {
        // java TableCellWidthReceiver.FitState fitState = TableCellWidthReceiver.getFitState();
        let mut fitState: FitState = TableCellWidthReceiver::getFitState();
        // java fitState.reSet(true, this.mTable.size() - 1, this.mTable.getData().isWaitFitUpdate());
        fitState.reSet(true, self.mTable.as_ref().unwrap().size() - 1, self.mTable.as_ref().unwrap().getData().isWaitFitUpdate());
      }
    }

    // java protected boolean checkRowNeedStartFit(IStateEnvoy stateEnvoy)
    pub fn checkRowNeedStartFit(&self) -> bool {
      // java boolean ret = this.mTable.isTableFit() && this.mTable.isComplete() && this.mTable.getLevel() == 1;
      let mut ret: bool = self.mTable.as_ref().unwrap().isTableFit() && self.mTable.as_ref().unwrap().isComplete() && self.mTable.as_ref().unwrap().getLevel() == 1;
      // java if (!ret)
      if !ret {
        // java return false;
        return false;
      }
      // java else
      else {
        // java if (this.mRow.getIndex() == 0)
        if self.mRow.as_ref().map(|r| r.getIndex()).unwrap_or(0) == 0 {
          // java ret = true;
          ret = true;
        }
        // java else if (this.mTable.getData().isWaitFitUpdate())
        else if self.mTable.as_ref().unwrap().getData().isWaitFitUpdate() {
          // java ret = true;
          ret = true;
        }
        // java return ret;
        return ret;
      }
    }

    // java protected void afterForCalTableFit()
    pub fn afterForCalTableFit(&mut self) {
      // java if (this.mTable.isTableFit() && this.mTable.getLevel() == 1)
      if self.mTable.as_ref().map(|t| t.isTableFit() && t.getLevel() == 1).unwrap_or(false) {
        // java TableCellWidthReceiver.FitState fitState = TableCellWidthReceiver.getFitState();
        let mut fitState: FitState = TableCellWidthReceiver::getFitState();
        let _ = &mut fitState;
      }
    }

    fn row_info(node: &crate::cn::wps::moffice::writer::data::table::tree::TableInfoNode::TableInfoNode, index: i32, row: &crate::cn::wps::moffice::writer::data::table::tree::RowInfoNode::RowInfoNode, rtl: bool, document: &dyn crate::cn::wps::moffice::writer::core::Document::Document, jc: &crate::cn::wps::moffice::writer::data::table::TableJCData::TableJCData) -> RowInfo {
      let rgdxa = (0..row.rgdxa_count()).map(|i| row.rgdxa(i)).collect();
      let cells = row.cells().iter().enumerate().map(|(cell_index, cell)| {
        let (vert_merged, first_merged, last_merged) = node.cell_env_flags(index as usize, cell_index);
        CellGeom {
          start: cell.getStartCP(),
          end: cell.getEndCP(),
          hori_merge: cell.getHoriMerge(),
          vert_merged,
          first_merged,
          last_merged,
          vert_align: cell.getVertAlign(),
          fit_text: cell.getFitText(),
          margin_top: cell.margin_top(),
          margin_left: cell.margin_left(),
          margin_right: cell.margin_right(),
          margin_bottom: cell.margin_bottom(),
          text_flow: cell.getTextFlow(),
          left: row.rgdxa(cell_index as i32),
          right: row.rgdxa(cell_index as i32 + 1),
        }
      }).collect();
      RowInfo {
        index,
        startCP: row.getStartCP(),
        endCP: row.getEndCP(),
        row_height: row.getRowHeight(),
        cant_split: row.cantSplit(),
        table_header: row.isTableHeader(),
        rtl,
        rgdxa,
        cells,
        jc: jc.jc as i32,
        jc_left: jc.jcLeft,
        jc_right: jc.jcRight,
        table_indent: jc.tableIndent,
        final_property: crate::cn::wps::moffice::writer::data::table::TableTool::TableTool::getFinalRowProperty(document, row.getEndCP().saturating_sub(1)),
        revision_delete: row.isRevisionDelete(),
        revision_insert: row.isRevisionInsert(),
        format_change: row.isFormatChange(),
      }
    }

    // java private void getTableInfo(IStateEnvoy stateEnvoy)
    pub fn getTableInfo(&mut self, layoutState: &mut EnvLayoutState) {
      // java this.mTable = stateEnvoy.getDocument().getTableInfos().getTableInfoByLevel(stateEnvoy.getCP(), stateEnvoy.getTablePeriLayer());
      let cp = layoutState.cp;
      let layer = layoutState.tablePeriLayer;
      let doc = layoutState.document;
      let node = doc.and_then(|document| unsafe { (*document).getTableInfos() })
        .and_then(|infos| unsafe { (*infos).getTableInfoByLevel(cp, layer) });
      if let Some(mut node) = node {
        let doc_ref = unsafe { &*doc.expect("document") };
        let jc = node.getJC();
        let index = node.rows().iter().position(|row| row.contains_cp(cp)).unwrap_or(0) as i32;
        let rtl = node.isRTL();
        let rows: Vec<RowInfo> = node.rows().iter().enumerate().map(|(i, row)| {
          Self::row_info(&node, i as i32, row, rtl, doc_ref, &jc)
        }).collect();
        self.mRow = rows.iter().find(|row| row.index == index).cloned();
        self.mHeaderRow = if index != 0 {
          rows.first().filter(|row| row.table_header).cloned()
        } else {
          None
        };
        self.mTable = Some(TableInfo {
          // Java: this.mTable = ...getTableInfoByLevel(...); its getPos() is the imported IPos.
          pos: node.getPos().cloned(),
          startCP: node.getStartCP(),
          endCP: node.getEndCP(),
          size: node.row_count(),
          complete: node.isComplete(),
          tableFit: node.isTableFit(doc_ref),
          level: node.getLevel(),
          row: self.mRow.clone(),
          rows,
          jc: TableJCData { jc: jc.jc as i32, jcLeft: jc.jcLeft, jcRight: jc.jcRight, tableIndent: jc.tableIndent },
        });
        // java Assert.assertNotNull("Row should not be null!", this.mRow);
        Assert::assertNotNull("Row should not be null!", &self.mRow);
      } else {
        self.mTable = None;
        self.mRow = None;
        self.mHeaderRow = None;
      }
    }

    // java protected void getAllowOverCoreHeightForNormal(IStateEnvoy stateEnvoy)
    pub fn getAllowOverCoreHeightForNormal(&mut self, stateEnvoy: &mut dyn IStateEnvoy) {
      // java PageEnv pageEnv = stateEnvoy.getPageEnv();
      let page_env = stateEnvoy.getPageEnv();
      // java this.mTableState.allowOverCoreHeight = pageEnv.allowOverCoreHeight;
      self.mTableState.allowOverCoreHeight = page_env.allowOverCoreHeight;
      // java this.mTableState.allowBreakLine = pageEnv.allowBreakLine;
      self.mTableState.allowBreakLine = page_env.allowBreakLine;
    }

    // java protected void getAllowOverCoreHeightForWrap(IStateEnvoy stateEnvoy)
    pub fn getAllowOverCoreHeightForWrap(&mut self, stateEnvoy: &mut dyn IStateEnvoy) {
      // java boolean allowOverCoreHeight = stateEnvoy.getTablePeriLayer() > 1;
      let mut allowOverCoreHeight: bool = stateEnvoy.getTablePeriLayer() > 1;
      // java allowOverCoreHeight |= this.mDocEnv.notBreakWrappedTables;
      allowOverCoreHeight = allowOverCoreHeight | self.mDocEnv.notBreakWrappedTables;
      // java this.mTableState.allowOverCoreHeight = allowOverCoreHeight;
      self.mTableState.allowOverCoreHeight = allowOverCoreHeight;
      // java this.mTableState.allowBreakLine = stateEnvoy.getPageEnv().allowBreakLine;
      self.mTableState.allowBreakLine = stateEnvoy.getPageEnv().allowBreakLine;
    }

    // java protected boolean shouldCutWrapTableIfExceeded(IPos pos, IStateEnvoy stateEnvoy)
    pub fn shouldCutWrapTableIfExceeded(&self, pos: Option<&dyn IPos>, stateEnvoy: &dyn IStateEnvoy) -> bool {
      // java if (stateEnvoy.getTablePeriLayer() > 1)
      if stateEnvoy.getTablePeriLayer() > 1 {
        // java return false;
        return false;
      }
      // java else if (this.mDocEnv.notBreakWrappedTables)
      else if self.doc_env().notBreakWrappedTables {
        // java return false;
        return false;
      }
      // java else if (null != pos)
      else if let Some(pos) = pos {
        // java return 2 == pos.getRelativeY();
        return 2 == pos.getRelativeY();
      }
      // java else
      else {
        // java return false;
        return false;
      }
    }

    fn flow_row_layouter(&mut self, layoutState: &mut EnvLayoutState) {
      let Some(row) = self.mRow.clone() else { return; };
      let result = self.flow_one_row(layoutState, &row, false);
      self.mTableState.rowHeightOverflow = result.row_height_overflow;
      self.mTableState.tableHeader = result.table_header;
      self.mTableState.firstTableHeader = result.first_table_header;
    }

    // java protected void calcOffsetForJC(int coreWidth, IStateEnvoy stateEnvoy)
    fn calc_offset_for_jc(row: &RowInfo, core_width: i32, doc_env: &DocumentEnv, page_pointer: i32, snapshot: &mut crate::cn::wps::moffice::writer::cache::TypoSnapshot::TypoSnapshot) -> i32 {
      let mut offset: i32 = 0;
      let prop = &row.final_property;
      if doc_env.alignTablesRowByRow && prop.contains(296) {
        if let Some(length) = Self::table_length(prop, 296) {
          offset = -length.getTWIPS();
        }
      }
      let jc = row.jc;
      let jc_left = row.jc_left;
      let jc_right = row.jc_right;
      let table_indent = row.table_indent;
      if Self::use_layout_page_jc(page_pointer, snapshot) {
        match jc {
          0 => {
            if prop.contains(322) {
              offset = prop.getInt__int_int(322, 0) - jc_left;
            } else {
              let margin = if doc_env.isTableIndentedByOldSTyle { -Self::default_cell_margin_left(prop) } else { 0.0f32 };
              offset = (offset as f32 + margin - jc_left as f32 + table_indent as f32) as i32;
            }
          }
          1 => {
            offset += core_width / 2 - (jc_left + jc_right) / 2;
          }
          2 => {
            let margin = if doc_env.isTableIndentedByOldSTyle { Self::default_cell_margin_right(prop, row.cells.len() as i32) } else { 0.0f32 };
            offset = (offset as f32 + (core_width - jc_right) as f32 + margin) as i32;
          }
          _ => {}
        }
      } else {
        match jc {
          0 => offset += -jc_left + table_indent,
          1 => offset += core_width / 2 - (jc_left + jc_right) / 2,
          2 => offset += core_width - jc_right,
          _ => {}
        }
      }
      offset
    }

    fn table_length(prop: &util::moffice::property::PropertySet::PropertySet, id: i32) -> Option<crate::cn::wps::moffice::writer::data::value::TableLength::TableLength> {
      prop.getValue(id).and_then(|value| value.as_any().downcast_ref::<crate::cn::wps::moffice::writer::data::value::TableLength::TableLength>().cloned())
    }

    fn cell_margins(prop: &util::moffice::property::PropertySet::PropertySet) -> Option<Vec<crate::cn::wps::moffice::writer::data::value::CellMargin::CellMargin>> {
      let value = prop.getValue(301)?;
      if let Some(margins) = value.as_any().downcast_ref::<Vec<crate::cn::wps::moffice::writer::data::value::CellMargin::CellMargin>>() {
        return Some(margins.clone());
      }
      value.as_any().downcast_ref::<std::sync::Arc<Vec<crate::cn::wps::moffice::writer::data::value::CellMargin::CellMargin>>>().map(|margins| (**margins).clone())
    }

    // java private float getDefaultCellMarginLeft()
    fn default_cell_margin_left(prop: &util::moffice::property::PropertySet::PropertySet) -> f32 {
      let Some(margins) = Self::cell_margins(prop) else { return 108.0; };
      for cell_margin in &margins {
        if 0 >= cell_margin.getFirst() && 0 < cell_margin.getBeyond() {
          let length = cell_margin.getCellSpacingValue();
          if 0 != (cell_margin.getGrbitChg() & 2) && length.isValid() {
            return length.getTWIPS() as f32;
          }
        }
      }
      108.0
    }

    // java private float getDefaultCellMarginRight()
    fn default_cell_margin_right(prop: &util::moffice::property::PropertySet::PropertySet, cell_count: i32) -> f32 {
      let last_cell_index = cell_count - 1;
      let Some(margins) = Self::cell_margins(prop) else { return 108.0; };
      for cell_margin in &margins {
        if last_cell_index >= cell_margin.getFirst() && last_cell_index < cell_margin.getBeyond() {
          let length = cell_margin.getCellSpacingValue();
          if 0 != (cell_margin.getGrbitChg() & 8) && length.isValid() {
            return length.getTWIPS() as f32;
          }
        }
      }
      for cell_margin in &margins {
        if cell_margin.getFirst() == 0 && cell_margin.getBeyond() == 1 {
          let length = cell_margin.getCellSpacingValue();
          if 0 != (cell_margin.getGrbitChg() & 8) && length.isValid() {
            return length.getTWIPS() as f32;
          }
        }
      }
      108.0
    }

    // java private boolean useLayoutPageJC(IStateEnvoy stateEnvoy)
    fn use_layout_page_jc(page_pointer: i32, snapshot: &mut crate::cn::wps::moffice::writer::cache::TypoSnapshot::TypoSnapshot) -> bool {
      let page_type = if page_pointer == 0 { 0 } else { snapshot.get__int(page_pointer) };
      if page_type == 2 || page_type == 9 || page_type == 14 {
        return true;
      }
      if page_type == 8 {
        let parent = crate::cn::wps::moffice::writer::cache::TypoObjBase::TypoObjBase::getParent__int_typosnapshot(page_pointer, snapshot);
        if crate::cn::wps::moffice::writer::cache::TypoMemoryObj::TypoMemoryObj::getType__int_typosnapshot(parent, snapshot) == 11 {
          let outer = crate::cn::wps::moffice::writer::cache::TypoObjBase::TypoObjBase::getParent__int_typosnapshot(parent, snapshot);
          return !Self::is_in_columns(outer, snapshot);
        }
        return true;
      }
      false
    }

    // java private boolean isInColumns(int parent)
    fn is_in_columns(mut parent: i32, snapshot: &mut crate::cn::wps::moffice::writer::cache::TypoSnapshot::TypoSnapshot) -> bool {
      while parent != 0 {
        if crate::cn::wps::moffice::writer::cache::TypoMemoryObj::TypoMemoryObj::getType__int_typosnapshot(parent, snapshot) == 14 {
          return true;
        }
        parent = crate::cn::wps::moffice::writer::cache::TypoObjBase::TypoObjBase::getParent__int_typosnapshot(parent, snapshot);
      }
      false
    }

    // java TableRowLayouter.prepareEnvs when the page does not already carry a broken row.
    fn fill_cell_envs(
        layout_state: &mut EnvLayoutState,
        row_index: i32,
        start_cp: i32,
        pools: &mut Pools,
        layouter: &mut TableRowLayouter,
    ) {
        let Some(doc) = layout_state.document.filter(|pointer| !pointer.is_null()) else { return };
        let Some(infos) = (unsafe { (*doc).getTableInfos() }).filter(|pointer| !pointer.is_null()) else { return };
        let Some(mut table) = (unsafe { (*infos).getTableInfoByLevel(start_cp, layout_state.tablePeriLayer) }) else { return };
        // Java: this.mTable is one shared TableInfo. getJC() has already validated
        // every RowInfo before TableRowLayouter.prepareEnvs reads merge flags.
        // The Rust cache facade returns an owned clone, so validate that clone
        // before reading CellInfo.getVertMerge()/isVertMergeCell().
        table.getJC();
        let Some(row) = table.getRowByCP(start_cp).cloned() else { return };
        let cant_split = row.cantSplit();
        let mut i = 0;
        while i < row.size() {
            let Some(cell) = row.getCellByIndex(i).cloned() else { i += 1; continue; };
            if cell.getHoriMerge() == 2 {
                i += 1;
                continue;
            }
            let (vert, first, last) = table.cell_env_flags(row_index as usize, i as usize);
            if std::env::var_os("WORD_TABLE_FLOW_TRACE").is_some() && (100..=169).contains(&cell.getStartCP()) && i == 0 {
                eprintln!("table_cell_env row={} rows={} cp={} merge={} flags={vert}/{first}/{last}",
                    row_index, table.row_count(), cell.getStartCP(), cell.getVertMerge());
            }
            // java getVertMerge()==1 && findValidFirstVMergeCell goes through prepareEnvsForVertMerge
            if cell.getVertMerge() == 1 && vert {
                if let Some(mut carried) = layouter.pop_merged() {
                    if carried.firstMerged {
                        let mut new_cell = Tools::createCellEnvForCell(layout_state, &cell, false, pools);
                        new_cell.vertMerged = vert;
                        new_cell.firstMerged = first;
                        new_cell.lastMerged = last;
                        new_cell.cantSplit = false;
                        new_cell.super_.curPage = carried.super_.curPage.clone();
                        new_cell.firstMergerCellState = Some(Box::new(carried));
                        new_cell.super_.cp = cell.getStartCP();
                        layout_state.cellStateList.add(new_cell);
                    } else {
                        carried.lastMerged = last;
                        carried.vertMerged = vert;
                        carried.setCellnfo(cell.clone());
                        carried.super_.setRange__int_int(cell.getStartCP(), cell.getEndCP());
                        carried.super_.cp = cell.getStartCP();
                        layout_state.cellStateList.add(carried);
                    }
                } else {
                    let mut new_cell = Tools::createCellEnvForCell(layout_state, &cell, false, pools);
                    new_cell.vertMerged = vert;
                    new_cell.firstMerged = first;
                    new_cell.lastMerged = last;
                    new_cell.cantSplit = false;
                    new_cell.super_.cp = cell.getStartCP();
                    layout_state.cellStateList.add(new_cell);
                }
            } else {
                let mut new_cell = Tools::createCellEnvForCell(layout_state, &cell, cant_split, pools);
                new_cell.vertMerged = vert;
                new_cell.firstMerged = first;
                new_cell.lastMerged = last;
                new_cell.cantSplit = if vert { false } else { cant_split };
                new_cell.super_.cp = cell.getStartCP();
                layout_state.cellStateList.add(new_cell);
            }
            i += 1;
        }
    }

    // java prepareEnvs when hasBreakCellOfLayoutPage: repair the first merge env and, on a new page, insert a mock row.
    fn repair_break_envs(
        layout_state: &mut EnvLayoutState,
        row_index: i32,
        start_cp: i32,
        pools: &mut Pools,
        snapshot: &mut TypoSnapshot,
        page_pointer: i32,
        mut lines: i32,
        mut line_pos: i32,
        cur_layout_height: i32,
        jc_offset: i32,
        is_table_fit: bool,
        rtl: bool,
        first_line_of_page: bool,
        doc_type: i32,
    ) -> (i32, i32) {
        let Some(doc) = layout_state.document.filter(|pointer| !pointer.is_null()) else { return (lines, line_pos); };
        let Some(infos) = (unsafe { (*doc).getTableInfos() }).filter(|pointer| !pointer.is_null()) else { return (lines, line_pos); };
        let Some(table) = (unsafe { (*infos).getTableInfoByLevel(start_cp, layout_state.tablePeriLayer) }) else { return (lines, line_pos); };
        let Some(row) = table.getRowByCP(start_cp).cloned() else { return (lines, line_pos); };
        let mut cell_index = 0i32;
        let mut env_index = 0usize;
        while env_index < layout_state.cellStateList.len() {
            let mut cell = None;
            while cell_index < row.size() {
                let Some(next) = row.getCellByIndex(cell_index).cloned() else { break; };
                cell_index += 1;
                if next.getHoriMerge() == 2 {
                    continue;
                }
                cell = Some(next);
                break;
            }
            let Some(cell) = cell else { return (lines, line_pos); };
            let (vert, _first, _last) = table.cell_env_flags(row_index as usize, (cell_index - 1) as usize);
            let (missing_first, missing_page) = {
                let env = unsafe { &*layout_state.cellStateList.ptr_at(env_index) };
                let missing_page = env.firstMergerCellState.as_ref().map(|first| first.super_.curPage.is_none()).unwrap_or(true);
                (env.firstMergerCellState.is_none(), missing_page)
            };
            if cell.getVertMerge() == 1 && vert && missing_page {
                if let Some((first_row_index, first_cell_index)) = table.first_vert_merge_cell(row_index as usize, (cell_index - 1) as usize) {
                    if first_row_index != row_index as usize || first_cell_index != (cell_index - 1) as usize {
                        if missing_first {
                            if let Some(first_cell) = table.rows().get(first_row_index).and_then(|first_row| first_row.getCellByIndex(first_cell_index as i32)).cloned() {
                                let mut first_env = Tools::createCellEnvForCell(layout_state, &first_cell, false, pools);
                                first_env.super_.cp = first_env.super_.getRangeEnd();
                                first_env.atLeastOneLine = true;
                                if first_line_of_page {
                                    let (new_lines, new_pos) = Self::add_mock_row(
                                        &mut first_env,
                                        &first_cell,
                                        snapshot,
                                        page_pointer,
                                        lines,
                                        line_pos,
                                        cur_layout_height,
                                        jc_offset,
                                        is_table_fit,
                                        rtl,
                                        doc,
                                        doc_type,
                                    );
                                    lines = new_lines;
                                    line_pos = new_pos;
                                }
                                let env = unsafe { &mut *layout_state.cellStateList.ptr_at(env_index) };
                                env.firstMergerCellState = Some(Box::new(first_env));
                            }
                        } else if first_line_of_page {
                            let first_cell = {
                                let env = unsafe { &*layout_state.cellStateList.ptr_at(env_index) };
                                env.firstMergerCellState.as_ref().and_then(|first| first.getCellInfo().cloned())
                            };
                            if let Some(first_cell) = first_cell {
                                let env = unsafe { &mut *layout_state.cellStateList.ptr_at(env_index) };
                                if let Some(first_env) = env.firstMergerCellState.as_mut() {
                                    let (new_lines, new_pos) = Self::add_mock_row(
                                        first_env,
                                        &first_cell,
                                        snapshot,
                                        page_pointer,
                                        lines,
                                        line_pos,
                                        cur_layout_height,
                                        jc_offset,
                                        is_table_fit,
                                        rtl,
                                        doc,
                                        doc_type,
                                    );
                                    lines = new_lines;
                                    line_pos = new_pos;
                                }
                            }
                        }
                    }
                }
            }
            let env = unsafe { &mut *layout_state.cellStateList.ptr_at(env_index) };
            env.atLeastOneLine = true;
            if let Some(first) = env.firstMergerCellState.as_mut() {
                first.atLeastOneLine = true;
            }
            env_index += 1;
        }
        (lines, line_pos)
    }

    // java addMockRow for one continued first-merge cell
    fn add_mock_row(
        first_env: &mut TableCellLayoutState,
        cell: &crate::cn::wps::moffice::writer::data::table::tree::CellInfoNode::CellInfoNode,
        snapshot: &mut TypoSnapshot,
        page_pointer: i32,
        lines: i32,
        line_pos: i32,
        cur_layout_height: i32,
        jc_offset: i32,
        is_table_fit: bool,
        rtl: bool,
        document: *mut dyn crate::cn::wps::moffice::writer::core::Document::Document,
        doc_type: i32,
    ) -> (i32, i32) {
        let mut mock = TypoTableRow::default();
        mock.alloc(1, snapshot);
        mock.setParent(page_pointer);
        mock.offset(0, cur_layout_height);
        mock.setHeight(0);
        mock.setHasBreakCell(true);
        if !document.is_null() {
            let doc_ref = unsafe { &*document };
            let level = first_env.super_.tablePeriLayer.saturating_sub(1);
            if let Some(infos) = doc_ref.getTableInfos().filter(|pointer| !pointer.is_null()) {
                if let Some(table) = unsafe { (*infos).getTableInfoByLevel(cell.getStartCP(), level) } {
                    if let Some(row) = table.getRowByCP(cell.getStartCP()) {
                        mock.setRange(doc_ref, row.getStartCP(), row.getEndCP());
                        if row.rgdxa_count() > 0 {
                            mock.setLeft(row.getRgdxa(0));
                            mock.setRight(row.getRgdxa(row.rgdxa_count() - 1));
                        }
                        mock.setCantSplit(row.cantSplit());
                        mock.setFixedRowHeight(row.getRowHeight() < 0);
                    }
                }
            }
        }
        mock.setRTL(rtl);
        mock.setTableFit(is_table_fit);
        mock.setOffsetOfJc(jc_offset);
        let mut typo_cell = crate::cn::wps::moffice::writer::cache::TypoTableCell::TypoTableCell::default();
        typo_cell.alloc(snapshot);
        typo_cell.setParent(mock.getPointer());
        typo_cell.setVertMergeCell(first_env.vertMerged);
        typo_cell.setVertAlign(cell.getVertAlign());
        typo_cell.setFirstVertMergeCell(first_env.firstMerged);
        typo_cell.setLastVertMergeCell(first_env.lastMerged);
        typo_cell.setFitText(cell.getFitText());
        typo_cell.setDocumentType(doc_type);
        if !document.is_null() {
            typo_cell.setCellEndCP__int_document(first_env.super_.getRangeEnd() - 1, unsafe { &*document });
        }
        let vertical = first_env.super_.pageEnv.textFlow != 0 && first_env.super_.pageEnv.textFlow != 4;
        if vertical {
            typo_cell.setLeft(first_env.getRgdxaCenterLeft());
            typo_cell.setRight(first_env.getRgdxaCenterRight());
        }
        mock.setCell(0, typo_cell.getPointer());
        let mut page = crate::cn::wps::moffice::writer::cache::TypoPage::TypoPage::default();
        page.snapshot = snapshot as *mut TypoSnapshot;
        page.snapshot_pointer = typo_cell.getPointer();
        first_env.super_.curPage = Some(page);
        let lines = SnapList::insert__int_int_int_typosnapshot(mock.getPointer(), line_pos, lines, snapshot);
        (lines, line_pos + 1)
    }

    fn flow_one_row(&mut self, layoutState: &mut EnvLayoutState, row: &RowInfo, repeat_header: bool) -> RowFlowResult {
      self.flow_one_row_at(layoutState, row, repeat_header, None)
    }

    // Java lineIt.assignment(textbox, 0) then TableRowLayouter.flow into that page.
    fn flow_one_row_at(&mut self, layoutState: &mut EnvLayoutState, row: &RowInfo, repeat_header: bool, host: Option<(i32, i32, i32, i32)>) -> RowFlowResult {
      let main = if self.mMainStateAlias.is_null() { &mut self.mMainState as *mut MainState } else { self.mMainStateAlias };
      let bind_page = host.is_none();
      let (page_pointer, lines, line_pos, cur_layout_height) = if let Some(host) = host {
        host
      } else {
        if let Some(page) = layoutState.curPage.as_mut() {
          page.ensure_snapshot_pointer(unsafe { &mut (*main).snapshot });
        }
        (
          layoutState.curPage.as_ref().map(|page| page.getPointer()).unwrap_or(0),
          layoutState.curPage.as_ref().map(|page| page.getLines()).unwrap_or(0),
          layoutState.linePos,
          self.mTableState.curLayoutHeight,
        )
      };
      let doc = layoutState.document;
      let doc_type = doc.map(|document| unsafe { (*document).getType__on_IDocument() }).unwrap_or(0);
      let doc_env = if self.mDocEnvAlias.is_null() { &mut self.mDocEnv as *mut DocumentEnv } else { self.mDocEnvAlias };
      let mut input = RowFlowInput {
        doc_env: Some(doc_env),
        main_state: Some(main),
        page_state: Some(layoutState as *mut _),
        index: row.index,
        start_cp: row.startCP,
        end_cp: row.endCP,
        row_height: row.row_height,
        cant_split: row.cant_split,
        table_header: row.table_header,
        is_table_fit: self.mTableState.isTableFit,
        rtl: row.rtl,
        rgdxa: row.rgdxa.clone(),
        cells: row.cells.clone(),
        first_line_of_page: self.mTableState.firstLineOfPage,
        at_least_one_line: layoutState.atLeastOneLine,
        allow_over_core_height: self.mTableState.allowOverCoreHeight,
        allow_break_line: self.mTableState.allowBreakLine,
        cur_layout_height,
        max_layout_height: self.mTableState.maxLayoutHeight,
        page_pointer,
        line_pos,
        lines,
        view_mode: self.view_env().getViewMode(),
        card_begin: self.mTableState.mIsCardBegin,
        card_end: self.mTableState.mIsCardEnd,
        line_before: self.mTableState.mLineBefore,
        line_after: self.mTableState.mLineAfter,
        doc_type,
        repeat_header,
        table_peri_layer: layoutState.tablePeriLayer,
        grid_model: layoutState.pageEnv.gridModel,
        grid_height: layoutState.pageEnv.gridHeight,
        grid_width: layoutState.pageEnv.gridWidth,
        char_space_for_grid: layoutState.pageEnv.charSpaceForGrid,
        jc_offset: Self::calc_offset_for_jc(row, self.mTableState.coreWidth, self.doc_env(), page_pointer, unsafe { &mut (*main).snapshot }),
        cell_states: Some(&mut layoutState.cellStateList as *mut _),
        has_break_of_page: self.mTableState.hasBreakCellOfLayoutPage,
        row_index: row.index,
        revision_delete: row.revision_delete,
        revision_insert: row.revision_insert,
        format_change: row.format_change,
        revision_hidden: {
          let view = self.view_env();
          let revision_final = view.isRevisionFinal() || !view.isShowRevision();
          let makeup = view.getShowRevisionMakeup();
          let balloon = view.isShowRevisionInsOrDelInBalloons();
          let mut hidden = false;
          if row.revision_delete && revision_final {
            hidden = if makeup { balloon } else { true };
          }
          if row.revision_insert && !revision_final {
            hidden = if makeup { balloon } else { true };
          }
          hidden
        },
        revisions: Some(unsafe { &mut (*main).revisions as *mut _ }),
        typo_document: Some(unsafe { &mut (*doc_env).typoDocument as *mut _ }),
      };
      if !self.mTableState.hasBreakCellOfLayoutPage {
        let pools = unsafe { &mut (*doc_env).pools as *mut Pools };
        if let Some(layouter) = if repeat_header { self.mTableHeaderLayouter.as_mut() } else { self.mTableRowLayouter.as_mut() } {
          layouter.collect_merge_cells(&mut layoutState.cellStateList, unsafe { &mut *pools });
          Self::fill_cell_envs(layoutState, row.index, row.startCP, unsafe { &mut *pools }, layouter);
        }
      } else {
        // java prepareEnvs else: the page already holds this broken row's cell envs
        let pools = unsafe { &mut (*doc_env).pools as *mut Pools };
        let (lines, line_pos) = Self::repair_break_envs(
          layoutState,
          row.index,
          row.startCP,
          unsafe { &mut *pools },
          unsafe { &mut (*main).snapshot },
          page_pointer,
          input.lines,
          input.line_pos,
          cur_layout_height,
          input.jc_offset,
          self.mTableState.isTableFit,
          row.rtl,
          self.mTableState.firstLineOfPage,
          doc_type,
        );
        input.lines = lines;
        input.line_pos = line_pos;
      }
      let layouter = if repeat_header { self.mTableHeaderLayouter.as_mut() } else { self.mTableRowLayouter.as_mut() };
      let Some(layouter) = layouter else { return RowFlowResult::default(); };
      let result = layouter.flow(&input, unsafe { &mut (*main).snapshot }, doc);
      self.mTableState.findBreakCell = result.find_break_cell;
      if bind_page {
        if let Some(page) = layoutState.curPage.as_mut() {
          page.setLines(result.lines);
        }
      } else if page_pointer != 0 {
        // Java: TypoPageLinesIterator.insert writes the possibly reallocated line list
        // back to the TypoTextBox page. RowFlowResult carries that pointer here.
        unsafe { (*main).snapshot.set__int_int(page_pointer + 17, result.lines); }
      }
      result
    }

    // java protected void flowNormalTableRow(IStateEnvoy stateEnvoy)
    pub fn flowNormalTableRow(&mut self, layoutState: &mut EnvLayoutState) {
      // java this.mTableState.page = stateEnvoy.getCurPage();
      self.mTableState.page = layoutState.curPage.clone();
      // java this.mTableState.coreWidth = stateEnvoy.getPageEnv().getCoreWidth();
      self.mTableState.coreWidth = layoutState.pageEnv.getCoreWidth();
      // java this.mTableState.firstLineOfPage = Tools.isFirstLineOfPage(stateEnvoy.getCurPage(), this.mMainState.snapshot);
      self.mTableState.firstLineOfPage = layoutState.curPage.as_ref().map(|page| !page.hasLines()).unwrap_or(true);
      // java this.mTableState.lineIt.assignment(stateEnvoy.getCurPage(), stateEnvoy.getLinePos());
      let page = layoutState.curPage.as_ref().map(|page| page.getPointer()).unwrap_or(0);
      let main = if self.mMainStateAlias.is_null() { &mut self.mMainState as *mut MainState } else { self.mMainStateAlias };
      self.mTableState.lineIt.assignment(page, layoutState.linePos, unsafe { &mut (*main).snapshot });
      // java this.mTableState.curLayoutHeight = stateEnvoy.getCurLayoutHeight();
      self.mTableState.curLayoutHeight = layoutState.curLayoutHeight;
      // java this.mTableState.maxLayoutHeight = stateEnvoy.getPageEnv().maxLayoutHeight;
      self.mTableState.maxLayoutHeight = layoutState.pageEnv.maxLayoutHeight;
      // java this.mTableState.row = this.mRow;
      self.mTableState.row = self.mRow.clone();
      // java if (this.mTableHeaderLayouter != null) this.mTableHeaderLayouter.flowTableHeader(stateEnvoy);
      if self.mTableState.firstLineOfPage && layoutState.cellStateList.isEmpty() && !self.mTableState.stopFlowTableHeader {
        if let Some(header) = self.mHeaderRow.clone() {
          let header_result = self.flow_one_row(layoutState, &header, true);
          if header_result.row_pointer != 0 {
            layoutState.hasFlowTableHeader = true;
            layoutState.curLayoutHeight = header_result.cur_layout_height;
            layoutState.linePos += 1;
            self.mTableState.curLayoutHeight = header_result.cur_layout_height;
          }
        }
      }
      // java this.getAllowOverCoreHeightForNormal(stateEnvoy);
      self.mTableState.allowOverCoreHeight = layoutState.pageEnv.allowOverCoreHeight;
      self.mTableState.allowBreakLine = layoutState.pageEnv.allowBreakLine;
      // java this.setIsFirstRowOfTable(stateEnvoy);
      let index = self.mRow.as_ref().map(|row| row.getIndex()).unwrap_or(0);
      self.mFirstRowOfTable = index == 0 && layoutState.cellStateList.isEmpty();
      // java if (this.mFirstRowOfTable) this.mTableState.stopFlowTableHeader = false;
      if self.mFirstRowOfTable {
        self.mTableState.stopFlowTableHeader = false;
      }
      // java this.mTableState.rowFinalProperty = this.mTableState.row.getFinalProperty();
      self.mTableState.rowFinalProperty = self.mTableState.row.as_ref().map(|row| row.getFinalProperty()).unwrap_or(0);
      // java this.mTableState.isTableFit = this.mTable.isTableFit();
      self.mTableState.isTableFit = self.mTable.as_ref().map(|table| table.isTableFit()).unwrap_or(false);
      // java this.mTableRowLayouter.flow(stateEnvoy);
      self.flow_row_layouter(layoutState);
      // java this.postFlowNormalTableRow(stateEnvoy);
      self.mTableState.lineIt.pos = layoutState.linePos;
      self.postFlowNormalTableRow(layoutState.getEnvoy());
      self.mTableState.flowedNormal += 1;
    }

    // java private boolean hasLinesOfDrawing(TypoDrawing drawing)
    pub fn hasLinesOfDrawing(&self, drawing: &TypoDrawing) -> bool {
      // java int pTypoTextBox = drawing.getTextbox();
      let snapshot = if self.mMainStateAlias.is_null() {
        &self.mMainState.snapshot
      } else {
        unsafe { &(*self.mMainStateAlias).snapshot }
      };
      let pTypoTextBox = TypoDrawing::getTextbox__int_typosnapshot(drawing.getPointer(), snapshot);
      // java if (pTypoTextBox != 0)
      if pTypoTextBox != 0 {
        // java int plines = TypoTextBox.getLines(pTypoTextBox, this.mMainState.snapshot);
        let plines = TypoTextBox::getLines__int_typosnapshot(pTypoTextBox, snapshot);
        // java if (plines != 0)
        if plines != 0 {
          // java return TypoList.size(plines, this.mMainState.snapshot) > 0;
          return crate::cn::wps::moffice::writer::cache::TypoList::TypoList::size__int_typosnapshot(plines, snapshot) > 0;
        }
      }
      // java return false;
      false
    }

    // java protected void flowWrapTable(IStateEnvoy stateEnvoy)
    pub fn flowWrapTable(&mut self, layoutState: &mut EnvLayoutState) {
      // java this.getAllowOverCoreHeightForWrap(stateEnvoy);
      self.mTableState.allowOverCoreHeight = layoutState.tablePeriLayer > 1 || self.doc_env().notBreakWrappedTables;
      self.mTableState.allowBreakLine = layoutState.pageEnv.allowBreakLine;
      // java this.mIsInPage = false;
      self.mIsInPage = false;
      // java this.setIsFirstRowOfTable(stateEnvoy);
      self.setIsFirstRowOfTable(layoutState.getEnvoy());
      let mut drawing = self.getWrapTableDrawing(layoutState);
      if !drawing.isShadow() && !self.hasLinesOfDrawing(&drawing) {
        // java stateEnvoy.setCP(this.mTable.getStartCP());
        if let Some(table) = self.mTable.as_ref() {
          layoutState.cp = table.getStartCP();
        }
      } else {
        // java TypoTableRow tableRow = this.getTableRowForAnchor(wrapTableDrawing, stateEnvoy);
        let table_row = self.getTableRowForAnchor(&drawing, layoutState.getEnvoy());
        // java if (this.mIsInPage) Tools.createCellEnvsForTableRow(layoutState, tableRow, true, true, mDocEnv, mMainState);
        if self.mIsInPage {
            let doc = if self.mDocEnvAlias.is_null() { &mut self.mDocEnv as *mut DocumentEnv } else { self.mDocEnvAlias };
            let main = if self.mMainStateAlias.is_null() { &mut self.mMainState as *mut MainState } else { self.mMainStateAlias };
            unsafe {
                Tools::createCellEnvsForTableRow(layoutState, table_row.getPointer(), true, true, &mut *doc, &mut *main);
            }
        }
        // java stateEnvoy.setValidLine(true);
        layoutState.validLine = true;
        // java if (this.mIsInPage) { ... setFinishPage when CP is still inside the table }
        if self.mIsInPage {
          if let Some(table) = self.mTable.as_ref() {
            if layoutState.cp < table.getEndCP() {
              layoutState.setFinishPage(true);
            }
          }
        }
      }
      let _ = &mut drawing;
      self.mTableState.flowedWrap += 1;
    }

    // java protected TypoTableRow getTableRowForAnchor(TypoDrawing wrapTableDrawing, IStateEnvoy stateEnvoy)
    pub fn getTableRowForAnchor(&mut self, wrapTableDrawing: &TypoDrawing, stateEnvoy: &mut dyn IStateEnvoy) -> TypoTableRow {
      let main = if self.mMainStateAlias.is_null() { &mut self.mMainState as *mut MainState } else { self.mMainStateAlias };
      let mut tableRow = TypoTableRow::new();
      let page_ptr = stateEnvoy.getCurPage().getPointer();
      let line_pos = stateEnvoy.getLinePos();
      let y = stateEnvoy.getCurLayoutHeight();
      {
        let snapshot = unsafe { &mut (*main).snapshot };
        // java tableRow.alloc(0, snapshot); offsetTo(0, curLayoutHeight); height 1; wrap; parent = page
        tableRow.alloc(0, snapshot);
        tableRow.offset(0, y);
        tableRow.setHeight(1);
        tableRow.setWrap(true);
        tableRow.setParent(page_ptr);
        tableRow.setCardEnd(self.mTableState.mIsCardEnd);
        tableRow.setCardBegin(self.mTableState.mIsCardBegin);
        tableRow.setLineBefore(self.mTableState.mLineBefore);
        tableRow.setLineAfter(self.mTableState.mLineAfter);
        // java curPage.insertLine(tableRow.getPointer(), linePos)
        if page_ptr != 0 {
          let lines = TypoPage::getLines__int_typosnapshot(page_ptr, snapshot);
          let lines = crate::cn::wps::moffice::writer::cache::TypoList::TypoList::insert__int_int_int_typosnapshot(
            tableRow.getPointer(), line_pos, lines, snapshot,
          );
          snapshot.set__int_int(page_ptr + 17, lines);
        }
        let pTypoText = wrapTableDrawing.getTextbox();
        if pTypoText != 0 {
          let pLines = TypoPage::getLines__int_typosnapshot(pTypoText, snapshot);
          if pLines != 0 {
            let last = crate::cn::wps::moffice::writer::cache::TypoList::TypoList::last__int_typosnapshot(pLines, snapshot);
            if last != 0 {
              let endCp = crate::cn::wps::moffice::writer::cache::TypoRangeBase::TypoRangeBase::getEndCP__int_typosnapshot(last, snapshot);
              let doc = stateEnvoy.getDocument();
              tableRow.setRange(doc, wrapTableDrawing.getCP(), endCp);
              let mut drawing = wrapTableDrawing.clone();
              drawing.setAnchorCP(wrapTableDrawing.getCP(), doc);
            }
          }
        }
      }
      tableRow
    }

    // java protected TypoDrawing getWrapTableDrawing(IStateEnvoy stateEnvoy)
    pub fn getWrapTableDrawing(&mut self, layoutState: &mut EnvLayoutState) -> TypoDrawing {
      let row_start = layoutState.cp;
      let page_ptr = layoutState.curPage.as_ref().map(|page| page.getPointer()).unwrap_or(0);
      let doc_ptr = layoutState.document;
      let main = if self.mMainStateAlias.is_null() { &mut self.mMainState as *mut MainState } else { self.mMainStateAlias };
      let existing = if let Some(doc) = doc_ptr {
        let found = unsafe { (*main).getDrawingLayers__document_boolean(&*doc, true) }
          .map(|layers| layers.getWrapTableByCP(row_start, page_ptr))
          .unwrap_or(0);
        found
      } else {
        0
      };
      if existing != 0 {
        // java mIsInPage = true; return obtainTypoDrawing(pDrawing)
        self.mIsInPage = true;
        let mut drawing = TypoDrawing::new();
        drawing.assignment(existing, unsafe { &mut (*main).snapshot });
        return drawing;
      }
      let mut drawing = TypoDrawing::new();
      unsafe { drawing.alloc(&mut (*main).snapshot); }
      drawing.setDrawingType(7);
      drawing.setWrap(true);
      if let Some(doc) = doc_ptr {
        drawing.setCP(row_start, unsafe { &*doc });
      }
      let mut textbox = TypoTextBox::new();
      unsafe { textbox.alloc(&mut (*main).snapshot); }
      drawing.setTextbox(textbox.getPointer());
      // Java: typoTextBox.setParent(drawing.getPointer()). The Rust cache
      // wrapper exposes the inherited TypoObjBase slot through this static facade.
      unsafe {
        TypoTextBox::setParent__int_int_typosnapshot(
          drawing.getPointer(), textbox.getPointer(), &mut (*main).snapshot,
        );
      }
      let doc_type = doc_ptr.map(|doc| unsafe { (*doc).getType__on_IDocument() }).unwrap_or(0);
      textbox.setDocumentType(doc_type);
      textbox.setMargin(0, 0, 0, 0);
      // java firstLineOfPage = !curPage.hasLines(); maxLayoutHeight = wrap table height
      self.mTableState.firstLineOfPage = layoutState.curPage.as_ref().map(|page| !page.hasLines()).unwrap_or(true);
      self.mTableState.maxLayoutHeight = layoutState.pageEnv.maxLayoutHeight.wrapping_sub(layoutState.pageEnv.minLayoutHeight);
      drawing.setParent(page_ptr);
      if std::env::var_os("WORD_PARITY_WRAP_TRACE").is_some() {
        eprintln!("wrap_table_drawing cp={} ptr={} parent={} wrap={}", row_start, drawing.getPointer(), page_ptr, drawing.isWrap());
      }
      if let Some(doc) = doc_ptr {
        if let Some(layers) = unsafe { (*main).getDrawingLayers__document_boolean(&*doc, true) } {
          if std::env::var_os("WORD_PARITY_WRAP_TRACE").is_some() {
            eprintln!("wrap_table_layers layers={} wraps_before={}", layers.getPointer(), layers.getWraps());
          }
          if layers.getPointer() != 0 {
            layers.appendDrawingObject(drawing.getPointer(), Some(unsafe { &*doc }));
            if std::env::var_os("WORD_PARITY_WRAP_TRACE").is_some() {
              eprintln!("wrap_table_layers wraps_after={}", layers.getWraps());
            }
          }
        }
      }
      let textbox_pointer = textbox.getPointer();
      let need_cut = self.mTable.as_ref()
        .and_then(|table| table.getPos())
        .is_some_and(|pos| self.shouldCutWrapTableIfExceeded(Some(pos), layoutState.getEnvoy()));
      // java StateSnapshot cellSnapshot = pools.obtainStateSnapshot();
      // java ((LayoutState.HostEnvoy)stateEnvoy).saveToSnapShot(cellSnapshot, mDocEnv);
      let mut cell_snapshot = crate::cn::wps::moffice::writer::layout::base::env::SuperEnvoy::StateSnapshot::default();
      let doc = if self.mDocEnvAlias.is_null() { &mut self.mDocEnv as *mut DocumentEnv } else { self.mDocEnvAlias };
      layoutState.getEnvoy().saveToSnapShot(&mut cell_snapshot, unsafe { &mut *doc });
      let org_has_break = self.mTableState.hasBreakCellOfLayoutPage;
      self.flow_wrap_rows(&mut drawing, layoutState);
      if std::env::var_os("WORD_PARITY_TABLE_DRAW_TRACE").is_some() {
        if let Some(pos) = self.mTable.as_ref().and_then(|table| table.getPos()) {
          eprintln!("table_pos before cp={} box={},{},{},{} rel={},{} align={},{} paraTop={} needCut={}",
            row_start, drawing.getLeft(), drawing.getTop(), drawing.getRight(), drawing.getBottom(),
            pos.getRelativeX(), pos.getRelativeY(), pos.getAlignmentH(), pos.getAlignmentV(),
            layoutState.curLayoutHeight, need_cut);
        }
      }
      let para_top = layoutState.curLayoutHeight;
      self.adjust_wrap_table(&mut drawing, para_top, need_cut, layoutState);
      if !self.mFirstRowOfTable {
        let offset = layoutState.pageEnv.minLayoutHeight.wrapping_sub(drawing.getTop());
        drawing.offset(0, offset);
      }
      // java isOverFlow: keep the drawing and mark overflow, or cut and flow the textbox again.
      if drawing.getBottom() > layoutState.pageEnv.maxLayoutHeight {
        if !need_cut {
          drawing.setWrapTableOverFlow(true);
          if self.checkOverlapToFootNote(&drawing) {
            self.recycle_page_content(textbox_pointer);
            self.remove_drawings_for_parent(textbox_pointer, doc_ptr);
            drawing.setTextbox(0);
            layoutState.setFinishPage(true);
            return drawing;
          }
        } else {
          let first_line_height = self.embed_min_first_line(textbox_pointer);
          let remaining = layoutState.pageEnv.maxLayoutHeight.wrapping_sub(drawing.getTop());
          self.mTableState.maxLayoutHeight = first_line_height.max(remaining);
          self.mTableState.firstLineOfPage = layoutState.curPage.as_ref().map(|page| !page.hasLines()).unwrap_or(true);
          self.recycle_page_content(textbox_pointer);
          self.remove_drawings_for_parent(textbox_pointer, doc_ptr);
          // java stateEnvoy.setCP(rowStartCP); recycle cell states; restoreFromSnapShot; hasBreakCell bak
          let doc = if self.mDocEnvAlias.is_null() { &mut self.mDocEnv as *mut DocumentEnv } else { self.mDocEnvAlias };
          layoutState.getEnvoy().restoreFromSnapShot(&mut cell_snapshot, unsafe { &mut *doc });
          self.mTableState.hasBreakCellOfLayoutPage = org_has_break;
          self.flow_wrap_rows(&mut drawing, layoutState);
          self.adjust_wrap_table(&mut drawing, para_top, need_cut, layoutState);
          if !self.mFirstRowOfTable {
            let offset = layoutState.pageEnv.minLayoutHeight.wrapping_sub(drawing.getTop());
            drawing.offset(0, offset);
          }
          if drawing.getBottom() > layoutState.pageEnv.maxLayoutHeight {
            drawing.setWrapTableOverFlow(true);
          }
        }
      }
      if std::env::var_os("WORD_PARITY_TABLE_DRAW_TRACE").is_some() {
        let main = if self.mMainStateAlias.is_null() { &mut self.mMainState as *mut MainState } else { self.mMainStateAlias };
        let snapshot = unsafe { &mut (*main).snapshot };
        let lines = TypoTextBox::getLines__int_typosnapshot(textbox_pointer, snapshot);
        let size = crate::cn::wps::moffice::writer::cache::TypoList::TypoList::size__int_typosnapshot(lines, snapshot);
        eprintln!("table_draw drawing cp={} box={},{},{},{} rows={}", row_start, drawing.getLeft(), drawing.getTop(), drawing.getRight(), drawing.getBottom(), size);
        for i in 0..size {
          let pointer = crate::cn::wps::moffice::writer::cache::TypoList::TypoList::quickGet__int_int_typosnapshot(i, lines, snapshot);
          if crate::cn::wps::moffice::writer::cache::TypoMemoryObj::TypoMemoryObj::getType__int_typosnapshot(pointer, snapshot) != 5 { continue; }
          let mut row = TypoTableRow::new();
          row.assignment(pointer, snapshot);
          let row_start = crate::cn::wps::moffice::writer::cache::TypoRangeBase::TypoRangeBase::getStartCP__int_typosnapshot(pointer, snapshot);
          let row_end = crate::cn::wps::moffice::writer::cache::TypoRangeBase::TypoRangeBase::getEndCP__int_typosnapshot(pointer, snapshot);
          let row_left = crate::cn::wps::moffice::writer::cache::TypoObjBase::TypoObjBase::getLeft__int_typosnapshot(pointer, snapshot);
          let row_right = crate::cn::wps::moffice::writer::cache::TypoObjBase::TypoObjBase::getRight__int_typosnapshot(pointer, snapshot);
          eprintln!("table_draw row cp={}..{} box={},{},{},{} cells={}", row_start, row_end, row_left, row.getTop(), row_right, row.getBottom(), row.getCellsCount());
          for index in 0..row.getCellsCount() {
            let cell_ptr = row.getCell(index);
            if cell_ptr == 0 { continue; }
            let mut cell = crate::cn::wps::moffice::writer::cache::TypoTableCell::TypoTableCell::new();
            cell.assignment(cell_ptr, snapshot);
            let mut rect = crate::cn::wps::moffice::writer::cache::TypoRectCoreImpl::TypoRectCoreImpl::default();
            cell.rectForPageCore(&mut rect);
            let cell_start = crate::cn::wps::moffice::writer::cache::TypoRangeBase::TypoRangeBase::getStartCP__int_typosnapshot(cell_ptr, snapshot);
            let cell_end = crate::cn::wps::moffice::writer::cache::TypoRangeBase::TypoRangeBase::getEndCP__int_typosnapshot(cell_ptr, snapshot);
            eprintln!("table_draw cell row={} idx={} cp={}..{} box={},{},{},{} brc={},{},{},{}", row_start, index, cell_start, cell_end, rect.super_.left, rect.super_.top, rect.super_.right, rect.super_.bottom, cell.getBrcTop(), cell.getBrcLeft(), cell.getBrcBottom(), cell.getBrcRight());
          }
        }
      }
      drawing
    }

    fn recycle_page_content(&mut self, page: i32) {
      let main = if self.mMainStateAlias.is_null() { &mut self.mMainState as *mut MainState } else { self.mMainStateAlias };
      let doc = if self.mDocEnvAlias.is_null() { &mut self.mDocEnv } else { unsafe { &mut *self.mDocEnvAlias } };
      doc.cacheCleaner.bind(main);
      doc.cacheCleaner.recyclePageContent(page);
    }

    fn remove_drawings_for_parent(&mut self, parent: i32, doc_ptr: Option<*mut dyn crate::cn::wps::moffice::writer::core::Document::Document>) {
      let Some(doc) = doc_ptr else { return; };
      let main = if self.mMainStateAlias.is_null() { &mut self.mMainState as *mut MainState } else { self.mMainStateAlias };
      if let Some(layers) = unsafe { (*main).getDrawingLayers__document_boolean(&*doc, true) } {
        if layers.getPointer() != 0 {
          layers.removeDrawinghsForParent(parent);
        }
      }
    }

    // java private int getEmbedMinFirstLine(int pFirsLine, TypoSnapshot snapshot)
    fn embed_min_first_line(&mut self, textbox: i32) -> i32 {
      let main = if self.mMainStateAlias.is_null() { &mut self.mMainState as *mut MainState } else { self.mMainStateAlias };
      let snapshot = unsafe { &mut (*main).snapshot };
      let lines = TypoTextBox::getLines__int_typosnapshot(textbox, snapshot);
      let first = crate::cn::wps::moffice::writer::cache::TypoList::TypoList::first__int_typosnapshot(lines, snapshot);
      self.getEmbedMinFirstLine(first, snapshot)
    }

    // Keep the Java method boundary callable by the table overflow gate.
    // The recursive implementation below follows nested row/cell first lines.
    pub fn getEmbedMinFirstLine(&self, first_line: i32, snapshot: &crate::cn::wps::moffice::writer::cache::TypoSnapshot::TypoSnapshot) -> i32 {
      Self::embed_min_first_line_at(first_line, snapshot)
    }

    fn embed_min_first_line_at(pointer: i32, snapshot: &crate::cn::wps::moffice::writer::cache::TypoSnapshot::TypoSnapshot) -> i32 {
      if pointer == 0 { return 0; }
      let mut first_line_height = crate::cn::wps::moffice::writer::cache::TypoObjBase::TypoObjBase::height__int_typosnapshot(pointer, snapshot);
      if snapshot.get__int(pointer) == 5 {
        let mut max_embed = 0i32;
        let count = TypoTableRow::getCellsCount__int_typosnapshot(pointer, snapshot);
        for i in 0..count {
          let cell = TypoTableRow::getCell__int_int_typosnapshot(i, pointer, snapshot);
          if cell == 0 { continue; }
          let cell_lines = TypoPage::getLines__int_typosnapshot(cell, snapshot);
          let embed_first = crate::cn::wps::moffice::writer::cache::TypoList::TypoList::first__int_typosnapshot(cell_lines, snapshot);
          max_embed = max_embed.max(Self::embed_min_first_line_at(embed_first, snapshot));
        }
        if max_embed > 0 { first_line_height = max_embed; }
      }
      first_line_height
    }

    // java private boolean checkOverlapToFootNote(TypoDrawing drawing)
    fn checkOverlapToFootNote(&mut self, drawing: &TypoDrawing) -> bool {
      let main = if self.mMainStateAlias.is_null() { &mut self.mMainState as *mut MainState } else { self.mMainStateAlias };
      let notes = unsafe { (*main).layoutPage.getFootnotes() };
      if notes == 0 { return false; }
      let snapshot = unsafe { &(*main).snapshot };
      let size = snapshot.get__int(notes + 7);
      let top = crate::cn::wps::moffice::writer::cache::TypoObjBase::TypoObjBase::getTop__int_typosnapshot(notes, snapshot);
      size > 0 && top < drawing.getBottom()
    }

    fn bind_wrap_page(&mut self, textbox: i32) {
      if textbox == 0 {
        self.mTableState.page = None;
        return;
      }
      let main = if self.mMainStateAlias.is_null() { &mut self.mMainState as *mut MainState } else { self.mMainStateAlias };
      let mut page = TypoPage::default();
      page.snapshot = unsafe { &mut (*main).snapshot };
      page.snapshot_pointer = textbox;
      self.mTableState.page = Some(page);
    }

    // java void flowRows(TypoDrawing drawing, IStateEnvoy stateEnvoy) for a wrap textbox
    fn flow_wrap_rows(&mut self, drawing: &mut TypoDrawing, layoutState: &mut EnvLayoutState) {
      let textbox = drawing.getTextbox();
      // java this.mTableState.page = textbox;
      self.bind_wrap_page(textbox);
      self.mTableState.coreWidth = layoutState.pageEnv.getCoreWidth();
      self.mTableState.curLayoutHeight = 0;
      self.mTableState.rowHeightOverflow = false;
      self.mTableState.findUnlayoutCell = false;
      self.mTableState.findBreakCell = false;
      let jc = self.mTable.as_ref().map(|table| table.getJC()).unwrap_or_default();
      let main = if self.mMainStateAlias.is_null() { &mut self.mMainState as *mut MainState } else { self.mMainStateAlias };
      if textbox != 0 {
        TypoTextBox::setWidth__int_int_typosnapshot(self.calJCWidth(&jc), textbox, unsafe { &mut (*main).snapshot });
      }
      let row_count = self.mTable.as_ref().map(|table| table.size()).unwrap_or(0);
      let mut index = self.mRow.as_ref().map(|row| row.index).unwrap_or(0);
      let mut lines = if textbox == 0 {
        0
      } else {
        TypoTextBox::getLines__int_typosnapshot(textbox, unsafe { &(*main).snapshot })
      };
      let mut line_pos = 0i32;
      while index < row_count {
        let Some(row) = self.mTable.as_ref().and_then(|table| table.getRowByIndex(index)) else { break; };
        self.mTableState.row = Some(row.clone());
        self.mTableState.rowFinalProperty = row.getFinalProperty();
        self.mTableState.isTableFit = self.mTable.as_ref().map(|table| table.isTableFit()).unwrap_or(false);
        let result = self.flow_one_row_at(
          layoutState,
          &row,
          false,
          Some((textbox, lines, line_pos, self.mTableState.curLayoutHeight)),
        );
        self.mTableState.rowHeightOverflow = result.row_height_overflow;
        lines = result.lines;
        self.mTableState.lineIt.assignment(textbox, line_pos, unsafe { &mut (*main).snapshot });
        self.postFlowWrapTableRow(layoutState.getEnvoy());
        line_pos = self.mTableState.lineIt.pos;
        self.mTableState.firstLineOfPage = false;
        self.mTableState.hasBreakCellOfLayoutPage = false;
        index += 1;
        if self.mTableState.rowHeightOverflow || self.mTableState.findUnlayoutCell || self.mTableState.findBreakCell {
          break;
        }
      }
      let row_jc = -jc.jcLeft;
      if lines != 0 {
        let snapshot = unsafe { &mut (*main).snapshot };
        let size = crate::cn::wps::moffice::writer::cache::TypoList::TypoList::size__int_typosnapshot(lines, snapshot);
        for i in 0..size {
          let p_line = snapshot.get__int(lines + 3 + i);
          if snapshot.get__int(p_line) == 5 {
            let mut row = TypoTableRow::new();
            row.assignment(p_line, snapshot as *mut TypoSnapshot);
            row.setOffsetOfJc(row_jc);
          }
        }
      }
      if textbox != 0 {
        let mut text = TypoTextBox::new();
        {
          let snapshot = unsafe { &mut (*main).snapshot };
          TypoTextBox::setHeight__int_int_typosnapshot(self.mTableState.curLayoutHeight, textbox, snapshot);
          text.assignment(textbox, snapshot as *mut TypoSnapshot);
          text.offsetTo(0, 0);
        }
        // java phone read mode and fit-table expands margins so border stroke stays inside the text box
        let phone_fit = crate::cn::wps::moffice::writer::base::ViewMode::ViewMode::isPhoneReadMode(self.view_env().getViewMode())
          && self.view_env().isFitTableForView();
        if phone_fit {
          self.adjustTableMargin(&mut text);
        }
        let lines = {
          let snapshot = unsafe { &mut (*main).snapshot };
          let left = snapshot.get__int(textbox + 3);
          let top = snapshot.get__int(textbox + 4);
          let right = snapshot.get__int(textbox + 5);
          let bottom = snapshot.get__int(textbox + 6);
          drawing.set(left, top, right, bottom);
          crate::cn::wps::moffice::writer::cache::TypoTextBox::TypoTextBox::getLines__int_typosnapshot(textbox, snapshot)
        };
        if std::env::var_os("WORD_PARITY_TABLE_DRAW_TRACE").is_some() {
          let snapshot = unsafe { &mut (*main).snapshot };
          let first_row = crate::cn::wps::moffice::writer::cache::TypoList::TypoList::first__int_typosnapshot(lines, snapshot);
          if first_row != 0 {
            let cell = TypoTableRow::getCell__int_int_typosnapshot(0, first_row, snapshot);
            if cell != 0 {
              eprintln!("table_draw before_overlap first_cell_brc={},{},{},{}", snapshot.get__int(cell + 21), snapshot.get__int(cell + 20), snapshot.get__int(cell + 23), snapshot.get__int(cell + 22));
            }
          }
        }
        TableCellBrcTool::removeOverlappedBrcs(unsafe { &mut (*main).snapshot }, lines);
      }
      if self.mTableState.findBreakCell || self.mTableState.findUnlayoutCell || self.mTableState.rowHeightOverflow {
        layoutState.setFinishPage(true);
      }
      self.mTableState.flowedRows += 1;
    }

    // java protected void setIsFirstRowOfTable(IStateEnvoy stateEnvoy)
    pub fn setIsFirstRowOfTable(&mut self, stateEnvoy: &mut dyn IStateEnvoy) {
      // java this.mFirstRowOfTable = false;
      self.mFirstRowOfTable = false;
      // java if (this.mRow.getIndex() != 0)
      if self.mRow.as_ref().is_some_and(|row| row.getIndex() != 0) {
        // java this.mFirstRowOfTable = false;
        self.mFirstRowOfTable = false;
      }
      // java else if (!stateEnvoy.getCellStateList().isEmpty())
      else if !stateEnvoy.getCellStateList().isEmpty() {
        // java this.mFirstRowOfTable = false;
        self.mFirstRowOfTable = false;
      }
      // java else
      else {
        // java this.mFirstRowOfTable = true;
        self.mFirstRowOfTable = true;
      }
    }


    // java protected TypoPage getFloatCotainerPage(IStateEnvoy stateEnvoy)
    pub fn getFloatCotainerPage(&self, stateEnvoy: &dyn IStateEnvoy) -> TypoPage {
      // java return stateEnvoy.getCurPage();
      stateEnvoy.getCurPage().clone()
    }

    // java protected boolean isOverFlow(TypoDrawing drawing, IStateEnvoy stateEnvoy)
    pub fn isOverFlow(&self, drawing: &TypoDrawing, stateEnvoy: &dyn IStateEnvoy) -> bool {
      // java return drawing.getBottom() > stateEnvoy.getPageEnv().maxLayoutHeight;
      drawing.getBottom() > stateEnvoy.getPageEnv().maxLayoutHeight
    }

    // java protected int calJCWidth(TableJCData jcData)
    pub fn calJCWidth(&self, jcData: &TableJCData) -> i32 {
      // java return jcData.jcRight - jcData.jcLeft;
      jcData.jcRight.wrapping_sub(jcData.jcLeft)
    }

    // java  void flowRows(TypoDrawing drawing, IStateEnvoy stateEnvoy)
    pub fn flowRows(&mut self, drawing: &TypoDrawing, stateEnvoy: &mut dyn IStateEnvoy) {
      let _ = drawing;
      self.mTableState.coreWidth = stateEnvoy.getPageEnv().getCoreWidth();
      self.mTableState.flowedRows += 1;
    }

    // java private void processCardInfo(LayoutState layoutState, IStateEnvoy stateEnvoy)
    pub fn processCardInfo(&mut self, layoutState: &mut EnvLayoutState) {
      // java if (this.mViewEnv.isShowCardInfo())
      if self.view_env().isShowCardInfo() {
        let startCP: i32 = self.mTable.as_ref().unwrap().getStartCP();
        let endCP: i32 = self.mTable.as_ref().unwrap().getEndCP();
        if layoutState.isUiCardStartCP(startCP) {
          self.mTableState.mLineBefore = 320;
          self.mTableState.mIsCardBegin = true;
        }
        if layoutState.isUiCardEndCP(endCP) {
          self.mTableState.mLineAfter = 240;
          self.mTableState.mIsCardEnd = true;
        }
        self.mTableState.mIsInUiCard = layoutState.isInUiCard();
        layoutState.curLayoutHeight += self.mTableState.mLineBefore;
      }
    }

    // java  void adjustTableMargin(TypoTextBox textBox)
    pub fn adjustTableMargin(&mut self, text_box: &mut TypoTextBox) {
      // java int width = textBox.width();
      let width = text_box.width();
      let pointer = text_box.getPointer();
      let main = if self.mMainStateAlias.is_null() { &mut self.mMainState as *mut MainState } else { self.mMainStateAlias };
      let (max_left, max_right, margin_left, margin_right) = {
        let snapshot = unsafe { &mut (*main).snapshot };
        // java BRC indexes written by index_brc are read back here. getObj stays the deferred share map.
        let lines = TypoTextBox::getLines__int_typosnapshot(pointer, snapshot);
        let count = crate::cn::wps::moffice::writer::cache::TypoPage::TypoPage::getLineCount__int_typosnapshot(pointer, snapshot);
        let mut max_left = 0i32;
        let mut max_right = 0i32;
        for i in 0..count {
          let row = crate::cn::wps::moffice::writer::cache::TypoList::TypoList::quickGet__int_int_typosnapshot(i, lines, snapshot);
          if crate::cn::wps::moffice::writer::cache::TypoMemoryObj::TypoMemoryObj::getType__int_typosnapshot(row, snapshot) != 5 {
            continue;
          }
          if crate::cn::wps::moffice::writer::cache::TypoTableRow::TypoTableRow::getCellsCount__int_typosnapshot(row, snapshot) <= 0 {
            continue;
          }
          let offset = crate::cn::wps::moffice::writer::cache::TypoTableRow::TypoTableRow::getOffsetOfJc__int_typosnapshot(row, snapshot);
          let first = crate::cn::wps::moffice::writer::cache::TypoTableRow::TypoTableRow::getCell__int_int_typosnapshot(0, row, snapshot);
          if first != 0 {
            if let Some(brc) = snapshot.get_brc(crate::cn::wps::moffice::writer::cache::TypoTableCell::TypoTableCell::getBrcLeft__int_typosnapshot(first, snapshot)) {
              let half = Self::half_brc_stroke(&brc);
              let left = crate::cn::wps::moffice::writer::cache::TypoObjBase::TypoObjBase::getLeft__int_typosnapshot(row, snapshot) + offset;
              if left - half < 0 {
                max_left = max_left.max(half - left);
              }
            }
          }
          let last_index = crate::cn::wps::moffice::writer::cache::TypoTableRow::TypoTableRow::getCellsCount__int_typosnapshot(row, snapshot) - 1;
          let last = crate::cn::wps::moffice::writer::cache::TypoTableRow::TypoTableRow::getCell__int_int_typosnapshot(last_index, row, snapshot);
          if last != 0 {
            if let Some(brc) = snapshot.get_brc(crate::cn::wps::moffice::writer::cache::TypoTableCell::TypoTableCell::getBrcRight__int_typosnapshot(last, snapshot)) {
              let half = Self::half_brc_stroke(&brc);
              let right = crate::cn::wps::moffice::writer::cache::TypoObjBase::TypoObjBase::getRight__int_typosnapshot(last, snapshot) + offset;
              if right + half > width {
                max_right = max_right.max(right + half - width);
              }
            }
          }
        }
        (
          max_left,
          max_right,
          crate::cn::wps::moffice::writer::cache::TypoPage::TypoPage::getMarginLeft__int_typosnapshot(pointer, snapshot),
          crate::cn::wps::moffice::writer::cache::TypoPage::TypoPage::getMarginRight__int_typosnapshot(pointer, snapshot),
        )
      };
      if max_left > 0 || max_right > 0 {
        if max_left > 0 {
          text_box.setMarginLeft(margin_left + max_left);
        }
        if max_right > 0 {
          text_box.setMarginRight(margin_right + max_right);
        }
        text_box.setWidth(text_box.width() + max_left + max_right);
      }
    }

    // java TableCellBrcTool.getHalfBrcStrokeWidth
    fn half_brc_stroke(brc: &crate::cn::wps::moffice::writer::data::value::BRC::BRC) -> i32 {
      util::moffice::util::UnitUtil::UnitUtil::PT2TWIPS(
        crate::cn::wps::moffice::writer::layout::base::tools::BRCBorderTool::BRCBorderTool::getActualStrokeWidth(
          brc.getBorderType(),
          brc.getLineWidth(),
        ),
      ) / 2
    }

    // java protected int getWrapTableHeight(IStateEnvoy stateEnvoy)
    pub fn getWrapTableHeight(&self, stateEnvoy: &dyn IStateEnvoy) -> i32 {
      // java PageEnv pageEnv = stateEnvoy.getPageEnv();
      let pageEnv = stateEnvoy.getPageEnv();
      // java return pageEnv.maxLayoutHeight - pageEnv.minLayoutHeight;
      pageEnv.maxLayoutHeight.wrapping_sub(pageEnv.minLayoutHeight)
    }

    // java protected void postFlowWrapTableRow(IStateEnvoy stateEnvoy)
    pub fn postFlowWrapTableRow(&mut self, stateEnvoy: &mut dyn IStateEnvoy) {
      // java if (!this.mTableState.rowHeightOverflow && !this.mTableState.findUnlayoutCell)
      if !self.mTableState.rowHeightOverflow && !self.mTableState.findUnlayoutCell {
        if self.mTableState.findBreakCell {
          // java this.mTableState.curLayoutHeight = TypoObjBase.getBottom(this.mTableState.lineIt.value(), snapshot);
          let line = self.mTableState.lineIt.value();
          let main = if self.mMainStateAlias.is_null() { &mut self.mMainState as *mut MainState } else { self.mMainStateAlias };
          self.mTableState.curLayoutHeight = crate::cn::wps::moffice::writer::cache::TypoObjBase::TypoObjBase::getBottom__int_typosnapshot(line, unsafe { &(*main).snapshot });
          // java this.mTableState.lineIt.next();
          self.mTableState.lineIt.next();
        } else {
          // java stateEnvoy.setCP(this.mTableState.row.getEndCP());
          if let Some(row) = self.mTableState.row.as_ref() {
            stateEnvoy.setCP(row.getEndCP());
          }
          // java this.mLayouterTask.endParaLayouter(stateEnvoy);
          let envoy = stateEnvoy.as_para_envoy();
          self.end_para_envoy(envoy);
          let line = self.mTableState.lineIt.value();
          let main = if self.mMainStateAlias.is_null() { &mut self.mMainState as *mut MainState } else { self.mMainStateAlias };
          self.mTableState.curLayoutHeight = crate::cn::wps::moffice::writer::cache::TypoObjBase::TypoObjBase::getBottom__int_typosnapshot(line, unsafe { &(*main).snapshot });
          self.mTableState.lineIt.next();
        }
      } else if !self.mTableState.lineIt.isEnd() {
        // java recycleRow(lineIt.remove(), getDrawingLayers(document))
        let removed = self.mTableState.lineIt.remove();
        self.recycle_removed_row(removed, stateEnvoy.getDocument());
        // java if (this.mTableState.page.hasLines())
        if self.mTableState.page.as_ref().is_some_and(|page| page.hasLines()) {
          let main = if self.mMainStateAlias.is_null() { &mut self.mMainState as *mut MainState } else { self.mMainStateAlias };
          let lines = self.mTableState.page.as_ref().map(|page| page.getLines()).unwrap_or(0);
          let snapshot = unsafe { &mut (*main).snapshot };
          let last = crate::cn::wps::moffice::writer::cache::TypoList::TypoList::last__int_typosnapshot(lines, snapshot);
          // java stateEnvoy.setCP(TypoRangeBase.getEndCP(lastLine, snapshot));
          stateEnvoy.setCP(crate::cn::wps::moffice::writer::cache::TypoRangeBase::TypoRangeBase::getEndCP__int_typosnapshot(last, snapshot));
        }
      }
    }

    // java protected void postFlowNormalTableRow(IStateEnvoy stateEnvoy)
    pub fn postFlowNormalTableRow(&mut self, stateEnvoy: &mut dyn IStateEnvoy) {
      // java if (!this.mTableState.rowHeightOverflow && !this.mTableState.findUnlayoutCell)
      if !self.mTableState.rowHeightOverflow && !self.mTableState.findUnlayoutCell {
        if self.mTableState.findBreakCell {
          // java stateEnvoy.setCurLayoutHeight(TypoObjBase.getBottom(lineIt.value(), snapshot));
          let line = self.mTableState.lineIt.value();
          let main = if self.mMainStateAlias.is_null() { &mut self.mMainState as *mut MainState } else { self.mMainStateAlias };
          let bottom = crate::cn::wps::moffice::writer::cache::TypoObjBase::TypoObjBase::getBottom__int_typosnapshot(line, unsafe { &(*main).snapshot });
          stateEnvoy.setCurLayoutHeight(bottom);
          if self.mTableState.firstLineOfPage && self.mTableState.tableHeader && self.mTableState.firstTableHeader {
            // java stateEnvoy.setCP(this.mRow.getEndCP());
            if let Some(row) = self.mRow.as_ref() {
              stateEnvoy.setCP(row.getEndCP());
            }
            let envoy = stateEnvoy.as_para_envoy();
            self.end_para_envoy(envoy);
            self.mTableState.curLayoutHeight = bottom;
            let cells = stateEnvoy.drainCellStates();
            let doc = if self.mDocEnvAlias.is_null() { &mut self.mDocEnv } else { unsafe { &mut *self.mDocEnvAlias } };
            doc.pools.recycleTableCellLayoutStates(cells);
            self.mTableState.stopFlowTableHeader = true;
          }
          stateEnvoy.setValidLine(true);
          stateEnvoy.setFinishPage(true);
        } else if self.mTableState.tableHeader && !self.mTableState.firstTableHeader {
          let envoy = stateEnvoy.as_para_envoy();
          self.end_para_envoy(envoy);
          self.set_height_from_line(stateEnvoy);
          stateEnvoy.setValidLine(true);
        } else {
          if let Some(row) = self.mRow.as_ref() {
            stateEnvoy.setCP(row.getEndCP());
          }
          let envoy = stateEnvoy.as_para_envoy();
          self.end_para_envoy(envoy);
          self.set_height_from_line(stateEnvoy);
          stateEnvoy.setValidLine(true);
        }
      } else {
        if !self.mTableState.lineIt.isEnd() {
          let removed = self.mTableState.lineIt.remove();
          self.recycle_removed_row(removed, stateEnvoy.getDocument());
        }
        stateEnvoy.setFinishPage(true);
      }
    }

    fn set_height_from_line(&mut self, stateEnvoy: &mut dyn IStateEnvoy) {
      let line_pos = stateEnvoy.getLinePos();
      let lines = stateEnvoy.getCurPage().getLines();
      let main = if self.mMainStateAlias.is_null() { &mut self.mMainState as *mut MainState } else { self.mMainStateAlias };
      let p_line = crate::cn::wps::moffice::writer::cache::TypoList::TypoList::get__int_int_typosnapshot(line_pos, lines, unsafe { &mut (*main).snapshot });
      let bottom = crate::cn::wps::moffice::writer::cache::TypoObjBase::TypoObjBase::getBottom__int_typosnapshot(p_line, unsafe { &(*main).snapshot });
      stateEnvoy.setCurLayoutHeight(bottom);
    }

    fn recycle_removed_row(&mut self, removed: i32, document: &dyn crate::cn::wps::moffice::writer::core::Document::Document) {
      let main = if self.mMainStateAlias.is_null() { &mut self.mMainState as *mut MainState } else { self.mMainStateAlias };
      let doc_env = if self.mDocEnvAlias.is_null() { &mut self.mDocEnv as *mut DocumentEnv } else { self.mDocEnvAlias };
      let layers = unsafe {
        (*main).getDrawingLayers(document).map(|layers| layers as *mut crate::cn::wps::moffice::writer::cache::TypoDrawingLayers::TypoDrawingLayers)
      }.unwrap_or(std::ptr::null_mut());
      unsafe {
        (*doc_env).cacheCleaner.bind_env(main, doc_env);
        (*doc_env).cacheCleaner.recycle_row(removed, layers);
      }
    }

    // java protected void onAdjustDrawing(TypoDrawing drawing, IStateEnvoy stateEnvoy)
    // java protected void onAdjustDrawing — empty in TableLayouter.java
    pub fn onAdjustDrawing(&mut self, drawing: &TypoDrawing, stateEnvoy: &mut dyn IStateEnvoy) {
      let _ = (drawing, stateEnvoy);
    }

}

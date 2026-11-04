#![allow(non_snake_case)]

// Java: import cn.wps.moffice.writer.cache.TypoPage
use crate::cn::wps::moffice::writer::cache::TypoPage::TypoPage;
// Java: import cn.wps.moffice.writer.cache.TypoSnapshot
use crate::cn::wps::moffice::writer::cache::TypoSnapshot::TypoSnapshot;
// Java: import cn.wps.moffice.writer.cache.TypoTableCell
use crate::cn::wps::moffice::writer::cache::TypoTableCell::TypoTableCell;
// Java: import cn.wps.moffice.writer.core.Document
use crate::cn::wps::moffice::writer::core::Document::Document;
// Java: import cn.wps.moffice.writer.layout.base.ParaLayouter
use crate::cn::wps::moffice::writer::layout::base::ParaLayouter::ParaLayouter;
// Java: import cn.wps.moffice.writer.layout.base.TextLayouter
use crate::cn::wps::moffice::writer::layout::base::TextLayouter::TextLayouter;
// Java: import cn.wps.moffice.writer.layout.base.env.DocumentEnv
use crate::cn::wps::moffice::writer::layout::base::env::DocumentEnv::DocumentEnv;
// Java: import cn.wps.moffice.writer.layout.base.env.LayoutState
use crate::cn::wps::moffice::writer::layout::base::env::LayoutState::LayoutState;
// Java: import cn.wps.moffice.writer.layout.base.env.MainState
use crate::cn::wps::moffice::writer::layout::base::env::MainState::MainState;
// Java: import cn.wps.moffice.writer.layout.base.env.ViewEnv
use crate::cn::wps::moffice::writer::layout::base::env::ViewEnv::ViewEnv;
use crate::cn::wps::moffice::writer::layout::base::table::TableCellPageLayouter::TableCellPageLayouter;
use crate::cn::wps::moffice::writer::data::table::tree::CellInfoNode::CellInfoNode;
use drawing::cn::wps::moffice::drawing::graphics::SHD::SHD;

// java public class TableCellLayouter extends TextLayouter
// Rust has no subclass, so the cell layouter owns the TextLayouter Java would inherit.
// super.flow runs on that instance against the parent snapshot heap.
pub struct TableCellLayouter {
    // java protected boolean mEnableProcessPagination;
    pub mEnableProcessPagination: bool,
    // java extends TextLayouter
    mText: TextLayouter,
    // java mDocEnv is the same DocumentEnv instance passed by TableRowLayouter.
    mDocEnvAlias: Option<*mut DocumentEnv>,
}

impl TableCellLayouter {
    pub fn new(doc_env: Option<*mut DocumentEnv>) -> Self {
        // java super(mainState, docEnv, viewEnv); initLayouters installs the paragraph layouter
        let mut mText = TextLayouter::TextLayouter(MainState::default(), DocumentEnv::default(), ViewEnv::default());
        mText.mParaLayouter = Some(ParaLayouter::ParaLayouter(DocumentEnv::default(), ViewEnv::default(), MainState::default()));
        Self { mEnableProcessPagination: false, mText, mDocEnvAlias: doc_env }
    }

    // java public void flow(LayoutState layoutState) — the flag half, before super.flow
    pub fn mark_first_break(&self, cell: &mut TypoTableCell, range_start: i32, cp: i32, cell_states_empty: bool) {
        // java boolean firstBreakCell = layoutState.getRangeStart() == layoutState.cp && layoutState.cellStateList.isEmpty();
        let first_break = range_start == cp && cell_states_empty;
        // java cell.setFirstBreakCell(firstBreakCell);
        cell.setFirstBreakCell(first_break);
    }

    // java this.mEnableProcessPagination = 0 == layoutState.document.getType();
    pub fn set_pagination_from_doc(&mut self, doc_type: i32) {
        self.mEnableProcessPagination = doc_type == 0;
    }

    // java protected void postFlow(LayoutState layoutState)
    pub fn post_flow(&self, cell: &mut TypoTableCell, document: &dyn Document) {
        cell.post_flow_empty_range(document);
    }

    // java public void flow(LayoutState layoutState) { ... super.flow(layoutState); }
    // The cell is already allocated in `snapshot`. The text layouter keeps its own
    // TypoSnapshot value, so the parent heap is swapped in for the call and swapped back.
    // tablePeriLayer + 1 keeps this cell's paragraphs from re-entering the same table.
    pub fn flow_text(
        &mut self,
        snapshot: &mut TypoSnapshot,
        main_state: Option<*mut MainState>,
        page_state: Option<*mut LayoutState>,
        cell_pointer: i32,
        start: i32,
        end: i32,
        document: *mut dyn Document,
        table_peri_layer: i32,
        grid_model: i32,
        grid_height: i32,
        grid_width: i32,
        char_space_for_grid: f32,
        text_flow: i32,
        width: i32,
        margin_top: i32,
        margin_left: i32,
        margin_bottom: i32,
        margin_right: i32,
        max_layout_height: i32,
        allow_over_core_height: bool,
        allow_break_line: bool,
        cell_paint: Option<(Option<CellInfoNode>, Option<SHD>)>,
    ) -> i32 {
        if document.is_null() || cell_pointer == 0 || end <= start {
            return end;
        }
        // java TableCellLayouter(super(mainState, docEnv, viewEnv)) shares docEnv.
        // Rust temporarily moves its state into TextLayouter and restores it
        // after this cell flows, retaining cache identity across cells.
        let parent_env = self.mDocEnvAlias.filter(|p| !p.is_null());
        if let Some(parent_env) = parent_env {
            std::mem::swap(unsafe { &mut *parent_env }, &mut self.mText.mDocEnv);
        }
        // java TextLayouter.initLayouters passes the same mDocEnv to its line
        // and paragraph children. Rebuild their Rust-owned environment views
        // from that document before the cell's first rollback/line flow.
        self.mText.rebase_line_env();
        let text_doc = self.mText.mDocEnv.textDocumentAlias;
        if !text_doc.is_null() {
            if let Some(line) = self.mText.mLineLayouter.as_mut() {
                if let Some(env) = line.super_mut().super_mut().super_mut().mDocEnv.as_mut() {
                    env.updateFromDocument(unsafe { &*text_doc });
                }
            }
            if let Some(para) = self.mText.mParaLayouter.as_mut() {
                para.mDocEnv.updateFromDocument(unsafe { &*text_doc });
            }
        }
        // Java TableCellLayouter and its parent hold the same MainState. Keep
        // the drawing layer identities with the snapshot while this Rust
        // subclass adapter runs; nested wrap tables append their drawing to
        // that shared layer and PageRender later looks it up by parent cell.
        if let Some(parent) = main_state.filter(|p| !p.is_null()) {
            std::mem::swap(unsafe { &mut (*parent).drawingLayers }, &mut self.mText.mMainState.drawingLayers);
            std::mem::swap(unsafe { &mut (*parent).headerFooterDrawingLayers }, &mut self.mText.mMainState.headerFooterDrawingLayers);
        }
        std::mem::swap(snapshot, &mut self.mText.mMainState.snapshot);
        self.mText.mMainState.drawingLayers.rebind_snapshot(&mut self.mText.mMainState.snapshot);
        self.mText.mMainState.headerFooterDrawingLayers.rebind_snapshot(&mut self.mText.mMainState.snapshot);
        let snap = &mut self.mText.mMainState.snapshot as *mut TypoSnapshot;
        let mut state = LayoutState::LayoutState(None);
        state.document = Some(document);
        state.cp = start;
        state.setRange__int_int(start, end);
        state.tablePeriLayer = table_peri_layer + 1;
        // Java Tools.createTableCellLayoutEnv carries the outer PageEnv grid
        // into the cell state; the Rust TextLayouter adapter must retain it.
        state.pageEnv.gridModel = grid_model;
        state.pageEnv.gridHeight = grid_height;
        state.pageEnv.gridWidth = grid_width;
        state.pageEnv.charSpaceForGrid = char_space_for_grid;
        state.pageEnv.maxLayoutHeight = max_layout_height;
        // java flowCells writes these onto the cell page env before flowCell
        state.pageEnv.allowOverCoreHeight = allow_over_core_height;
        state.pageEnv.allowBreakLine = allow_break_line;
        state.pageEnv.width = width;
        state.pageEnv.textFlow = text_flow;
        state.pageEnv.marginTop = margin_top;
        state.pageEnv.marginLeft = margin_left;
        state.pageEnv.marginBottom = margin_bottom;
        state.pageEnv.marginRight = margin_right;
        // java TableCellPageLayouter.onStart is dispatched by super.flow's
        // preFlow. The Rust cell adapter has no PageLayouter subclass slot,
        // so invoke the same class at the corresponding cell start boundary.
        let mut cell_page = TableCellPageLayouter::TableCellPageLayouter(
            DocumentEnv::default(), ViewEnv::default());
        let mut page = TypoPage::default();
        page.snapshot_pointer = cell_pointer;
        page.snapshot = snap;
        state.curPage = Some(page);
        cell_page.onStart(&mut state, cell_paint.as_ref());
        self.mText.mEnableProcessPagination = self.mEnableProcessPagination;
        self.mText.flow(&mut state);
        let cp = state.cp;
        // Java's cell TextLayouter writes TypoRun graph widths and embed boxes
        // into the same shared page cache. The Rust adapter's LayoutState is
        // separate, so transfer only accepted cell-line measurements upward.
        if let Some(parent) = page_state.filter(|p| !p.is_null()) {
            let parent = unsafe { &mut *parent };
            if let Some(page) = state.curPage.as_ref() {
                for (index, advances) in state.committed_line_advances.iter().enumerate() {
                    if let Some(line) = page.line_at(index as i32) {
                        parent.table_line_advances.push((line.getStartCP(), line.getEndCP(), advances.clone()));
                    }
                }
            }
            for place in state.embed_places.iter().copied() {
                if !parent.embed_places.contains(&place) {
                    parent.embed_places.push(place);
                }
            }
            parent.table_line_advances.extend(state.table_line_advances.drain(..));
        }
        std::mem::swap(snapshot, &mut self.mText.mMainState.snapshot);
        if let Some(parent) = main_state.filter(|p| !p.is_null()) {
            std::mem::swap(unsafe { &mut (*parent).drawingLayers }, &mut self.mText.mMainState.drawingLayers);
            std::mem::swap(unsafe { &mut (*parent).headerFooterDrawingLayers }, &mut self.mText.mMainState.headerFooterDrawingLayers);
            unsafe {
                (*parent).drawingLayers.rebind_snapshot(&mut (*parent).snapshot);
                (*parent).headerFooterDrawingLayers.rebind_snapshot(&mut (*parent).snapshot);
            }
        }
        if let Some(parent_env) = parent_env {
            std::mem::swap(unsafe { &mut *parent_env }, &mut self.mText.mDocEnv);
        }
        cp
    }
}

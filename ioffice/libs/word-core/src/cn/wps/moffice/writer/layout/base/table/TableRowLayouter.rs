#![allow(non_snake_case)]

// Java: import cn.wps.moffice.writer.cache.TypoList
use crate::cn::wps::moffice::writer::cache::TypoList::TypoList;
// Java: import cn.wps.moffice.writer.cache.TypoObjBase
use crate::cn::wps::moffice::writer::cache::TypoObjBase::TypoObjBase;
// Java: import cn.wps.moffice.writer.cache.TypoPage
use crate::cn::wps::moffice::writer::cache::TypoPage::TypoPage;
// Java: import cn.wps.moffice.writer.cache.TypoSnapshot
use crate::cn::wps::moffice::writer::cache::TypoSnapshot::TypoSnapshot;
// Java: import cn.wps.moffice.writer.cache.TypoTableCell
use crate::cn::wps::moffice::writer::cache::TypoTableCell::TypoTableCell;
// Java: import cn.wps.moffice.writer.cache.TypoTableRow
use crate::cn::wps::moffice::writer::cache::TypoTableRow::TypoTableRow;
// Java: import cn.wps.moffice.writer.core.Document
use crate::cn::wps::moffice::writer::core::Document::Document;
// Java: import cn.wps.moffice.writer.layout.base.table.TableCellLayouter
use crate::cn::wps::moffice::writer::layout::base::table::TableCellLayouter::TableCellLayouter;

// Java: CellInfo fields read by newCell / prepareEnvs (hori merge 2 is skipped)
#[derive(Clone, Default)]
pub struct CellGeom {
    pub start: i32,
    pub end: i32,
    pub hori_merge: i32,
    pub vert_merged: bool,
    pub first_merged: bool,
    pub last_merged: bool,
    pub vert_align: i32,
    pub fit_text: bool,
    pub margin_top: i32,
    pub margin_left: i32,
    pub margin_right: i32,
    pub margin_bottom: i32,
    pub text_flow: i32,
    pub left: i32,
    pub right: i32,
}

// Java: TableState.row + the page numbers flow/doFlow/newTableRow read
#[derive(Clone, Default)]
pub struct RowFlowInput {
    // java TableRowLayouter.mDocEnv shared with TableCellLayouter
    pub doc_env: Option<*mut crate::cn::wps::moffice::writer::layout::base::env::DocumentEnv::DocumentEnv>,
    // java TableRowLayouter.mMainState supplies drawing layers to vAlignCell.
    pub main_state: Option<*mut crate::cn::wps::moffice::writer::layout::base::env::MainState::MainState>,
    // Parent page/cell state receives committed graph measurements from cell flow.
    pub page_state: Option<*mut crate::cn::wps::moffice::writer::layout::base::env::LayoutState::LayoutState>,
    pub index: i32,
    pub start_cp: i32,
    pub end_cp: i32,
    pub row_height: i32,
    pub cant_split: bool,
    pub table_header: bool,
    pub is_table_fit: bool,
    pub rtl: bool,
    pub rgdxa: Vec<i32>,
    pub cells: Vec<CellGeom>,
    pub first_line_of_page: bool,
    pub at_least_one_line: bool,
    pub allow_over_core_height: bool,
    pub allow_break_line: bool,
    pub cur_layout_height: i32,
    pub max_layout_height: i32,
    pub page_pointer: i32,
    pub line_pos: i32,
    pub lines: i32,
    pub view_mode: i32,
    pub card_begin: bool,
    pub card_end: bool,
    pub line_before: i32,
    pub line_after: i32,
    pub doc_type: i32,
    // java flowTableHeader forces tableHeader=true and firstTableHeader=false
    pub repeat_header: bool,
    // java layoutState.tablePeriLayer; cell flow uses this + 1
    pub table_peri_layer: i32,
    // Java Tools.createTableCellLayoutEnv copies the outer page's document grid.
    pub grid_model: i32,
    pub grid_height: i32,
    pub grid_width: i32,
    pub char_space_for_grid: f32,
    // java TableRowLayouter.mOffsetForJC from calcOffsetForJC
    pub jc_offset: i32,
    // java TableRowLayouter.mCellStateList
    pub cell_states: Option<*mut crate::cn::wps::moffice::writer::layout::base::env::LayoutState::CellStateList>,
    // java TableState.hasBreakCellOfLayoutPage
    pub has_break_of_page: bool,
    // java TableState.row index, used by prepareEnvs
    pub row_index: i32,
    // java doFlow revision-hidden row: the cells stay empty and the row height is 0
    pub revision_hidden: bool,
    pub revision_delete: bool,
    pub revision_insert: bool,
    pub format_change: bool,
    pub revisions: Option<*mut crate::cn::wps::moffice::writer::layout::base::env::MainState::RevisionsHolder>,
    pub typo_document: Option<*mut crate::cn::wps::moffice::writer::cache::TypoDocument::TypoDocument>,
}

#[derive(Clone, Default)]
pub struct RowFlowResult {
    pub row_pointer: i32,
    pub lines: i32,
    pub row_height_overflow: bool,
    pub table_header: bool,
    pub first_table_header: bool,
    pub is_first_row_of_table: bool,
    pub cur_layout_height: i32,
    // java TableState.findBreakCell after flowCells
    pub find_break_cell: bool,
}

// java first vertical-merge cell, flowed when the last cell of the chain is reached
struct OpenVertMerge {
    left: i32,
    pointer: i32,
    start: i32,
    end: i32,
    text_flow: i32,
    margin_top: i32,
    margin_left: i32,
    margin_bottom: i32,
    margin_right: i32,
    width: i32,
    // java firstMergerCellState.getCellInfo() survives until flowCell(lastMerged).
    paint: Option<(Option<crate::cn::wps::moffice::writer::data::table::tree::CellInfoNode::CellInfoNode>, Option<drawing::cn::wps::moffice::drawing::graphics::SHD::SHD>)>,
}

// java public class TableRowLayouter
#[derive(Default)]
pub struct TableRowLayouter {
    // java protected int mOffsetForJC;
    mOffsetForJC: i32,
    // java protected int mRowHeight;
    mRowHeight: i32,
    // java protected TypoTableRow mTableRow;
    mTableRow: i32,
    // java line list after TypoPageLinesIterator.insert (may reallocate)
    mLines: i32,
    // java mMergedcellStateList: first cells waiting for the last merge cell
    open_merges: Vec<OpenVertMerge>,
    // java KList<TableCellLayoutState> mMergedcellStateList
    m_merged: Vec<crate::cn::wps::moffice::writer::layout::base::env::TableCellLayoutState::TableCellLayoutState>,
    // java TableState.findBreakCell from the previous row, read by collectMergeCell
    m_find_break: bool,
    // java TableState.findBreakCell for the row currently being flowed
    m_row_break: bool,
    // java TableState.maxTopBrcWidth / maxBottomBrcWidth, filled by flowCells
    m_max_top_brc: i32,
    m_max_bottom_brc: i32,
}

impl TableRowLayouter {
    pub fn new() -> Self { Self::default() }

    // java public void dispose()
    pub fn dispose(&mut self) {
        self.mTableRow = 0;
        self.mLines = 0;
        self.mRowHeight = 0;
        self.mOffsetForJC = 0;
    }

    // java public void flow(IStateEnvoy stateEnvoy)
    pub fn flow(&mut self, input: &RowFlowInput, snapshot: &mut TypoSnapshot, document: Option<*mut dyn Document>) -> RowFlowResult {
        // java this.mTableState.findUnlayoutCell = false;
        // java this.mTableState.rowHeightOverflow = false;
        // java this.mTableState.cantSplit = false;
        // java this.mTableRow = null;
        self.mTableRow = 0;
        self.mLines = input.lines;
        if input.index == 0 && !input.repeat_header {
            self.open_merges.clear();
        }
        let mut overflow = false;
        // java this.mTableState.tableHeader = false;
        let mut table_header = false;
        // java this.mTableState.firstTableHeader = false;
        let mut first_table_header = false;
        // java this.mTableState.isFirstRowOfTable = false;
        let mut is_first_row = false;
        // java if (this.mTableState.row.getIndex() == 0)
        if input.repeat_header {
            // java flowTableHeader: tableHeader = true; firstTableHeader = false
            table_header = true;
            first_table_header = false;
        } else if input.index == 0 {
            // java this.mTableState.tableHeader = this.mTableState.rowFinalProperty.getBoolean(320, false);
            table_header = input.table_header;
            // java this.mTableState.firstTableHeader = this.mTableState.tableHeader;
            first_table_header = table_header;
            // java this.mTableState.isFirstRowOfTable = true;
            is_first_row = true;
        }
        // java this.doFlow(stateEnvoy);
        self.m_row_break = false;
        self.do_flow(input, snapshot, document, &mut overflow);
        self.m_find_break = self.m_row_break;
        // java if (this.mTableRow != null)
        let mut laid_bottom = input.cur_layout_height;
        if self.mTableRow != 0 {
            let mut row = TypoTableRow::default();
            row.assignment(self.mTableRow, snapshot as *mut TypoSnapshot);
            // java this.mTableRow.setTableHeader(this.mTableState.tableHeader);
            row.setTableHeader(table_header);
            // java this.mTableRow.setFirstHeader(this.mTableState.firstTableHeader);
            row.setFirstHeader(first_table_header);
            // java postFlowTableHeader: curLayoutHeight = mTableRow.getBottom()
            laid_bottom = row.getBottom();
        } else {
            // java this.mTableState.tableHeader = false;
            table_header = false;
            // java this.mTableState.firstTableHeader = false;
            first_table_header = false;
        }
        RowFlowResult {
            row_pointer: self.mTableRow,
            lines: self.mLines,
            row_height_overflow: overflow,
            table_header,
            first_table_header,
            is_first_row_of_table: is_first_row,
            cur_layout_height: laid_bottom,
            find_break_cell: self.m_row_break,
        }
    }

    // java mMergedcellStateList.removeFirst()
    pub fn pop_merged(&mut self) -> Option<crate::cn::wps::moffice::writer::layout::base::env::TableCellLayoutState::TableCellLayoutState> {
        if self.m_merged.is_empty() { None } else { Some(self.m_merged.remove(0)) }
    }

    // java protected void collectMergeCell()
    pub fn collect_merge_cells(
        &mut self,
        cells: &mut crate::cn::wps::moffice::writer::layout::base::env::LayoutState::CellStateList,
        pools: &mut crate::cn::wps::moffice::writer::layout::base::env::Pools::Pools,
    ) {
        let old = std::mem::take(&mut self.m_merged);
        pools.recycleTableCellLayoutStates(old);
        for mut cell in cells.drain() {
            // java keep a vertical-merge cell that is not the last, or a last cell while the row broke
            let keep = cell.vertMerged && (!cell.lastMerged || self.m_find_break);
            if !keep {
                if let Some(first) = cell.firstMergerCellState.take() {
                    if cell.lastMerged {
                        pools.recycleTableCellLayoutState(*first);
                    } else {
                        self.m_merged.push(*first);
                    }
                }
                pools.recycleTableCellLayoutState(cell);
            } else {
                self.m_merged.push(cell);
            }
        }
    }

    // java private void doFlow — prepareEnvs, prepareTableRow, newTableRow, adjustRowHeight for a known row height
    fn do_flow(&mut self, input: &RowFlowInput, snapshot: &mut TypoSnapshot, document: Option<*mut dyn Document>, overflow: &mut bool) {
        // java this.mRowHeight = this.getRowHeight();
        self.mRowHeight = input.row_height;
        // java this.mTableState.cantSplit = this.mTableState.row.cantSplit();
        let _cant_split = input.cant_split;
        // java this.calcOffsetForJC(this.mTableState.coreWidth, stateEnvoy);
        self.mOffsetForJC = input.jc_offset;
        // java if (!this.prepareTableRow(stateEnvoy)) this.rollbackTableRow(stateEnvoy);
        if !self.prepare_table_row(input, snapshot, document, overflow) {
            self.mTableRow = 0;
        }
    }

    // java protected boolean hasEnoughHeight(IStateEnvoy stateEnvoy)
    fn has_enough_height(&self, input: &RowFlowInput, overflow: &mut bool) -> bool {
        // java if (!this.mTableState.firstLineOfPage || !stateEnvoy.isAtLeastOnLine())
        if !input.first_line_of_page || !input.at_least_one_line {
            if self.mRowHeight > 0 {
                if !input.allow_over_core_height && input.max_layout_height - input.cur_layout_height < self.mRowHeight {
                    // java this.mTableState.rowHeightOverflow = true;
                    *overflow = true;
                    return false;
                }
            } else if self.mRowHeight < 0 && !input.allow_over_core_height && input.max_layout_height - input.cur_layout_height < -self.mRowHeight {
                *overflow = true;
                return false;
            }
        }
        true
    }

    // java protected boolean prepareTableRow(IStateEnvoy stateEnvoy)
    fn prepare_table_row(&mut self, input: &RowFlowInput, snapshot: &mut TypoSnapshot, document: Option<*mut dyn Document>, overflow: &mut bool) -> bool {
        if !self.has_enough_height(input, overflow) {
            // java if (!this.prevRowIsHeader()) return false; this.mTableState.rowHeightOverflow = false;
            if !Self::prev_row_is_header(input, snapshot) {
                return false;
            }
            *overflow = false;
        }
        // java this.newTableRow(stateEnvoy);
        self.new_table_row(input, snapshot, document);
        true
    }

    // java private boolean prevRowIsHeader()
    fn prev_row_is_header(input: &RowFlowInput, snapshot: &mut TypoSnapshot) -> bool {
        if input.lines == 0 {
            return false;
        }
        let size = TypoList::size__int_typosnapshot(input.lines, snapshot);
        if size <= 0 {
            return false;
        }
        // java lineIt.isEnd() → previous is the last line; otherwise the line before the insert position
        let prev = if input.line_pos >= size {
            TypoList::last__int_typosnapshot(input.lines, snapshot)
        } else if input.line_pos > 0 {
            TypoList::get__int_int_typosnapshot(input.line_pos - 1, input.lines, snapshot)
        } else {
            0
        };
        if prev == 0 || snapshot.get__int(prev) != 5 {
            return false;
        }
        TypoTableRow::isTableHeader__int_typosnapshot(prev, snapshot)
            && !TypoTableRow::isFirstHeader__int_typosnapshot(prev, snapshot)
    }

    // java newTableRow / flowCells read the cell envs prepared for this row.
    fn laid_cells(&self, input: &RowFlowInput, geom: &[&CellGeom]) -> Vec<(CellGeom, Option<usize>)> {
        let Some(list) = input.cell_states else {
            return geom.iter().map(|cell| ((*cell).clone(), None)).collect();
        };
        if list.is_null() {
            return geom.iter().map(|cell| ((*cell).clone(), None)).collect();
        }
        let list = unsafe { &mut *list };
        if list.isEmpty() {
            return geom.iter().map(|cell| ((*cell).clone(), None)).collect();
        }
        let mut laid = Vec::new();
        let mut i = 0usize;
        while i < list.len() {
            let env = unsafe { &*list.ptr_at(i) };
            let mut cell = if let Some(source) = geom.get(i) {
                (*source).clone()
            } else {
                CellGeom::default()
            };
            cell.start = env.super_.getRangeStart().max(env.super_.cp);
            cell.end = env.super_.getRangeEnd();
            cell.vert_merged = env.vertMerged;
            cell.first_merged = env.firstMerged;
            cell.last_merged = env.lastMerged;
            cell.text_flow = env.super_.pageEnv.textFlow;
            cell.margin_top = env.super_.pageEnv.marginTop;
            cell.margin_left = env.super_.pageEnv.marginLeft;
            cell.margin_bottom = env.super_.pageEnv.marginBottom;
            cell.margin_right = env.super_.pageEnv.marginRight;
            if cell.left == 0 && cell.right == 0 {
                cell.left = env.getRgdxaCenterLeft();
                cell.right = env.getRgdxaCenterRight();
            }
            laid.push((cell, Some(i)));
            i += 1;
        }
        laid
    }

    // java protected void newTableRow(IStateEnvoy stateEnvoy)
    fn new_table_row(&mut self, input: &RowFlowInput, snapshot: &mut TypoSnapshot, document: Option<*mut dyn Document>) {
        let geom: Vec<&CellGeom> = input.cells.iter().filter(|cell| cell.hori_merge != 2).collect();
        // java newTableRow walks mCellStateList. Geometry still comes from the same row's non-continuation cells.
        let laid = self.laid_cells(input, &geom);
        // java flowCells resets maxTopBrcWidth and maxBottomBrcWidth before the cell walk
        self.m_max_top_brc = 0;
        self.m_max_bottom_brc = 0;
        let is_first_row = input.first_line_of_page || input.index == 0;
        let mut row = TypoTableRow::default();
        // java this.mTableRow.alloc(this.mCellStateList.size(), snapshot);
        row.alloc(laid.len() as i32, snapshot);
        // java this.mTableRow.setOffsetOfJc(this.mOffsetForJC);
        row.setOffsetOfJc(self.mOffsetForJC);
        // java this.mTableRow.setRange(document, row.getStartCP(), row.getEndCP());
        if let Some(document) = document {
            row.setRange(unsafe { &*document }, input.start_cp, input.end_cp);
        }
        // java this.mTableState.lineIt.insert(this.mTableRow.getPointer());
        let lines = TypoList::insert__int_int_int_typosnapshot(row.getPointer(), input.line_pos, input.lines, snapshot);
        // java this.mTableRow.setParent(this.mTableState.page.getPointer());
        row.setParent(input.page_pointer);
        // java this.mTableRow.offset(0, this.mTableState.curLayoutHeight);
        row.offset(0, input.cur_layout_height);
        let rgdxa_count = input.rgdxa.len() as i32;
        // java this.mTableRow.setLeft(this.mTableState.row.getRgdxa(0));
        if rgdxa_count > 0 {
            row.setLeft(input.rgdxa[0]);
            // java this.mTableRow.setRight(this.mTableState.row.getRgdxa(count - 1));
            row.setRight(input.rgdxa[(rgdxa_count - 1) as usize]);
        }
        // java this.mTableRow.setRTL(this.mTableState.row.getParent().isRTL());
        row.setRTL(input.rtl);
        // java this.mTableRow.setCantSplit(this.mTableState.row.cantSplit());
        row.setCantSplit(input.cant_split);
        // java this.mTableRow.setFixedRowHeight(this.mTableState.row.getRowHeight() < 0);
        row.setFixedRowHeight(input.row_height < 0);
        // java this.mTableRow.setTableFit(this.mTableState.isTableFit);
        row.setTableFit(input.is_table_fit);
        row.setCardEnd(input.card_end);
        row.setCardBegin(input.card_begin);
        row.setLineBefore(input.line_before);
        row.setLineAfter(input.line_after);
        let mut index = 0i32;
        let mut cell_ptrs: Vec<(i32, bool, bool, bool)> = Vec::new();
        // Java: createTableCellLayouter() caches one TableCellLayouter for all
        // cells flowed by this TableRowLayouter.
        let mut cell_layouter = TableCellLayouter::new(input.doc_env);
        cell_layouter.set_pagination_from_doc(input.doc_type);
        for (cell, state_index) in &laid {
            if std::env::var_os("WORD_TABLE_FLOW_TRACE").is_some() && (100..=169).contains(&cell.start) && cell.left == -108 {
                eprintln!("table_cell_flow row={} cp={}..{} modelVert={} first={} last={} stateIndex={:?}",
                    input.index, cell.start, cell.end, cell.vert_merged, cell.first_merged, cell.last_merged, state_index);
            }
            // java TypoTableCell newCell = this.newCell(...)
            let mut typo_cell = TypoTableCell::default();
            typo_cell.alloc(snapshot);
            typo_cell.setParent(row.getPointer());
            typo_cell.setVertMergeCell(cell.vert_merged);
            typo_cell.setVertAlign(cell.vert_align);
            typo_cell.setFirstVertMergeCell(cell.first_merged);
            typo_cell.setLastVertMergeCell(cell.last_merged);
            typo_cell.setFitText(cell.fit_text);
            typo_cell.setDocumentType(input.doc_type);
            // java TableCellPageLayouter.onStart: setLeft / setRight
            typo_cell.setLeft(cell.left);
            typo_cell.setRight(cell.right);
            // java TableCellPageLayouter.calcMargins
            typo_cell.apply_page_margins(cell.text_flow, cell.margin_top, cell.margin_left, cell.margin_bottom, cell.margin_right);
            // java first break when the cell starts at its range and no cell state is carried in
            cell_layouter.mark_first_break(&mut typo_cell, cell.start, cell.start, true);
            if let Some(document) = document {
                // java newCell.setCellEndCP(cellEnv.getRangeEnd() - 1, document);
                typo_cell.setCellEndCP__int_document(cell.end - 1, unsafe { &*document });
            }
            if cell.first_merged {
                let paint = state_index.and_then(|index| input.cell_states.filter(|list| !list.is_null()).map(|list| {
                    let env = unsafe { &*(*list).ptr_at(index) };
                    (env.getCellInfo().cloned(), env.getMergedCellSHD())
                }));
                self.open_merges.push(OpenVertMerge {
                    left: cell.left,
                    pointer: typo_cell.getPointer(),
                    start: cell.start,
                    end: cell.end,
                    text_flow: cell.text_flow,
                    margin_top: cell.margin_top,
                    margin_left: cell.margin_left,
                    margin_bottom: cell.margin_bottom,
                    margin_right: cell.margin_right,
                    width: (cell.right - cell.left).max(0),
                    paint,
                });
            }
            // java flowCells records the widest top border on the first row and the widest bottom border
            if let Some(index) = *state_index {
                if let Some(list) = input.cell_states {
                    if !list.is_null() {
                        let env = unsafe { &*(*list).ptr_at(index) };
                        if !env.vertMerged || env.firstMerged {
                            if is_first_row {
                                self.m_max_top_brc = self.m_max_top_brc.max(Self::getActualLineWidth(env.getBrcTop()));
                            }
                            self.m_max_bottom_brc = self.m_max_bottom_brc.max(Self::getActualLineWidth(env.getBrcBottom()));
                        }
                    }
                }
            }
            // java flowCells: a vertical-merge cell is not flowed on its own row.
            // The last cell flows the first cell's range. firstMerged continues before that.
            let vertical = cell.text_flow != 0 && cell.text_flow != 4;
            if !input.revision_hidden && cell.vert_merged && !cell.first_merged {
                if let Some(document) = document {
                    let paint = if let Some(state_index) = *state_index {
                        input.cell_states.filter(|list| !list.is_null()).map(|list| {
                            let env = unsafe { &*(*list).ptr_at(state_index) };
                            (env.getCellInfo().cloned(), env.getMergedCellSHD())
                        })
                    } else {
                        None
                    };
                    // java initVertMergeCell before the last cell flows the first merge cell
                    self.init_vert_merge_cell(&mut typo_cell, cell, unsafe { &*document }, paint);
                }
            }
            if !input.revision_hidden && cell.vert_merged && !cell.first_merged && cell.last_merged && !vertical {
                if let Some(pos) = self.open_merges.iter().rposition(|open| open.left == cell.left) {
                    let open = self.open_merges.remove(pos);
                    if let Some(document) = document {
                        let (remain, mut allow_over, allow_break) = Self::cell_page_limits(input, true, open.margin_top, open.margin_bottom);
                        // java exctRowAndEnoughHeight forces allowOverCoreHeight on the first merge cell
                        if input.row_height < 0 && input.max_layout_height - input.cur_layout_height >= -input.row_height {
                            allow_over = true;
                        }
                        let mut first = TypoTableCell::default();
                        first.assignment(open.pointer, snapshot as *mut TypoSnapshot);
                        // java flowCell(firstMergeCellState) enters
                        // TableCellPageLayouter.onStart -> calcMargins here.
                        first.apply_page_margins(open.text_flow, open.margin_top, open.margin_left, open.margin_bottom, open.margin_right);
                        let cp = cell_layouter.flow_text(
                            snapshot,
                            input.main_state,
                            input.page_state,
                            open.pointer,
                            open.start,
                            open.end,
                            document,
                            input.table_peri_layer,
                            input.grid_model,
                            input.grid_height,
                            input.grid_width,
                            input.char_space_for_grid,
                            open.text_flow,
                            open.width,
                            open.margin_top,
                            open.margin_left,
                            open.margin_bottom,
                            open.margin_right,
                            remain,
                            allow_over,
                            allow_break,
                            open.paint.clone(),
                        );
                        if cp >= open.end {
                            first.setBreakCell(false);
                        } else {
                            // java onEnd: !fixRowHeight || firstMerged. This object is the first merge cell.
                            first.setBreakCell(true);
                            self.m_row_break = true;
                        }
                        let lines = TypoPage::getLines__int_typosnapshot(open.pointer, snapshot);
                        if lines != 0 {
                            let last = TypoList::last__int_typosnapshot(lines, snapshot);
                            if last != 0 {
                                let core = TypoObjBase::getBottom__int_typosnapshot(last, snapshot);
                                first.setHeight(first.getMarginTop().wrapping_add(core).wrapping_add(first.getMarginBottom()));
                            }
                        }
                    }
                }
            }
            if !input.revision_hidden && !cell.vert_merged && !vertical {
                if let Some(document) = document {
                    let width = (cell.right - cell.left).max(0);
                    let (remain, allow_over, allow_break) = Self::cell_page_limits(input, cell.vert_merged, cell.margin_top, cell.margin_bottom);
                    let cell_paint = state_index.and_then(|index| input.cell_states.filter(|list| !list.is_null()).map(|list| {
                        let env = unsafe { &*(*list).ptr_at(index) };
                        (env.getCellInfo().cloned(), env.getMergedCellSHD())
                    }));
                    let cp = cell_layouter.flow_text(
                        snapshot,
                        input.main_state,
                        input.page_state,
                        typo_cell.getPointer(),
                        cell.start,
                        cell.end,
                        document,
                        input.table_peri_layer,
                        input.grid_model,
                        input.grid_height,
                        input.grid_width,
                        input.char_space_for_grid,
                        cell.text_flow,
                        width,
                        cell.margin_top,
                        cell.margin_left,
                        cell.margin_bottom,
                        cell.margin_right,
                        remain,
                        allow_over,
                        allow_break,
                        cell_paint,
                    );
                    // java TableCellPageLayouter.onEnd. findBreakCell stays false here:
                    // postFlowNormalTableRow would finish the page without moving CP.
                    if cp >= cell.end {
                        typo_cell.setBreakCell(false);
                    } else if input.row_height >= 0 || cell.first_merged {
                        typo_cell.setBreakCell(true);
                        // java flowCells sets TableState.findBreakCell when the cell does not finish
                        self.m_row_break = true;
                    }
                }
            }
            if let Some(document) = document {
                // java TableCellLayouter.postFlow: empty range becomes [cellEnd, cellEnd+1]
                cell_layouter.post_flow(&mut typo_cell, unsafe { &*document });
            }
            // java adjustMinHeight: marginTop + lastLine.bottom + marginBottom
            let lines = TypoPage::getLines__int_typosnapshot(typo_cell.getPointer(), snapshot);
            if lines != 0 {
                let last = TypoList::last__int_typosnapshot(lines, snapshot);
                if last != 0 {
                    let core = TypoObjBase::getBottom__int_typosnapshot(last, snapshot);
                    typo_cell.setHeight(typo_cell.getMarginTop().wrapping_add(core).wrapping_add(typo_cell.getMarginBottom()));
                }
            }
            if let Some(index) = *state_index {
                if let Some(list) = input.cell_states {
                    if !list.is_null() {
                        let env = unsafe { &mut *(*list).ptr_at(index) };
                        let mut page = TypoPage::default();
                        page.snapshot = snapshot as *mut TypoSnapshot;
                        page.snapshot_pointer = typo_cell.getPointer();
                        env.super_.curPage = Some(page);
                        env.breakCell = typo_cell.isBreakCell();
                    }
                }
            }
            cell_ptrs.push((typo_cell.getPointer(), cell.first_merged, cell.vert_merged, cell.last_merged));
            row.setCell(index, typo_cell.getPointer());
            index += 1;
        }
        if input.format_change {
            row.setFormatChange(true);
        }
        if !input.revision_hidden && input.format_change {
            // java else-branch: revisions.addRevisionChangeTable before flowCells returns
            self.record_format_change(snapshot, document, input);
        }
        if input.revision_hidden {
            // java flowRevisionHiddenTable: empty cells, row height 0, record the delete revision
            self.finish_revision_hidden(&mut row, &laid, &cell_ptrs, snapshot, document, input);
            self.mTableRow = row.getPointer();
            self.mLines = lines;
            if input.page_pointer != 0 {
                snapshot.set__int_int(input.page_pointer + 17, lines);
            }
            return;
        }
        if input.revision_insert {
            row.setRevisionInsert(true);
        }
        self.adjust_row_height(&mut row, input);
        // java adjustMarginsForBRC runs after every cell's border width has been measured
        for (pointer, first_merged, vert_merged, _) in &cell_ptrs {
            if self.mRowHeight < 0 || *first_merged || !vert_merged {
                let mut cell = TypoTableCell::default();
                cell.assignment(*pointer, snapshot as *mut TypoSnapshot);
                self.adjust_margins_for_brc(is_first_row, &mut cell, snapshot);
            }
        }
        // java adjustRowHeight: last merge or a normal cell raises the row bottom to the first merge cell
        for (pointer, _, vert_merged, last_merged) in &cell_ptrs {
            if *last_merged || !vert_merged {
                let mut cell = TypoTableCell::default();
                cell.assignment(*pointer, snapshot as *mut TypoSnapshot);
                let first_pointer = cell.getFirstVertMergeCell();
                if first_pointer != 0 {
                    let mut first = TypoTableCell::default();
                    first.assignment(first_pointer, snapshot as *mut TypoSnapshot);
                    // java this.adjustMinHeight(firstVertMergeCell, stateEnvoy);
                    self.adjustMinHeight(&mut first, snapshot);
                    let mut rect = crate::cn::wps::moffice::writer::cache::TypoRectCoreImpl::TypoRectCoreImpl::default();
                    first.rectForPageCore(&mut rect);
                    if rect.getBottom() > row.getBottom() {
                        row.setHeight(rect.getBottom().wrapping_sub(row.getTop()));
                    }
                }
            }
        }
        if self.mRowHeight >= 0 && !input.allow_over_core_height && !input.allow_break_line && row.getBottom() > input.max_layout_height {
            row.setHeight(input.max_layout_height - row.getTop());
        }
        // java adjustRowHeight raises the row bottom to the flowed cell bottom
        let mut content_height = 0i32;
        for (pointer, _, _, _) in &cell_ptrs {
            let top = TypoObjBase::getTop__int_typosnapshot(*pointer, snapshot);
            let bottom = TypoObjBase::getBottom__int_typosnapshot(*pointer, snapshot);
            content_height = content_height.max(bottom.wrapping_sub(top));
        }
        let row_height = row.getBottom().wrapping_sub(row.getTop());
        if content_height > row_height {
            row.setHeight(content_height);
        }
        if let Some(document) = document {
            // java doFlow calls flowVerticalTextFlowCells after adjustRowHeight when the row did not break
            if !self.m_row_break && !input.repeat_header {
                self.flow_vertical_cells(&mut row, &laid, &cell_ptrs, &mut cell_layouter, snapshot, document, input);
            } else {
                // java flowRestCells when the row broke or this is a repeated header
                self.flow_rest_cells(&mut row, &laid, &cell_ptrs, &mut cell_layouter, snapshot, document, input);
            }
        }
        // java this.adjustCells(this.mTableRow); runs after flowRestCells/
        // flowVerticalTextFlowCells, so every ordinary cell owns the row height.
        self.adjustCells(&row, &cell_ptrs, snapshot, document, input);
        if std::env::var_os("WORD_PARITY_TABLE_FLOW_TRACE").is_some()
            && (6334..=6653).contains(&input.start_cp) {
            eprintln!("table_row cp={}..{} layer={} top={} bottom={} height={} property_height={} content_height={} brc_top={} brc_bottom={} max={} break={}",
                input.start_cp, input.end_cp, input.table_peri_layer,
                row.getTop(), row.getBottom(), row.getBottom().wrapping_sub(row.getTop()),
                input.row_height, content_height, self.m_max_top_brc, self.m_max_bottom_brc,
                input.max_layout_height, self.m_row_break);
        }
        self.mTableRow = row.getPointer();
        self.mLines = lines;
        // java lineIt.insert writes the page line list; a grown TypoList gets a new pointer
        if input.page_pointer != 0 {
            snapshot.set__int_int(input.page_pointer + 17, lines);
        }
    }

    // java protected void adjustMinHeight(TypoTableCell cell, IStateEnvoy stateEnvoy)
    fn adjustMinHeight(&self, cell: &mut TypoTableCell, snapshot: &mut TypoSnapshot) {
        // java int coreHeight = 0;
        let mut core_height = 0;
        // java if (cell.hasLines()) { int lastLine = TypoList.last(cell.getLines(), snapshot); ... }
        if cell.hasLines() {
            let lines = TypoPage::getLines__int_typosnapshot(cell.getPointer(), snapshot);
            let last = TypoList::last__int_typosnapshot(lines, snapshot);
            if last != 0 {
                core_height = TypoObjBase::getBottom__int_typosnapshot(last, snapshot);
            }
        }
        // java cell.setHeight(cell.getMarginTop() + coreHeight + cell.getMarginBottom());
        cell.setHeight(cell.getMarginTop().wrapping_add(core_height).wrapping_add(cell.getMarginBottom()));
    }

    // java protected void adjustCells(TypoTableRow row)
    fn adjustCells(&self, row: &TypoTableRow, cell_ptrs: &[(i32, bool, bool, bool)], snapshot: &mut TypoSnapshot, document: Option<*mut dyn Document>, input: &RowFlowInput) {
        let row_height = row.getBottom().wrapping_sub(row.getTop());
        for (pointer, first_merged, _vert_merged, last_merged) in cell_ptrs {
            let mut cell = TypoTableCell::default();
            cell.assignment(*pointer, snapshot as *mut TypoSnapshot);
            // java curCell.setHeight(row.height());
            cell.setHeight(row_height);
            if *last_merged || cell.isBreakCell() {
                // java int pFirstMergeCell = this.getFirstVerMergerCell(curCell);
                let first_pointer = cell.getFirstVertMergeCell();
                if first_pointer != 0 {
                    let mut first = TypoTableCell::default();
                    first.assignment(first_pointer, snapshot as *mut TypoSnapshot);
                    let mut rect = crate::cn::wps::moffice::writer::cache::TypoRectCoreImpl::TypoRectCoreImpl::default();
                    first.rectForPageCore(&mut rect);
                    // java firstMergeCell.setBottom(row.getBottom() - rectForPage.getTop());
                    let bottom = row.getBottom().wrapping_sub(rect.getTop());
                    let top = TypoObjBase::getTop__int_typosnapshot(first_pointer, snapshot);
                    first.setHeight(bottom.wrapping_sub(top));
                    // java this.vAlignCell(firstMergeCell);
                    self.vAlignCell(&first, snapshot, document, input);
                }
            }
            if !*first_merged {
                // java curCell.setHeight(row.height());
                cell.setHeight(row_height);
                // java this.vAlignCell(curCell);
                self.vAlignCell(&cell, snapshot, document, input);
            }
        }
    }

    // java protected void vAlignCell(TypoTableCell cell)
    // The no-float branch is the one used by Sample 3. Float-aware alignment
    // remains behind the drawing-layer dependency and is not inferred here.
    fn vAlignCell(&self, cell: &TypoTableCell, snapshot: &mut TypoSnapshot, document: Option<*mut dyn Document>, input: &RowFlowInput) {
        if cell.getVertAlign() == 0 || !cell.hasLines() { return; }
        let (Some(main), Some(doc)) = (input.main_state.filter(|p| !p.is_null()), document.filter(|p| !p.is_null())) else { return; };
        let mut floats = Vec::new();
        let has_layers = unsafe { (*main).getDrawingLayers(&*doc) }.map(|layers| {
            layers.getFloats__list_int(&mut floats, cell.getPointer());
        }).is_some();
        if !has_layers || !floats.is_empty() { return; }
        let lines = cell.getLines();
        let first = TypoList::first__int_typosnapshot(lines, snapshot);
        let last = TypoList::last__int_typosnapshot(lines, snapshot);
        if first == 0 || last == 0 { return; }
        // java spaceBeforeContent = firstLine.top; maxBottom = lastLine.bottom;
        let space_before = TypoObjBase::getTop__int_typosnapshot(first, snapshot);
        let max_bottom = TypoObjBase::getBottom__int_typosnapshot(last, snapshot);
        let core_height = TypoPage::getCoreHeight__int_typosnapshot(cell.getPointer(), snapshot);
        let space_after = core_height - max_bottom;
        let text_flow = TypoPage::getTextFlow__int_typosnapshot(cell.getPointer(), snapshot);
        let before_2013 = input.doc_env.filter(|p| !p.is_null()).and_then(|p| unsafe { (*p).textDocument.as_mut() })
            .map(|doc| doc.getCompatibilityMode().isBefore2013Mode()).unwrap_or(true);
        if space_after <= 0 && !(text_flow != 0 && before_2013) { return; }
        let y_offset = match cell.getVertAlign() {
            // java case 1: center, then clamp the final line to the core.
            1 => {
                let mut offset = (space_after - space_before) / 2;
                if text_flow == 0 || !before_2013 {
                    if max_bottom + offset > core_height { offset -= max_bottom + offset - core_height; }
                }
                offset
            }
            // java case 2: bottom.
            2 => space_after,
            _ => 0,
        };
        if std::env::var_os("WORD_PARITY_TABLE_VALIGN_TRACE").is_some() {
            eprintln!("table_valign cp={} align={} flow={} core={} before={} margins={},{} last={} after={} offset={} floats={}",
                crate::cn::wps::moffice::writer::cache::TypoRangeBase::TypoRangeBase::getStartCP__int_typosnapshot(cell.getPointer(), snapshot),
                cell.getVertAlign(), text_flow, core_height,
                space_before, cell.getMarginTop(), cell.getMarginBottom(), max_bottom, space_after, y_offset, floats.len());
        }
        if y_offset == 0 { return; }
        // java this.offsetTypoObjects(cell.getLines(), 0, yOffset);
        for index in 0..TypoList::size__int_typosnapshot(lines, snapshot) {
            let line = TypoList::get__int_int_typosnapshot(index, lines, snapshot);
            if line != 0 { TypoObjBase::offset__int_int_int_typosnapshot(0, y_offset, line, snapshot); }
        }
    }

    // java revisions.addRevisionChangeTable(document, row.startCP, tablePeriLayer, authorId, page)
    fn record_format_change(&mut self, snapshot: &mut TypoSnapshot, document: Option<*mut dyn Document>, input: &RowFlowInput) {
        let Some(document) = document.filter(|pointer| !pointer.is_null()) else { return; };
        let doc = unsafe { &*document };
        let Some(papx) = doc.getPapxTable().filter(|pointer| !pointer.is_null()) else { return; };
        let Some(para) = crate::cn::wps::moffice::writer::data::IPapxTable::IPapxTable::seek(unsafe { &*papx }, input.end_cp - 1) else { return; };
        let Some(prop) = crate::cn::wps::moffice::writer::data::IPapxTable::Entry::getProperty(para.as_ref()) else { return; };
        let Some(revision) = prop.getValue(363).and_then(|value| value.as_any().downcast_ref::<crate::cn::wps::moffice::writer::data::value::Revision::Revision>().cloned()) else { return; };
        let author_id = input.typo_document.filter(|pointer| !pointer.is_null()).map(|pointer| {
            unsafe { (*pointer).getAuthorId(revision.getAuthor().as_deref().unwrap_or("")) }
        }).unwrap_or(0);
        let Some(revisions) = input.revisions.filter(|pointer| !pointer.is_null()) else { return; };
        let mut author_of = |name: &str| {
            input.typo_document.filter(|pointer| !pointer.is_null()).map(|pointer| unsafe { (*pointer).getAuthorId(name) }).unwrap_or(0)
        };
        unsafe {
            (*revisions).addRevisionChangeTable(doc, input.start_cp, input.table_peri_layer, author_id, input.page_pointer, snapshot, &mut author_of);
        }
    }

    // java void flowRevisionHiddenTable(RowInfo row, IStateEnvoy stateEnvoy, Revision revision)
    fn finish_revision_hidden(
        &mut self,
        row: &mut TypoTableRow,
        laid: &[(CellGeom, Option<usize>)],
        cell_ptrs: &[(i32, bool, bool, bool)],
        snapshot: &mut TypoSnapshot,
        document: Option<*mut dyn Document>,
        input: &RowFlowInput,
    ) {
        for ((geom, _), (pointer, _, _, _)) in laid.iter().zip(cell_ptrs.iter()) {
            let mut cell = TypoTableCell::default();
            cell.assignment(*pointer, snapshot as *mut TypoSnapshot);
            cell.setDocumentType(input.doc_type);
            cell.setLeft(geom.left);
            cell.setRight(geom.right);
            cell.setMargin(0, geom.margin_left, 0, geom.margin_right);
            cell.setHeight(0);
            cell.setSHD(0);
            cell.setBrcTop(0);
            cell.setBrcLeft(0);
            cell.setBrcRight(0);
            cell.setBrcBottom(0);
        }
        if input.revision_delete {
            row.setRevisionDelete(true);
        } else {
            row.setRevisionInsert(true);
        }
        row.setHeight(0);
        if let Some(document) = document.filter(|pointer| !pointer.is_null()) {
            let doc = unsafe { &*document };
            if let Some(revision) = crate::cn::wps::moffice::writer::cache::RevisionCache::run_revision(doc, input.end_cp - 1, 45) {
                let author_id = input.typo_document.filter(|pointer| !pointer.is_null()).map(|pointer| {
                    unsafe { (*pointer).getAuthorId(revision.getAuthor().as_deref().unwrap_or("")) }
                }).unwrap_or(0);
                if let Some(revisions) = input.revisions.filter(|pointer| !pointer.is_null()) {
                    let mut author_of = |name: &str| {
                        input.typo_document.filter(|pointer| !pointer.is_null()).map(|pointer| unsafe { (*pointer).getAuthorId(name) }).unwrap_or(0)
                    };
                    unsafe {
                        (*revisions).addRevisionDeleteTable(doc, input.start_cp, author_id, input.page_pointer, snapshot, &mut author_of);
                    }
                }
            }
        }
        let keep = laid.iter().any(|(geom, _)| geom.vert_merged && !geom.last_merged);
        if !keep {
            if let (Some(list), Some(env)) = (input.cell_states.filter(|pointer| !pointer.is_null()), input.doc_env.filter(|pointer| !pointer.is_null())) {
                let drained = unsafe { (*list).drain() };
                unsafe { (*env).pools.recycleTableCellLayoutStates(drained); }
            }
        }
    }

    // java private void flowVerticalTextFlowCells(IStateEnvoy stateEnvoy)
    fn flow_vertical_cells(
        &mut self,
        row: &mut TypoTableRow,
        laid: &[(CellGeom, Option<usize>)],
        cell_ptrs: &[(i32, bool, bool, bool)],
        cell_layouter: &mut TableCellLayouter,
        snapshot: &mut TypoSnapshot,
        document: *mut dyn Document,
        input: &RowFlowInput,
    ) {
        let cell_bottom = row.getBottom();
        for ((geom, _), (pointer, _, vert_merged, last_merged)) in laid.iter().zip(cell_ptrs.iter()) {
            let vertical = geom.text_flow != 0 && geom.text_flow != 4;
            if *vert_merged {
                if !last_merged {
                    continue;
                }
                let Some(pos) = self.open_merges.iter().rposition(|open| open.left == geom.left) else { continue; };
                if self.open_merges[pos].text_flow == 0 || self.open_merges[pos].text_flow == 4 {
                    continue;
                }
                let open = self.open_merges.remove(pos);
                self.flow_cell_with_bottom(row, cell_layouter, snapshot, document, input, open.pointer, open.start, open.end, open.text_flow, open.width, open.margin_top, open.margin_left, open.margin_bottom, open.margin_right, cell_bottom);
            } else if vertical {
                let width = (geom.right - geom.left).max(0);
                self.flow_cell_with_bottom(row, cell_layouter, snapshot, document, input, *pointer, geom.start, geom.end, geom.text_flow, width, geom.margin_top, geom.margin_left, geom.margin_bottom, geom.margin_right, cell_bottom);
            }
        }
    }

    // java private void flowRestCells(IStateEnvoy stateEnvoy)
    fn flow_rest_cells(
        &mut self,
        row: &mut TypoTableRow,
        laid: &[(CellGeom, Option<usize>)],
        cell_ptrs: &[(i32, bool, bool, bool)],
        cell_layouter: &mut TableCellLayouter,
        snapshot: &mut TypoSnapshot,
        document: *mut dyn Document,
        input: &RowFlowInput,
    ) {
        let cell_bottom = row.getBottom();
        for ((geom, _), (pointer, first_merged, vert_merged, last_merged)) in laid.iter().zip(cell_ptrs.iter()) {
            let vertical = geom.text_flow != 0 && geom.text_flow != 4;
            if *vert_merged {
                if *first_merged {
                    let width = (geom.right - geom.left).max(0);
                    self.flow_cell_with_bottom(row, cell_layouter, snapshot, document, input, *pointer, geom.start, geom.end, geom.text_flow, width, geom.margin_top, geom.margin_left, geom.margin_bottom, geom.margin_right, cell_bottom);
                } else if !last_merged {
                    if let Some(pos) = self.open_merges.iter().rposition(|open| open.left == geom.left) {
                        let open = self.open_merges.remove(pos);
                        self.flow_cell_with_bottom(row, cell_layouter, snapshot, document, input, open.pointer, open.start, open.end, open.text_flow, open.width, open.margin_top, open.margin_left, open.margin_bottom, open.margin_right, cell_bottom);
                    } else {
                        self.adjust_first_vmerge_cell(*pointer, cell_bottom, snapshot);
                    }
                } else if let Some(pos) = self.open_merges.iter().rposition(|open| open.left == geom.left) {
                    if self.open_merges[pos].text_flow != 0 && self.open_merges[pos].text_flow != 4 {
                        let open = self.open_merges.remove(pos);
                        self.flow_cell_with_bottom(row, cell_layouter, snapshot, document, input, open.pointer, open.start, open.end, open.text_flow, open.width, open.margin_top, open.margin_left, open.margin_bottom, open.margin_right, cell_bottom);
                    }
                }
            } else if vertical {
                let width = (geom.right - geom.left).max(0);
                self.flow_cell_with_bottom(row, cell_layouter, snapshot, document, input, *pointer, geom.start, geom.end, geom.text_flow, width, geom.margin_top, geom.margin_left, geom.margin_bottom, geom.margin_right, cell_bottom);
            }
        }
    }

    // java private void adjustFirstVMergeCell(TypoTableCell cell, int cellBottom)
    fn adjust_first_vmerge_cell(&self, pointer: i32, cell_bottom: i32, snapshot: &mut TypoSnapshot) {
        let mut cell = TypoTableCell::default();
        cell.assignment(pointer, snapshot as *mut TypoSnapshot);
        let first_pointer = cell.getFirstVertMergeCell();
        if first_pointer == 0 {
            return;
        }
        let mut first = TypoTableCell::default();
        first.assignment(first_pointer, snapshot as *mut TypoSnapshot);
        if first.isFirstVertMergeCell() {
            let mut rect = crate::cn::wps::moffice::writer::cache::TypoRectCoreImpl::TypoRectCoreImpl::default();
            first.rectForPageCore(&mut rect);
            first.setHeight(cell_bottom.wrapping_sub(rect.getTop()));
        }
    }

    // java private void flowCellWithBottom(TableCellLayoutState curState, int cellBottom, IStateEnvoy stateEnvoy)
    fn flow_cell_with_bottom(
        &mut self,
        row: &mut TypoTableRow,
        cell_layouter: &mut TableCellLayouter,
        snapshot: &mut TypoSnapshot,
        document: *mut dyn Document,
        input: &RowFlowInput,
        pointer: i32,
        start: i32,
        end: i32,
        text_flow: i32,
        width: i32,
        margin_top: i32,
        margin_left: i32,
        margin_bottom: i32,
        margin_right: i32,
        cell_bottom: i32,
    ) {
        let mut cell = TypoTableCell::default();
        cell.assignment(pointer, snapshot as *mut TypoSnapshot);
        cell.setBreakCell(false);
        let mut rect = crate::cn::wps::moffice::writer::cache::TypoRectCoreImpl::TypoRectCoreImpl::default();
        cell.rectForPageCore(&mut rect);
        let cell_height = cell_bottom.wrapping_sub(rect.getTop());
        cell.setHeight(cell_height);
        // java flowCellWithBottom -> flowCell -> TableCellPageLayouter.onStart
        // -> calcMargins: the second flow replaces the temporary BRC-adjusted
        // margin stored on the first vertical-merge cell.
        cell.apply_page_margins(text_flow, margin_top, margin_left, margin_bottom, margin_right);
        let remain = (cell_height - margin_top - margin_bottom).max(0);
        let cp = cell_layouter.flow_text(
            snapshot,
            input.main_state,
            input.page_state,
            pointer,
            start,
            end,
            document,
            input.table_peri_layer,
            input.grid_model,
            input.grid_height,
            input.grid_width,
            input.char_space_for_grid,
            text_flow,
            width,
            margin_top,
            margin_left,
            margin_bottom,
            margin_right,
            remain,
            true,
            input.allow_break_line,
            None,
        );
        if cp < end {
            self.m_row_break = true;
            row.setHasBreakCell(true);
        }
    }

    // java flowCells: maxLayoutHeight minus the cell margins, and allowOverCoreHeight from atLeastOneLine
    fn cell_page_limits(input: &RowFlowInput, vert_merged: bool, margin_top: i32, margin_bottom: i32) -> (i32, bool, bool) {
        let at_least = input.row_height < 0 || (input.at_least_one_line && input.first_line_of_page);
        let allow_over = input.allow_over_core_height || (at_least && input.cant_split && !vert_merged);
        let remain = (input.max_layout_height - input.cur_layout_height - margin_top - margin_bottom).max(0);
        (remain, allow_over, input.allow_break_line)
    }

    // java protected static final int getActualLineWidth(BRC brc)
    fn getActualLineWidth(brc: Option<crate::cn::wps::moffice::writer::data::value::BRC::BRC>) -> i32 {
        let Some(brc) = brc else { return 0 };
        util::moffice::util::UnitUtil::UnitUtil::PT2TWIPS(
            crate::cn::wps::moffice::writer::layout::base::tools::BRCBorderTool::BRCBorderTool::getActualStrokeWidth(
                brc.getBorderType(),
                brc.getLineWidth(),
            ),
        )
    }

    // java protected void initVertMergeCell(TableCellLayoutState cellState, IStateEnvoy stateEnvoy)
    fn init_vert_merge_cell(
        &self,
        cell: &mut TypoTableCell,
        geom: &CellGeom,
        document: &dyn Document,
        paint: Option<(Option<crate::cn::wps::moffice::writer::data::table::tree::CellInfoNode::CellInfoNode>, Option<drawing::cn::wps::moffice::drawing::graphics::SHD::SHD>)>,
    ) {
        // java cellState.breakCell = false; cell.setBreakCell(false);
        cell.setBreakCell(false);
        // java cell.setLeft(cellState.getRgdxaCenterLeft()); cell.setRight(cellState.getRgdxaCenterRight());
        cell.setLeft(geom.left);
        cell.setRight(geom.right);
        cell.setHeight(0);
        // java cell.setSHDAndBRCs(cellInfo)
        if let Some((info, shd)) = paint {
            if let Some(info) = info.as_ref() {
                cell.setSHDAndBRCs(info, shd.as_ref());
            }
        }
        cell.setVertMergeCell(true);
        cell.setLastVertMergeCell(geom.last_merged);
        let cell_end_cp = geom.end - 1;
        cell.setCellEndCP__int_document(cell_end_cp, document);
        cell.setRange(document, cell_end_cp, cell_end_cp + 1);
    }

    // java private void adjustMarginsForBRC(boolean isFirstRow, TypoTableCell cell)
    fn adjust_margins_for_brc(&self, is_first_row: bool, cell: &mut TypoTableCell, snapshot: &TypoSnapshot) {
        let cp = crate::cn::wps::moffice::writer::cache::TypoRangeBase::TypoRangeBase::getStartCP__int_typosnapshot(cell.getPointer(), snapshot);
        if std::env::var_os("WORD_PARITY_TABLE_VALIGN_TRACE").is_some() {
            eprintln!("table_margin before cp={} firstMerge={} bottom={} maxBottom={}", cp, cell.isFirstVertMergeCell(), cell.getMarginBottom(), self.m_max_bottom_brc);
        }
        cell.setBottomBrcPadding(self.m_max_bottom_brc);
        let mut top = cell.getMarginTop();
        let left = cell.getMarginLeft();
        let bottom = cell.getMarginBottom() + self.m_max_bottom_brc;
        let right = cell.getMarginRight();
        if is_first_row {
            top += self.m_max_top_brc;
        }
        cell.setMargin(top, left, bottom, right);
        if std::env::var_os("WORD_PARITY_TABLE_VALIGN_TRACE").is_some() {
            eprintln!("table_margin after cp={} bottom={}", cp, cell.getMarginBottom());
        }
    }

    // java protected void adjustRowHeight — row-height branch before cell content changes the bottom
    fn adjust_row_height(&self, row: &mut TypoTableRow, input: &RowFlowInput) {
        // java row.setHeight(0);
        row.setHeight(0);
        let is_first_row = input.index == 0 || input.first_line_of_page;
        if self.mRowHeight >= 0 {
            // java int addBrcWidthRowHeight = this.mRowHeight + maxBottomBrcWidth; first row also adds maxTopBrcWidth
            let mut add_brc = self.mRowHeight + self.m_max_bottom_brc;
            if is_first_row {
                add_brc += self.m_max_top_brc;
            }
            // java if (viewMode == 0 && !allowOverCoreHeight) row.setHeight(min(maxRowHeight, addBrc))
            if input.view_mode == 0 && !input.allow_over_core_height {
                let max_row_height = input.max_layout_height - input.cur_layout_height;
                row.setHeight(max_row_height.min(add_brc));
            } else {
                row.setHeight(add_brc);
            }
            // java if (!allowOverCoreHeight && !allowBreakLine) row.setBottom(min(maxLayoutHeight, row.getBottom()));
            if !input.allow_over_core_height && !input.allow_break_line {
                let bottom = row.getBottom();
                if bottom > input.max_layout_height {
                    let top = row.getTop();
                    row.setHeight(input.max_layout_height - top);
                }
            }
        } else if self.mRowHeight < 0 {
            // java int rowHeight = -this.mRowHeight; first row also adds maxTopBrcWidth
            let mut row_height = -self.mRowHeight;
            if is_first_row {
                row_height += self.m_max_top_brc;
            }
            row.setHeight(row_height);
        }
    }
}

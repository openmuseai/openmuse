#![allow(dead_code, non_snake_case, unused_variables)]
// Java: import cn.wps.moffice.writer.core.KStyle
use crate::cn::wps::moffice::writer::core::KStyles::KStyles;
// Java: import cn.wps.moffice.writer.layout.base.env.DocumentEnv
use crate::cn::wps::moffice::writer::layout::base::env::DocumentEnv::DocumentEnv;
// Java: import cn.wps.moffice.writer.layout.base.env.SuperEnvoy
use crate::cn::wps::moffice::writer::layout::base::env::SuperEnvoy::StateSnapshot;
// Java: import cn.wps.moffice.writer.layout.base.env.TableCellLayoutState
use crate::cn::wps::moffice::writer::layout::base::env::TableCellLayoutState::TableCellLayoutState;

// java public class TableCellStateEnvoy extends SuperEnvoy<TableCellLayoutState>
pub struct TableCellStateEnvoy {
    // java TableCellLayoutState mLayoutState;
    cell: *mut TableCellLayoutState,
    // java Integer mTableStyleId;
    mTableStyleId: Option<i32>,
}

impl Default for TableCellStateEnvoy {
    fn default() -> Self {
        Self { cell: std::ptr::null_mut(), mTableStyleId: None }
    }
}

impl TableCellStateEnvoy {
    pub fn new() -> Self { Self::default() }

    pub fn bind(cell: *mut TableCellLayoutState) -> Self {
        Self { cell, mTableStyleId: None }
    }

    fn cell(&mut self) -> &mut TableCellLayoutState {
        unsafe { &mut *self.cell }
    }

    // java public void setBreakCell(boolean breakCell)
    pub fn setBreakCell(&mut self, breakCell: bool) {
        // java this.mLayoutState.breakCell = breakCell;
        self.cell().breakCell = breakCell;
    }

    // java public boolean isFirstMerged()
    pub fn isFirstMerged(&mut self) -> bool {
        // java return this.mLayoutState.firstMerged;
        self.cell().firstMerged
    }

    // java public boolean getFixRowHeight()
    pub fn getFixRowHeight(&mut self) -> bool {
        // java return this.mLayoutState.fixRowHeight;
        self.cell().fixRowHeight
    }

    // java public void reset()
    pub fn reset(&mut self) {
        // java this.mTableStyleId = null;
        self.mTableStyleId = None;
    }

    // java public int getTableStyleIdUseToPara()
    pub fn getTableStyleIdUseToPara(&mut self) -> i32 {
        // java if (this.mTableStyleId == null)
        if self.mTableStyleId.is_none() {
            let style_id = self.cell().table_style_id().unwrap_or(4095);
            // java if (tableStyleId != 4095) { KStyle style = document.getStyles().getStyleById(tableStyleId); if (style != null && style.getType() == 3) mTableStyleId = tableStyleId; }
            let mut chosen = 4095;
            if style_id != 4095 {
                let doc = self.cell().super_.document.filter(|p| !p.is_null());
                if let Some(doc) = doc {
                    if let Some(styles) = unsafe { (*doc).getStyles() } {
                        if !styles.is_null() {
                            if let Some(style) = unsafe { (*styles).getStyleById__int(style_id) } {
                                if style.getType() == 3 {
                                    chosen = style_id;
                                }
                            }
                        }
                    }
                }
            }
            // java if (this.mTableStyleId == null) this.mTableStyleId = 4095;
            self.mTableStyleId = Some(chosen);
        }
        // java return this.mTableStyleId != null ? this.mTableStyleId : 4095;
        self.mTableStyleId.unwrap_or(4095)
    }

    // java protected void enumToSave(StateSnapshot stateSnapshot, DocumentEnv docEnv)
    pub fn enumToSave(&mut self, stateSnapshot: &mut StateSnapshot, docEnv: &mut DocumentEnv) {
        // java super.enumToSave(stateSnapshot, docEnv);
        self.cell().super_.getEnvoy().enumToSave(stateSnapshot, docEnv);
        let cell = self.cell();
        // java stateSnapshot.putBool(11, this.mLayoutState.firstMerged);
        stateSnapshot.putBool(11, cell.firstMerged);
        // java stateSnapshot.putBool(12, this.mLayoutState.lastMerged);
        stateSnapshot.putBool(12, cell.lastMerged);
        // java stateSnapshot.putBool(13, this.mLayoutState.vertMerged);
        stateSnapshot.putBool(13, cell.vertMerged);
        // java stateSnapshot.putBool(14, this.mLayoutState.breakCell);
        stateSnapshot.putBool(14, cell.breakCell);
        // java stateSnapshot.putBool(15, this.mLayoutState.cantSplit);
        stateSnapshot.putBool(15, cell.cantSplit);
        // java if (null != this.mLayoutState.firstMergerCellState)
        if cell.firstMergerCellState.is_some() {
            let mut first = StateSnapshot::default();
            let child = cell.firstMergerCellState.as_mut().map(|c| c.as_mut() as *mut TableCellLayoutState).unwrap_or(std::ptr::null_mut());
            if !child.is_null() {
                // java firstMergerCellState.getEnvoy().saveToSnapShot(firstCellState, docEnv);
                TableCellStateEnvoy::bind(child).saveToSnapShot(&mut first, docEnv);
                // java stateSnapshot.setFirstMergeCell(firstCellState);
                stateSnapshot.setFirstMergeCell(first);
            }
        }
    }

    // java public void saveToSnapShot(StateSnapshot stateSnapshot, DocumentEnv docEnv)
    pub fn saveToSnapShot(&mut self, stateSnapshot: &mut StateSnapshot, docEnv: &mut DocumentEnv) {
        // java super.enumToSave then walk this.mLayoutState.cellStateList
        self.enumToSave(stateSnapshot, docEnv);
        stateSnapshot.cells.clear();
        let n = self.cell().super_.cellStateList.len();
        for i in 0..n {
            let child = self.cell().super_.cellStateList.ptr_at(i);
            if child.is_null() {
                continue;
            }
            let mut dest = StateSnapshot::default();
            TableCellStateEnvoy::bind(child).saveToSnapShot(&mut dest, docEnv);
            stateSnapshot.cells.push(dest);
        }
    }

    // java protected boolean enumIntToRestore(int key, int value, DocumentEnv docEnv)
    pub fn enumIntToRestore(&mut self, key: i32, value: i32, docEnv: &mut DocumentEnv) -> bool {
        // java switch (key) case 11..15 set the cell flags, default super
        let cell = self.cell();
        match key {
            11 => cell.firstMerged = value == 1,
            12 => cell.lastMerged = value == 1,
            13 => cell.vertMerged = value == 1,
            14 => cell.breakCell = value == 1,
            15 => cell.cantSplit = value == 1,
            _ => {
                return cell.super_.getEnvoy().enumIntToRestore(key, value, docEnv);
            }
        }
        true
    }

    // java public void restoreFromSnapShot(StateSnapshot stateSnapshot, DocumentEnv docEnv)
    pub fn restoreFromSnapShot(&mut self, stateSnapshot: &mut StateSnapshot, docEnv: &mut DocumentEnv) {
        // java super.restoreFromSnapShot(stateSnapshot, docEnv);
        self.cell().super_.getEnvoy().restoreFromSnapShot(stateSnapshot, docEnv);
        // java case 11..15
        for key in 11..=15 {
            if let Some(value) = stateSnapshot.getInt(key) {
                self.enumIntToRestore(key, value, docEnv);
            }
        }
        // java case 10: firstMergerCellState = pools.obtainTableCellLayoutState(); envoy.restoreFromSnapShot
        if let Some(first) = stateSnapshot.first_merge.clone() {
            let mut child = Box::new(TableCellLayoutState::default());
            let mut first = first;
            TableCellStateEnvoy::bind(child.as_mut()).restoreFromSnapShot(first.as_mut(), docEnv);
            self.cell().firstMergerCellState = Some(child);
        }
        // java if (cellInfo != null) copy margins and Tools.getTextFlow
        self.cell().apply_cell_margins();
    }
}

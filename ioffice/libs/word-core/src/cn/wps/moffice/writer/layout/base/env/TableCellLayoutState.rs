#![allow(dead_code, non_snake_case, unused_variables)]
// Java: import cn.wps.moffice.writer.data.table.CellInfo
use crate::cn::wps::moffice::writer::data::table::tree::CellInfoNode::CellInfoNode;
// Java: import cn.wps.moffice.writer.data.table.RowInfo
use crate::cn::wps::moffice::writer::data::table::tree::RowInfoNode::RowInfoNode;
// Java: import cn.wps.moffice.writer.data.table.TableInfo
use crate::cn::wps::moffice::writer::data::table::TableInfos::TableInfos;
use crate::cn::wps::moffice::writer::data::table::tree::TableInfoNode::TableInfoNode;
// Java: import cn.wps.moffice.writer.data.value.BRC
use crate::cn::wps::moffice::writer::data::value::BRC::BRC;
// Java: import cn.wps.moffice.writer.layout.base.env.LayoutState
use crate::cn::wps::moffice::writer::layout::base::env::LayoutState::LayoutState;
// Java: import cn.wps.moffice.writer.core.Document
use crate::cn::wps::moffice::writer::core::Document::Document;

// java public class TableCellLayoutState extends LayoutState
#[derive(Default)]
pub struct TableCellLayoutState {
    // java LayoutState super_
    pub super_: LayoutState,
    // java public boolean firstMerged;
    pub firstMerged: bool,
    // java public boolean lastMerged;
    pub lastMerged: bool,
    // java public boolean vertMerged;
    pub vertMerged: bool,
    // java public boolean breakCell;
    pub breakCell: bool,
    // java public boolean cantSplit;
    pub cantSplit: bool,
    // java public boolean fixRowHeight;
    pub fixRowHeight: bool,
    // java public boolean atLeastOneLine;
    pub atLeastOneLine: bool,
    // java public TableCellLayoutState firstMergerCellState;
    pub firstMergerCellState: Option<Box<TableCellLayoutState>>,
    // java private CellInfo cellInfo;
    cellInfo: Option<CellInfoNode>,
}

// java public static class Alloc extends LayoutState.Alloc
#[derive(Default)]
pub struct Alloc;

impl TableCellLayoutState {
    // java public TableCellLayoutState(TableCellStateEnvoy stateEnvoy)
    pub fn TableCellLayoutState() -> Self {
        // java super(stateEnvoy != null ? stateEnvoy : new TableCellStateEnvoy());
        Self {
            super_: LayoutState::LayoutState(None),
            ..Self::default()
        }
    }

    // java public void setCellnfo(CellInfo cellnfo)
    pub fn setCellnfo(&mut self, cellnfo: CellInfoNode) {
      // java this.cellInfo = cellnfo;
      self.cellInfo = Some(cellnfo);
    }

    // java public CellInfo getCellInfo()
    pub fn getCellInfo(&self) -> Option<&CellInfoNode> {
      self.cellInfo.as_ref()
    }

    // java public static SHD getMergedCellSHD(CellInfo cellInfo)
    pub fn getMergedCellSHD(&self) -> Option<drawing::cn::wps::moffice::drawing::graphics::SHD::SHD> {
        let info = self.getCellInfo()?;
        if info.getVertMerge() != 1 {
            return info.getSHD().cloned();
        }
        // Cell env stores the text level (table level + 1). The owning table is one level up.
        let level = self.super_.tablePeriLayer.saturating_sub(1).max(1);
        let doc = self.super_.document.filter(|p| !p.is_null())?;
        let infos = unsafe { (*doc).getTableInfos() }?;
        if infos.is_null() {
            return info.getSHD().cloned();
        }
        let table = unsafe { (&*infos).getTableInfoByLevel(self.super_.cp, level) }?;
        let row_index = table.rows().iter().position(|row| row.contains_cp(self.super_.cp))?;
        let cell_index = table.rows().get(row_index)?.indexOfCP(self.super_.cp)? as usize;
        let (first_row, first_cell) = table.first_vert_merge_cell(row_index, cell_index)?;
        table.rows().get(first_row)?.getCellByIndex(first_cell as i32)?.getSHD().cloned()
    }

    // java public int getRgdxaCenterLeft()
    pub fn getRgdxaCenterLeft(&self) -> i32 {
        // java return this.cellInfo.getRgdxaLeft();
        self.with_cell(|row, cell, index| {
            let _ = cell;
            row.getRgdxa(index)
        }).unwrap_or(0)
    }

    // java public int getRgdxaCenterRight()
    pub fn getRgdxaCenterRight(&self) -> i32 {
        // java return getMergedCellRgdxaRight(this.cellInfo);
        self.with_cell(|row, cell, index| {
            Self::merged_rgdxa_right(row, cell, index)
        }).unwrap_or(0)
    }

    // java public BRC getBrcLeft()
    pub fn getBrcLeft(&self) -> Option<BRC> {
        // java return this.cellInfo.getBrcLeft();
        self.cellInfo.as_ref().and_then(|cell| cell.getBrcLeft().cloned())
    }

    // java public BRC getBrcTop()
    pub fn getBrcTop(&self) -> Option<BRC> {
        // java return this.cellInfo.getBrcTop();
        self.cellInfo.as_ref().and_then(|cell| cell.getBrcTop().cloned())
    }

    // java public BRC getBrcRight()
    pub fn getBrcRight(&self) -> Option<BRC> {
        // java return this.cellInfo.getBrcRight();
        self.cellInfo.as_ref().and_then(|cell| cell.getBrcRight().cloned())
    }

    // java public BRC getBrcBottom()
    pub fn getBrcBottom(&self) -> Option<BRC> {
        // java return this.cellInfo.getBrcBottom();
        self.cellInfo.as_ref().and_then(|cell| cell.getBrcBottom().cloned())
    }

    // java public static int getMergedCellRgdxaRight(CellInfo cellInfo)
    fn merged_rgdxa_right(row: &RowInfoNode, cell: &CellInfoNode, index: i32) -> i32 {
        let _ = cell;
        // java int right = cellInfo.getRgdxaRight();
        let mut right = row.getRgdxa(index + 1);
        // java for (int i = cellInfo.getIndex() + 1; i < rowInfo.size(); ++i)
        let mut i = index + 1;
        let size = row.size();
        while i < size {
            let Some(next) = row.getCellByIndex(i) else { break; };
            // java if (2 != nextCell.getHoriMerge()) break;
            if next.getHoriMerge() != 2 {
                break;
            }
            right = row.getRgdxa(i + 1);
            i += 1;
        }
        right
    }

    pub(crate) fn with_cell<T>(&self, f: impl FnOnce(&RowInfoNode, &CellInfoNode, i32) -> T) -> Option<T> {
        let mut table = self.table()?;
        let index = {
            let row = table.getRowByCP(self.super_.cp)?;
            row.indexOfCP(self.super_.cp)?
        };
        let row = table.getRowByCP(self.super_.cp)?;
        let cell = row.getCellByIndex(index)?;
        Some(f(row, cell, index))
    }

    fn table(&self) -> Option<TableInfoNode> {
        let doc = self.super_.document.filter(|p| !p.is_null())?;
        let infos = unsafe { (*doc).getTableInfos() }?;
        if infos.is_null() {
            return None;
        }
        unsafe { (&*infos).getTableInfoByLevel(self.super_.cp, self.super_.tablePeriLayer) }
    }

    pub(crate) fn apply_cell_margins(&mut self) {
        // java pageEnv.margin* = cellInfo.getMargin*; pageEnv.textFlow = Tools.getTextFlow(...)
        let paint = if let Some(cell) = &self.cellInfo {
            Some(CellPaint {
                top: cell.margin_top(),
                left: cell.margin_left(),
                right: cell.margin_right(),
                bottom: cell.margin_bottom(),
                text_flow: text_flow_for_cell(cell.getTextFlow()),
                style_id: 0,
            })
        } else {
            self.with_cell(|_r, cell, _i| CellPaint {
                top: cell.margin_top(),
                left: cell.margin_left(),
                right: cell.margin_right(),
                bottom: cell.margin_bottom(),
                text_flow: text_flow_for_cell(cell.getTextFlow()),
                style_id: 0,
            })
        };
        if let Some(paint) = paint {
            self.super_.pageEnv.marginTop = paint.top;
            self.super_.pageEnv.marginLeft = paint.left;
            self.super_.pageEnv.marginRight = paint.right;
            self.super_.pageEnv.marginBottom = paint.bottom;
            self.super_.pageEnv.textFlow = paint.text_flow;
        }
    }

    pub(crate) fn table_style_id(&self) -> Option<i32> {
        // java cellInfo.getParent().getParent().getStyleId()
        let mut table = self.table()?;
        Some(table.getStyleId())
    }
}

pub(crate) struct CellPaint {
    pub top: i32,
    pub left: i32,
    pub right: i32,
    pub bottom: i32,
    pub text_flow: i32,
    pub style_id: i32,
}

// java Tools.getTextFlow: case 1 -> 5, case 5 -> 1
pub fn text_flow_for_cell(text_flow: i32) -> i32 {
    match text_flow {
        1 => 5,
        5 => 1,
        other => other,
    }
}
